//! The borders of a claim, shown as blocks that only one player sees (`show_block`). The world does not change.
//! The client gets the real block back when the block or its chunk is sent again, so the mod shows the borders again
//! every 5 seconds, and sends the real blocks after 60 seconds or before the next borders.

use super::with;
use crate::tool::border;
use mod_api::{BlockState, Host, LevelRef, ServerHost};

/// How long the borders show: 60 seconds.
const SHOW_TICKS: u64 = 20 * 60;
/// How often they are shown again: 5 seconds.
const AGAIN_TICKS: u64 = 20 * 5;
/// In the first second they are shown again more often: this many ticks after they first show. The game sends the
/// clicked block back to the client after a refused click (the client may have made a path of grass with the
/// shovel), and that block takes the place of the border there.
const FIRST_AGAIN: &[u64] = &[1, 2, 4, 8, 16];
/// How far up or down from the player's height the top block of a column is looked for.
const REACH: i32 = 24;

/// The borders a player sees: each block, and the state shown there.
#[derive(Clone, Debug, Default)]
pub struct Shown {
    blocks: Vec<([i32; 3], BlockState)>,
    since: u64,
    until: u64,
    again: u64,
}

/// How the border looks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Look {
    /// A claim: glowstone corners, gold sides.
    Claim,
    /// A claim that cannot be made, or another player's claim in the way: redstone ore corners, netherrack sides.
    Refused,
}

fn air(host: &dyn Host, state: BlockState) -> bool {
    host.block_state_string(state).is_some_and(|text| {
        matches!(
            text.split('[').next(),
            Some("minecraft:air" | "minecraft:cave_air" | "minecraft:void_air")
        )
    })
}

/// The y of the top block of column `column` near the height `near`: the highest block that is not air, with air
/// above it.
fn top(host: &dyn Host, level: LevelRef, column: [i32; 2], near: i32) -> i32 {
    let at = |y: i32| level.block_state(host, [column[0], y, column[1]]);
    match at(near) {
        Some(state) if air(host, state) => (near - REACH..near)
            .rev()
            .find(|y| at(*y).is_some_and(|s| !air(host, s)))
            .unwrap_or(near),
        Some(_) => (near + 1..=near + REACH)
            .find(|y| at(*y).is_none_or(|s| air(host, s)))
            .map_or(near, |y| y - 1),
        None => near,
    }
}

/// How high the pillar over a corner goes, in blocks over the top block.
const PILLAR: i32 = 64;

/// The blocks of the pillar of a corner: `base` in the place of the top block `[x, y, z]`, then end rods over it.
/// The rods take only the place of air, so the player sees no block of the world go away.
fn pillar(
    host: &dyn Host,
    level: LevelRef,
    top: [i32; 3],
    base: BlockState,
    rod: BlockState,
) -> Vec<([i32; 3], BlockState)> {
    let highest = level
        .height(host)
        .map_or(top[1] + PILLAR, |(min, height)| min + height - 1);
    let rods = (top[1] + 1..=(top[1] + PILLAR).min(highest))
        .map(|y| [top[0], y, top[2]])
        .filter(|pos| level.block_state(host, *pos).is_some_and(|s| air(host, s)))
        .map(|pos| (pos, rod));
    std::iter::once((top, base)).chain(rods).collect()
}

/// Shows `player` the border of the rectangle `min`..`max`, near the height `near`: a pillar over each corner, and
/// a block every 10 blocks along the sides. The borders they saw before go away first.
pub fn show(host: &mut dyn ServerHost, player: u128, min: [i32; 2], max: [i32; 2], near: i32, look: Look) {
    hide(host, player);
    let Some(level) = host.player(player).and_then(|p| p.level(host)) else {
        return;
    };
    let (base, side) = match look {
        Look::Claim => ("minecraft:glowstone", "minecraft:gold_block"),
        Look::Refused => ("minecraft:redstone_block", "minecraft:netherrack"),
    };
    let ids = [base, side, "minecraft:end_rod"].map(|id| host.block_state_id(id));
    let [Some(base), Some(side), Some(rod)] = ids else {
        return;
    };
    let mut blocks = Vec::new();
    for mark in border(min, max) {
        let y = top(host, level, mark.column, near);
        let pos = [mark.column[0], y, mark.column[1]];
        if mark.corner {
            blocks.extend(pillar(host, level, pos, base, rod));
        } else {
            blocks.push((pos, side));
        }
    }
    remember(host, player, blocks);
}

/// Sends `player` the border blocks and keeps them, to send them again and to take them away.
fn remember(host: &mut dyn ServerHost, player: u128, blocks: Vec<([i32; 3], BlockState)>) {
    send(host, player, &blocks);
    with(|state| {
        let shown = Shown {
            blocks,
            since: state.now,
            until: state.now + SHOW_TICKS,
            again: state.now + FIRST_AGAIN[0],
        };
        state.borders.insert(player, shown);
    });
}

/// Sends `player` the border blocks. A refusal is written to the server's log, once for each call.
fn send(host: &mut dyn ServerHost, player: u128, blocks: &[([i32; 3], BlockState)]) {
    let Some(handle) = host.player(player) else {
        return;
    };
    let mut refusal = None;
    for (pos, state) in blocks {
        if let Err(why) = handle.show_block(host, *pos, *state) {
            refusal.get_or_insert(format!("show_block at {pos:?}: {why:?}"));
        }
    }
    if let Some(line) = refusal {
        super::log(host, mod_api::ConsoleLevel::Warn, &line);
    }
}

/// Shows `player` the pillar over the block at `pos`: the first corner of a new claim.
pub fn mark(host: &mut dyn ServerHost, player: u128, pos: [i32; 3]) {
    hide(host, player);
    let Some(level) = host.player(player).and_then(|p| p.level(host)) else {
        return;
    };
    let ids = ["minecraft:glowstone", "minecraft:end_rod"].map(|id| host.block_state_id(id));
    let [Some(base), Some(rod)] = ids else {
        return;
    };
    let blocks = pillar(host, level, pos, base, rod);
    remember(host, player, blocks);
}

/// Sends `player` the real blocks where their borders were.
pub fn hide(host: &mut dyn ServerHost, player: u128) {
    let Some(shown) = with(|state| state.borders.remove(&player)) else {
        return;
    };
    let Some(handle) = host.player(player) else {
        return;
    };
    let Some(level) = handle.level(host) else {
        return;
    };
    for (pos, _) in shown.blocks {
        if let Some(real) = level.block_state(host, pos) {
            let _ = handle.show_block(host, pos, real);
        }
    }
}

/// Each tick: shows the borders again every 5 seconds, and hides them after 60 seconds.
pub fn tick(host: &mut dyn ServerHost) {
    let (again, over) = with(|state| {
        let now = state.now;
        let mut again = Vec::new();
        let mut over = Vec::new();
        for (player, shown) in state.borders.iter_mut() {
            if now >= shown.until {
                over.push(*player);
            } else if now >= shown.again {
                let age = now - shown.since;
                shown.again = match FIRST_AGAIN.iter().find(|after| **after > age) {
                    Some(after) => shown.since + after,
                    None => now + AGAIN_TICKS,
                };
                again.push((*player, shown.blocks.clone()));
            }
        }
        (again, over)
    });
    for player in over {
        hide(host, player);
    }
    for (player, blocks) in again {
        send(host, player, &blocks);
    }
}
