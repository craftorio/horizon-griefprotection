//! All claims, with an index by chunk: finding the claim at a block looks only at the claims of its chunk. The
//! bonus claim blocks of the players are kept here too, so they are saved with the claims.

use super::Claim;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Default)]
pub struct Claims {
    claims: BTreeMap<u64, Claim>,
    /// The claims that touch each chunk (dimension, chunk x, chunk z).
    chunks: HashMap<(String, i32, i32), Vec<u64>>,
    /// The bonus claim blocks that admins gave, by player.
    bonus: BTreeMap<u128, i64>,
    next: u64,
}

fn chunks_of(claim: &Claim) -> impl Iterator<Item = (String, i32, i32)> + '_ {
    let [x0, z0] = claim.min.map(|c| c >> 4);
    let [x1, z1] = claim.max.map(|c| c >> 4);
    (x0..=x1).flat_map(move |x| (z0..=z1).map(move |z| (claim.dimension.clone(), x, z)))
}

/// Why a claim cannot be made or changed.
#[derive(Clone, Debug, PartialEq)]
pub enum Refusal {
    /// It overlaps claim `id`.
    Overlaps(u64),
    /// It is narrower than `min_width` blocks, or smaller than `min_area` columns.
    TooSmall { min_width: i32, min_area: i64 },
    /// It needs `needed` claim blocks, and the player has `left`.
    NotEnoughBlocks { needed: i64, left: i64 },
    /// There is no claim with this id.
    NoClaim,
}

/// The size rules of `grief_protection.toml`, and the claim blocks the owner earned.
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub earned: i64,
    pub min_width: i32,
    pub min_area: i64,
}

impl Claims {
    pub fn from_saved(list: Vec<Claim>, bonus: BTreeMap<u128, i64>) -> Self {
        let mut claims = Claims {
            bonus,
            ..Claims::default()
        };
        for claim in list {
            claims.next = claims.next.max(claim.id + 1);
            claims.insert(claim);
        }
        claims
    }

    pub fn from_list(list: Vec<Claim>) -> Self {
        Self::from_saved(list, BTreeMap::new())
    }

    pub fn list(&self) -> Vec<Claim> {
        self.claims.values().cloned().collect()
    }

    pub fn bonuses(&self) -> &BTreeMap<u128, i64> {
        &self.bonus
    }

    pub fn bonus(&self, player: u128) -> i64 {
        self.bonus.get(&player).copied().unwrap_or(0)
    }

    /// Adds `amount` (it can be negative) to the bonus of `player`; returns the new bonus.
    pub fn add_bonus(&mut self, player: u128, amount: i64) -> i64 {
        let bonus = self.bonus(player).saturating_add(amount);
        if bonus == 0 {
            self.bonus.remove(&player);
        } else {
            self.bonus.insert(player, bonus);
        }
        bonus
    }

    fn insert(&mut self, claim: Claim) {
        for chunk in chunks_of(&claim) {
            self.chunks.entry(chunk).or_default().push(claim.id);
        }
        self.claims.insert(claim.id, claim);
    }

    pub fn next_id(&self) -> u64 {
        self.next.max(1)
    }

    /// The claim at block column `x`, `z` of `dimension`.
    pub fn at(&self, dimension: &str, x: i32, z: i32) -> Option<&Claim> {
        let ids = self.chunks.get(&(dimension.to_owned(), x >> 4, z >> 4))?;
        ids.iter()
            .filter_map(|id| self.claims.get(id))
            .find(|c| c.contains(dimension, x, z))
    }

    /// Whether column `x`, `z` is in a claim of any dimension.
    pub fn in_any_dimension(&self, x: i32, z: i32) -> bool {
        let mut dimensions: Vec<&str> = self.claims.values().map(|c| c.dimension.as_str()).collect();
        dimensions.sort_unstable();
        dimensions.dedup();
        dimensions.iter().any(|dimension| self.at(dimension, x, z).is_some())
    }

    pub fn get(&self, id: u64) -> Option<&Claim> {
        self.claims.get(&id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut Claim> {
        self.claims.get_mut(&id)
    }

    /// The claims of `owner`, in the order they were made.
    pub fn of_owner(&self, owner: u128) -> Vec<&Claim> {
        self.claims.values().filter(|c| c.owner == owner).collect()
    }

    /// The claim blocks the claims of `owner` use.
    pub fn used_by(&self, owner: u128) -> i64 {
        self.claims.values().filter(|c| c.owner == owner).map(Claim::area).sum()
    }

    /// The first claim that `claim` overlaps, other than claim `except`.
    pub fn overlap(&self, claim: &Claim, except: Option<u64>) -> Option<&Claim> {
        chunks_of(claim)
            .filter_map(|chunk| self.chunks.get(&chunk))
            .flatten()
            .filter(|id| Some(**id) != except)
            .filter_map(|id| self.claims.get(id))
            .find(|other| other.overlaps(claim))
    }

    /// Whether `claim` may stand: large enough, paid for, and over no other claim. `except` is the claim it
    /// takes the place of (a resize): its area is free and it may overlap itself.
    fn check(&self, claim: &Claim, limits: Limits, except: Option<u64>) -> Result<(), Refusal> {
        if claim.width() < limits.min_width || claim.area() < limits.min_area {
            return Err(Refusal::TooSmall {
                min_width: limits.min_width,
                min_area: limits.min_area,
            });
        }
        let old = except.and_then(|id| self.claims.get(&id)).map_or(0, Claim::area);
        let left = limits.earned - self.used_by(claim.owner) + old;
        if claim.area() > left {
            return Err(Refusal::NotEnoughBlocks {
                needed: claim.area(),
                left,
            });
        }
        match self.overlap(claim, except) {
            Some(other) => Err(Refusal::Overlaps(other.id)),
            None => Ok(()),
        }
    }

    /// Adds `claim` when it overlaps no other, is large enough, and its owner has the claim blocks for it.
    pub fn add(&mut self, mut claim: Claim, earned: i64, min_width: i32, min_area: i64) -> Result<u64, Refusal> {
        let limits = Limits {
            earned,
            min_width,
            min_area,
        };
        self.check(&claim, limits, None)?;
        claim.id = self.next_id();
        self.next = claim.id + 1;
        let id = claim.id;
        self.insert(claim);
        Ok(id)
    }

    /// Gives claim `id` the corners `a` and `b`, with the checks of a new claim. Its old area counts as free.
    /// `limits.earned` is the claim blocks of the claim's owner.
    pub fn resize(&mut self, id: u64, a: [i32; 2], b: [i32; 2], limits: Limits) -> Result<(), Refusal> {
        let old = self.claims.get(&id).ok_or(Refusal::NoClaim)?;
        let mut claim = old.clone();
        claim.min = [a[0].min(b[0]), a[1].min(b[1])];
        claim.max = [a[0].max(b[0]), a[1].max(b[1])];
        self.check(&claim, limits, Some(id))?;
        self.remove(id);
        self.insert(claim);
        Ok(())
    }

    pub fn remove(&mut self, id: u64) -> Option<Claim> {
        let claim = self.claims.remove(&id)?;
        for chunk in chunks_of(&claim) {
            if let Some(ids) = self.chunks.get_mut(&chunk) {
                ids.retain(|other| *other != id);
            }
        }
        Some(claim)
    }
}
