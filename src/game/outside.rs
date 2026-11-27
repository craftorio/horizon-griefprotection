//! The hooks of what comes into a claim from outside: fluids, fire, pistons, explosions and mobs. The rules are in
//! `crate::outside`.

use super::with_loaded;
use crate::claims::rules::{Action, allowed};
use crate::outside::{GRIEFERS, PistonMove, dispenser_may_reach, may_cross, piston_may_move, spared};
use mod_api::{EntityRef, Flow, Host};

fn cancel(refuse: bool, cancelled: &mut bool) -> Flow {
    if refuse {
        *cancelled = true;
        Flow::Cancel
    } else {
        Flow::Continue
    }
}

/// `block_spread`: a fluid flows, or a fire spreads, from `from` into `to`.
pub fn spread(dimension: &str, from: [i32; 3], to: [i32; 3], cancelled: &mut bool) -> Flow {
    let refuse = with_loaded(|claims| !may_cross(claims, dimension, [from[0], from[2]], [to[0], to[2]]));
    cancel(refuse.unwrap_or(false), cancelled)
}

/// `block_burn`: a fire burns the block at `pos` away. A block of a claim does not burn away: the fire came from
/// outside, or the owner can put it out.
pub fn burn(dimension: &str, pos: [i32; 3], cancelled: &mut bool) -> Flow {
    let refuse = with_loaded(|claims| claims.at(dimension, pos[0], pos[2]).is_some());
    cancel(refuse.unwrap_or(false), cancelled)
}

/// `block_ignite`: a fire starts at `pos`. Without a player (lava, lightning, a dispenser), not in a claim. A player
/// needs build trust. The hook has no server, so an admin is not known here: an admin needs build trust too.
pub fn ignite(dimension: &str, pos: [i32; 3], player: Option<u128>, cancelled: &mut bool) -> Flow {
    let refuse = with_loaded(|claims| {
        claims.at(dimension, pos[0], pos[2]).is_some_and(|claim| match player {
            Some(player) => allowed(claim, player, Action::Build, false).is_err(),
            None => true,
        })
    });
    cancel(refuse.unwrap_or(false), cancelled)
}

/// `piston_pre`: a piston moves blocks.
pub fn piston(dimension: &str, piston: PistonMove<'_>, cancelled: &mut bool) -> Flow {
    let refuse = with_loaded(|claims| !piston_may_move(claims, dimension, piston));
    cancel(refuse.unwrap_or(false), cancelled)
}

/// `dispenser_acts`: whether a dispenser at `pos` that faces `facing` must not put out its item, because the block in
/// front of it is in a claim of another owner.
pub fn dispense(dimension: &str, pos: [i32; 3], facing: [i32; 3]) -> bool {
    with_loaded(|claims| !dispenser_may_reach(claims, dimension, pos, facing)).unwrap_or(false)
}

/// `explosion_detonate`: the blocks in claims stay. The entities still take damage.
pub fn explosion(dimension: &str, blocks: &mut Vec<[i32; 3]>) {
    with_loaded(|claims| blocks.retain(|pos| !spared(claims, dimension, *pos)));
}

/// `entity_mob_griefing`: the mobs of `GRIEFERS` do not change blocks in a claim.
pub fn mob_griefing(host: &dyn Host, entity: EntityRef, can_grief: &mut bool) {
    if !*can_grief {
        return;
    }
    let griefer = entity
        .kind(host)
        .map(|kind| kind.id(host))
        .is_some_and(|id| GRIEFERS.contains(&id.as_str()));
    if !griefer {
        return;
    }
    let (Some(level), Some(pos)) = (entity.level(host), entity.block_position(host)) else {
        return;
    };
    let dimension = level.dimension_id();
    if with_loaded(|claims| claims.at(dimension, pos[0], pos[2]).is_some()) == Some(true) {
        *can_grief = false;
    }
}

/// `farmland_trample`: nobody tramples farmland in a claim. The hook does not say the dimension or who lands, so
/// a claim at this column in any dimension keeps the farmland.
pub fn trample(pos: [i32; 3], cancelled: &mut bool) -> Flow {
    let refuse = with_loaded(|claims| claims.in_any_dimension(pos[0], pos[2]));
    cancel(refuse.unwrap_or(false), cancelled)
}
