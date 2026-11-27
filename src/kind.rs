//! What a right click on a block is, in a claim: the use of a door or a button (access), or of a block that
//! holds something (container). Other clicks either do nothing to the block, or place a block (`entity_place`).

use crate::claims::rules::Action;

/// Blocks a click uses without holding anything: by the end of their id.
const ACCESS: &[&str] = &[
    "_door",
    "_trapdoor",
    "_fence_gate",
    "_button",
    "_pressure_plate",
    "_bed",
];
const ACCESS_IDS: &[&str] = &[
    "minecraft:lever",
    "minecraft:bell",
    "minecraft:note_block",
    "minecraft:jukebox",
    // a click changes a redstone circuit: the delay of a repeater, the shape of redstone wire
    "minecraft:repeater",
    "minecraft:redstone_wire",
];
/// Blocks with no block entity that still hold or give something: what a click on them takes or puts is the
/// owner's.
const CONTAINER_IDS: &[&str] = &["minecraft:composter", "minecraft:flower_pot", "minecraft:cake"];

/// The action a right click on block `id` is; `has_contents`: it has a block entity.
pub fn click(id: &str, has_contents: bool) -> Option<Action> {
    if ACCESS.iter().any(|end| id.ends_with(end)) || ACCESS_IDS.contains(&id) {
        return Some(Action::Access);
    }
    if CONTAINER_IDS.contains(&id) || id.starts_with("minecraft:potted_") || id.ends_with("candle_cake") {
        return Some(Action::Container);
    }
    // a sign, a lectern or a chest: what it holds can be read or taken
    has_contents.then_some(Action::Container)
}
