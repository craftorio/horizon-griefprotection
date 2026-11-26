//! Who may do what in a claim.

use super::{Claim, Trust};

/// What a player does to a block or an entity in a claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    /// Use a door, a trapdoor, a gate, a button, a lever, a pressure plate or a bed.
    Access,
    /// Open a block that holds something; hurt or use an animal, an item frame, an armor stand.
    Container,
    /// Break or place a block.
    Build,
    /// Trust or untrust a player, change the claim.
    Manage,
}

impl Action {
    /// The trust the action needs.
    pub fn needs(self) -> Trust {
        match self {
            Action::Access => Trust::Access,
            Action::Container => Trust::Container,
            Action::Build => Trust::Build,
            Action::Manage => Trust::Manager,
        }
    }
}

/// Why an action is refused: the claim's owner, for the message to the player.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Refused {
    pub owner: u128,
}

/// Whether `player` may do `action` in `claim`. An admin (the permission `griefprotection.admin`, or an
/// operator) may do everything everywhere.
pub fn allowed(claim: &Claim, player: u128, action: Action, admin: bool) -> Result<(), Refused> {
    if admin || claim.trust_of(player).is_some_and(|trust| trust >= action.needs()) {
        Ok(())
    } else {
        Err(Refused { owner: claim.owner })
    }
}
