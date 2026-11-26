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
