//! What players do in a claim: build, open containers, use doors, hurt animals, use buckets, light fires, trample
//! farmland, use item frames and armor stands. A player without the trust for it is refused, and told so on their
//! action bar.

use super::{action_bar, admin, name_of, with};
use crate::claims::rules::{Action, allowed};
use crate::lang::Key;
use mod_api::{Flow, HostBlockStates, ServerHost};

/// Whether `player` may not do `action` at `pos` in the dimension they are in; tells them so.
pub fn refused(host: &mut dyn ServerHost, player: u128, pos: [i32; 3], action: Action) -> bool {
    let Some(dimension) = super::dimension_of(host, player) else {
        return false;
    };
    refused_in(host, player, &dimension, pos, action)
}

/// Whether `player` may not do `action` at `pos` of `dimension`; tells them so.
pub fn refused_in(host: &mut dyn ServerHost, player: u128, dimension: &str, pos: [i32; 3], action: Action) -> bool {
    let is_admin = admin(host, player);
    let owner = with(|state| {
        let claim = state.claims(host).at(dimension, pos[0], pos[2])?;
        allowed(claim, player, action, is_admin)
            .err()
            .map(|refused| refused.owner)
    });
    let Some(owner) = owner else {
        return false;
    };
    let name = name_of(host, owner);
    action_bar(host, player, Key::NotTrusted, &[("owner", &name)]);
    true
}

pub fn cancel(refused: bool, cancelled: &mut bool) -> Flow {
    if !refused {
        return Flow::Continue;
    }
    *cancelled = true;
    Flow::Cancel
}

fn acting(host: &mut dyn ServerHost) -> Option<u128> {
    host.players().acting_player().map(|p| p.uuid())
}

/// `break_block`: the acting player breaks the block at `pos`.
pub fn refuse_build(host: &mut dyn ServerHost, pos: [i32; 3], cancelled: &mut bool) -> Flow {
    let Some(player) = acting(host) else {
        return Flow::Continue;
    };
    refuse_build_by(host, player, pos, cancelled)
}

pub fn refuse_build_by(host: &mut dyn ServerHost, player: u128, pos: [i32; 3], cancelled: &mut bool) -> Flow {
    cancel(refused(host, player, pos, Action::Build), cancelled)
}

/// `right_click_block`: whether the click of `player` on the block at `pos` uses something they may not use.
pub fn refuse_click(host: &mut dyn ServerHost, player: u128, pos: [i32; 3]) -> bool {
    let Some(level) = host.player(player).and_then(|p| p.level(host)) else {
        return false;
    };
    let Some(state) = level.block_state(host, pos) else {
        return false;
    };
    let Some(text) = host.block_state_string(state) else {
        return false;
    };
    let id = text.split('[').next().unwrap_or(&text).to_owned();
    let has_contents = host.block_states().has_block_entity(state) == Some(true);
    match crate::kind::click(&id, has_contents) {
        Some(action) => refused(host, player, pos, action),
        None => false,
    }
}

/// `attack_entity`: the acting player hits an entity of `kind` at `pos`. Monsters and players may be hit anywhere.
pub fn refuse_hit(host: &mut dyn ServerHost, kind: &str, pos: [f64; 3], cancelled: &mut bool) -> Flow {
    if kind == "minecraft:player" || host.entity_type_is_enemy(kind) {
        return Flow::Continue;
    }
    let Some(player) = acting(host) else {
        return Flow::Continue;
    };
    let block = pos.map(|c| c.floor() as i32);
    cancel(refused(host, player, block, Action::Container), cancelled)
}

/// `entity_interact`, `entity_interact_specific`: the acting player uses the entity `entity` (an animal, an item
/// frame, an armor stand, a villager...). Monsters and players may be used anywhere.
pub fn refuse_use(host: &mut dyn ServerHost, player: u128, entity: i32, kind: &str) -> bool {
    if kind == "minecraft:player" || host.entity_type_is_enemy(kind) {
        return false;
    }
    let Some(pos) = host.entity(entity as u32).and_then(|e| e.block_position(host)) else {
        return false;
    };
    refused(host, player, pos, Action::Container)
}

/// `bucket_empty`, `fill_bucket`, `block_ignite` by a player, `farmland_trample` by a player: build trust.
pub fn refuse_acting_build(host: &mut dyn ServerHost, pos: [i32; 3], cancelled: &mut bool) -> Flow {
    refuse_build(host, pos, cancelled)
}
