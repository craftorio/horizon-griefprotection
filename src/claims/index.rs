//! All claims, with an index by chunk: finding the claim at a block looks only at the claims of its chunk.

use super::Claim;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Default)]
pub struct Claims {
    claims: BTreeMap<u64, Claim>,
    /// The claims that touch each chunk (dimension, chunk x, chunk z).
    chunks: HashMap<(String, i32, i32), Vec<u64>>,
    next: u64,
}

fn chunks_of(claim: &Claim) -> impl Iterator<Item = (String, i32, i32)> + '_ {
    let [x0, z0] = claim.min.map(|c| c >> 4);
    let [x1, z1] = claim.max.map(|c| c >> 4);
    (x0..=x1).flat_map(move |x| (z0..=z1).map(move |z| (claim.dimension.clone(), x, z)))
}

/// Why a claim cannot be made.
#[derive(Clone, Debug, PartialEq)]
pub enum Refusal {
    /// It overlaps claim `id`.
    Overlaps(u64),
    /// It is narrower than `min` blocks, or smaller than `min_area` columns.
    TooSmall { min_width: i32, min_area: i64 },
    /// It needs `needed` claim blocks, and the player has `left`.
    NotEnoughBlocks { needed: i64, left: i64 },
}

impl Claims {
    pub fn from_list(list: Vec<Claim>) -> Self {
        let mut claims = Claims::default();
        for claim in list {
            claims.next = claims.next.max(claim.id + 1);
            claims.insert(claim);
        }
        claims
    }

    pub fn list(&self) -> Vec<Claim> {
        self.claims.values().cloned().collect()
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

    pub fn get_mut(&mut self, id: u64) -> Option<&mut Claim> {
        self.claims.get_mut(&id)
    }

    /// The claim blocks the claims of `owner` use.
    pub fn used_by(&self, owner: u128) -> i64 {
        self.claims.values().filter(|c| c.owner == owner).map(Claim::area).sum()
    }

    /// Adds `claim` when it overlaps no other, is large enough, and its owner has the claim blocks for it.
    pub fn add(&mut self, mut claim: Claim, earned: i64, min_width: i32, min_area: i64) -> Result<u64, Refusal> {
        if claim.width() < min_width || claim.area() < min_area {
            return Err(Refusal::TooSmall { min_width, min_area });
        }
        let left = earned - self.used_by(claim.owner);
        if claim.area() > left {
            return Err(Refusal::NotEnoughBlocks {
                needed: claim.area(),
                left,
            });
        }
        if let Some(other) = chunks_of(&claim)
            .filter_map(|chunk| self.chunks.get(&chunk))
            .flatten()
            .filter_map(|id| self.claims.get(id))
            .find(|other| other.overlaps(&claim))
        {
            return Err(Refusal::Overlaps(other.id));
        }
        claim.id = self.next_id();
        self.next = claim.id + 1;
        let id = claim.id;
        self.insert(claim);
        Ok(id)
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
