//! Claims: land a player owns, as GriefPrevention has it. A claim is a rectangle of columns in one dimension,
//! from the bottom of the world to the top. Its owner trusts other players by name, at a level each.

pub mod blocks;
pub mod index;
pub mod rules;
#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// How much a player is trusted in a claim. Each level allows what the levels below allow.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Trust {
    /// Doors, trapdoors, gates, buttons, levers, pressure plates, beds (GriefPrevention's `/accesstrust`).
    Access,
    /// Also chests and every block that holds something, animals, item frames (`/containertrust`).
    Container,
    /// Also breaking and placing blocks (`/trust`).
    Build,
    /// Also trusting and untrusting other players (`/permissiontrust`).
    Manager,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Claim {
    pub id: u64,
    /// The owner's UUID.
    pub owner: u128,
    /// `minecraft:overworld`.
    pub dimension: String,
    /// The lowest x and z of the claim, inclusive.
    pub min: [i32; 2],
    /// The highest x and z of the claim, inclusive.
    pub max: [i32; 2],
    /// The trusted players and their levels.
    #[serde(default)]
    pub trusted: BTreeMap<u128, Trust>,
    /// When it was made, in seconds since 1970.
    #[serde(default)]
    pub created: i64,
}

impl Claim {
    pub fn contains(&self, dimension: &str, x: i32, z: i32) -> bool {
        self.dimension == dimension
            && (self.min[0]..=self.max[0]).contains(&x)
            && (self.min[1]..=self.max[1]).contains(&z)
    }

    /// The number of columns: what it costs in claim blocks.
    pub fn area(&self) -> i64 {
        i64::from(self.max[0] - self.min[0] + 1) * i64::from(self.max[1] - self.min[1] + 1)
    }

    pub fn width(&self) -> i32 {
        (self.max[0] - self.min[0] + 1).min(self.max[1] - self.min[1] + 1)
    }

    pub fn overlaps(&self, other: &Claim) -> bool {
        self.dimension == other.dimension
            && self.min[0] <= other.max[0]
            && other.min[0] <= self.max[0]
            && self.min[1] <= other.max[1]
            && other.min[1] <= self.max[1]
    }

    /// The level `player` has here: the owner manages everything.
    pub fn trust_of(&self, player: u128) -> Option<Trust> {
        if player == self.owner {
            Some(Trust::Manager)
        } else {
            self.trusted.get(&player).copied()
        }
    }
}

/// The claim with corners `a` and `b` (any two opposite corners).
pub fn rectangle(id: u64, owner: u128, dimension: &str, a: [i32; 2], b: [i32; 2], created: i64) -> Claim {
    Claim {
        id,
        owner,
        dimension: dimension.to_owned(),
        min: [a[0].min(b[0]), a[1].min(b[1])],
        max: [a[0].max(b[0]), a[1].max(b[1])],
        trusted: BTreeMap::new(),
        created,
    }
}
