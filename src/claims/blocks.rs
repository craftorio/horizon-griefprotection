//! Claim blocks, as GriefPrevention has them: a player may claim as many columns as they have claim blocks. They
//! start with some, and get more for each hour they play, up to a limit.

use crate::config::Settings;

/// The claim blocks of a player who played `play_ticks` (vanilla's `play_time` statistic, 20 a second) and was
/// given `bonus` blocks by an admin.
pub fn earned(settings: &Settings, play_ticks: u64, bonus: i64) -> i64 {
    let hours = i64::try_from(play_ticks / (20 * 3600)).unwrap_or(i64::MAX);
    let accrued = settings
        .initial_blocks
        .saturating_add(hours.saturating_mul(settings.blocks_per_hour));
    accrued.min(settings.max_blocks).saturating_add(bonus)
}

/// The `minecraft:play_time` statistic in the JSON of a player's `stats/<uuid>.json`; 0 when it is not there.
pub fn play_ticks(statistics: &str) -> u64 {
    let Ok(json) = mod_api::serde_json::from_str::<mod_api::serde_json::Value>(statistics) else {
        return 0;
    };
    json.pointer("/stats/minecraft:custom/minecraft:play_time")
        .and_then(mod_api::serde_json::Value::as_u64)
        .unwrap_or(0)
}
