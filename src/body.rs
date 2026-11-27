//! The bodies of Knockout (`craftorio/horizon-knockout`): who may open one, without the game.
//!
//! Knockout does not know claims. It marks each body entity with tags, and this module reads them. A body is a
//! `minecraft:mannequin` with the tags `knockout_body`, `knockout_body_<id>` and `knockout_owner_<uuid>`. After an
//! hour it becomes a skeleton. The owner's decision, 2026-11-27: in a claim, a body opens only for its owner and for
//! the players with container trust; a skeleton body opens for every player.

/// The tag of every body entity.
pub const BODY_TAG: &str = "knockout_body";
/// The start of the tag with the owner's UUID: 32 hex digits follow.
pub const OWNER_TAG: &str = "knockout_owner_";
/// The tag of a skeleton body. Knockout does not set it yet, so a body of the type [`SKELETON_KIND`] is a skeleton
/// too.
pub const SKELETON_TAG: &str = "knockout_skeleton";
/// The entity type of a skeleton body.
pub const SKELETON_KIND: &str = "minecraft:skeleton";

/// What a right click on an entity needs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Open {
    /// The entity is not a body: the rule for other entities holds.
    NotBody,
    /// Every player may open it: the owner of the body, or a skeleton body.
    Anyone,
    /// A body of another player: in a claim, the player needs container trust.
    NeedsTrust,
}

/// What a click of `player` on an entity of the type `kind` with `tags` needs.
pub fn open(kind: &str, tags: &[String], player: u128) -> Open {
    let has = |tag: &str| tags.iter().any(|t| t == tag);
    if has(SKELETON_TAG) {
        return Open::Anyone;
    }
    if !has(BODY_TAG) {
        return Open::NotBody;
    }
    if kind == SKELETON_KIND || owner(tags) == Some(player) {
        return Open::Anyone;
    }
    Open::NeedsTrust
}

/// The owner's UUID from the tag `knockout_owner_<uuid>`.
pub fn owner(tags: &[String]) -> Option<u128> {
    tags.iter()
        .filter_map(|tag| tag.strip_prefix(OWNER_TAG))
        .find_map(|hex| u128::from_str_radix(hex, 16).ok())
}
