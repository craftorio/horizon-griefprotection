//! The claims of a world, saved as `claims.json` in the world's data folder for this mod
//! (`<world>/horizon-mod-data/grief_protection/`), so they are copied and backed up with the world.

use crate::claims::Claim;
use serde::{Deserialize, Serialize};

pub const FILE: &str = "claims.json";
/// The format version this mod reads and writes.
pub const VERSION: u32 = 1;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Saved {
    pub version: u32,
    pub claims: Vec<Claim>,
}

pub fn to_text(claims: Vec<Claim>) -> String {
    mod_api::serde_json::to_string_pretty(&Saved {
        version: VERSION,
        claims,
    })
    .unwrap_or_default()
}

pub fn from_text(text: &str) -> Result<Vec<Claim>, String> {
    let saved: Saved = mod_api::serde_json::from_str(text).map_err(|e| e.to_string())?;
    if saved.version != VERSION {
        return Err(format!(
            "claims file version {}, this mod reads {VERSION}",
            saved.version
        ));
    }
    Ok(saved.claims)
}
