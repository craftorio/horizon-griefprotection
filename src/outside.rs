//! Protection from outside a claim: a fluid, a fire or a piston may change a block in a claim only when it comes
//! from the same owner's land. "Outside" is land in no claim, or in a claim of another owner.

use crate::claims::index::Claims;

/// Whether a change that comes from block column `from` may change block column `to` of `dimension`.
pub fn may_cross(claims: &Claims, dimension: &str, from: [i32; 2], to: [i32; 2]) -> bool {
    let Some(target) = claims.at(dimension, to[0], to[1]) else {
        return true;
    };
    claims
        .at(dimension, from[0], from[1])
        .is_some_and(|source| source.owner == target.owner)
}

/// A piston's move: where the piston is, the direction of its move, and the blocks it moves and breaks.
#[derive(Clone, Copy, Debug)]
pub struct PistonMove<'a> {
    pub piston: [i32; 3],
    /// The direction of the move (`[0, 1, 0]` is up).
    pub direction: [i32; 3],
    pub moved: &'a [[i32; 3]],
    pub destroyed: &'a [[i32; 3]],
}

/// Whether a piston's move changes only land of the piston's own owner, or unclaimed land. The blocks next to the
/// piston and next to each moved block are checked on both sides along the direction: so the check is correct when
/// the piston pushes and when it pulls.
pub fn piston_may_move(claims: &Claims, dimension: &str, piston: PistonMove<'_>) -> bool {
    let from = [piston.piston[0], piston.piston[2]];
    let d = piston.direction;
    let around = |pos: [i32; 3]| {
        [
            [pos[0], pos[2]],
            [pos[0] + d[0], pos[2] + d[2]],
            [pos[0] - d[0], pos[2] - d[2]],
        ]
    };
    let moved = piston.moved.iter().chain(std::iter::once(&piston.piston));
    let columns = moved
        .flat_map(|pos| around(*pos))
        .chain(piston.destroyed.iter().map(|pos| [pos[0], pos[2]]));
    columns.into_iter().all(|to| may_cross(claims, dimension, from, to))
}

/// Whether a dispenser at `dispenser` that faces `facing` may put out an item (lava, water, fire, a block...) into
/// the block in front of it.
pub fn dispenser_may_reach(claims: &Claims, dimension: &str, dispenser: [i32; 3], facing: [i32; 3]) -> bool {
    let to = [dispenser[0] + facing[0], dispenser[2] + facing[2]];
    may_cross(claims, dimension, [dispenser[0], dispenser[2]], to)
}

/// The mobs that may not change blocks in a claim: they break or take blocks (an enderman, a ravager, a silverfish,
/// a wither, a zombie at a door), or set fire (a blaze, a ghast). Other mobs (a sheep that eats grass, a villager
/// that farms, a fox, an allay) change the world as in vanilla. Explosions never break blocks in a claim.
pub const GRIEFERS: &[&str] = &[
    "minecraft:blaze",
    "minecraft:creeper",
    "minecraft:drowned",
    "minecraft:ender_dragon",
    "minecraft:enderman",
    "minecraft:evoker",
    "minecraft:ghast",
    "minecraft:husk",
    "minecraft:ravager",
    "minecraft:silverfish",
    "minecraft:wither",
    "minecraft:zombie",
    "minecraft:zombie_villager",
];

/// Whether the block at `pos` stays in an explosion: the blocks in a claim never break in one.
pub fn spared(claims: &Claims, dimension: &str, pos: [i32; 3]) -> bool {
    claims.at(dimension, pos[0], pos[2]).is_some()
}
