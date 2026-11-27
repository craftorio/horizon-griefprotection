//! The claims of a world, saved as `claims.json` in the world's data folder for this mod
//! (`<world>/horizon-mod-data/grief_protection/`), so they are copied and backed up with the world.
//!
//! Format 2 has the claims and the players' bonus claim blocks. Format 1 (Grief Protection 0.1.0) has only the
//! claims; it is read as format 2 with no bonus.

use crate::claims::Claim;
use crate::claims::index::Claims;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const FILE: &str = "claims.json";
/// The format version this mod writes.
pub const VERSION: u32 = 2;

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Player {
    #[serde(default)]
    pub bonus: i64,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Saved {
    pub version: u32,
    pub claims: Vec<Claim>,
    /// By UUID.
    #[serde(default)]
    pub players: BTreeMap<u128, Player>,
}

pub fn to_text(claims: &Claims) -> String {
    let players = claims
        .bonuses()
        .iter()
        .map(|(uuid, bonus)| (*uuid, Player { bonus: *bonus }))
        .collect();
    mod_api::serde_json::to_string_pretty(&Saved {
        version: VERSION,
        claims: claims.list(),
        players,
    })
    .unwrap_or_default()
}

pub fn from_text(text: &str) -> Result<Claims, String> {
    let saved: Saved = mod_api::serde_json::from_str(text).map_err(|e| e.to_string())?;
    if !(1..=VERSION).contains(&saved.version) {
        return Err(format!(
            "claims file version {}, this mod reads 1 to {VERSION}",
            saved.version
        ));
    }
    let bonus = saved
        .players
        .into_iter()
        .filter(|(_, player)| player.bonus != 0)
        .map(|(uuid, player)| (uuid, player.bonus))
        .collect();
    Ok(Claims::from_saved(saved.claims, bonus))
}
