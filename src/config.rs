//! The mod's settings: `grief_protection.toml` (for each world in `serverconfig/`, else in `config/`).

use serde::Deserialize;

pub const FILE: &str = "grief_protection.toml";

pub const DEFAULT: &str = r#"# Grief Protection: land claims. A claim covers whole columns, from the bottom of the world to the top.

# The claim blocks a new player has, how many they get for each hour played, and the most they get by playing
# (an admin can give more).
initial_blocks = 100
blocks_per_hour = 100
max_blocks = 80000
# The smallest claim: this many blocks on each side, and this many columns.
min_width = 5
min_area = 100
"#;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Settings {
    pub initial_blocks: i64,
    pub blocks_per_hour: i64,
    pub max_blocks: i64,
    pub min_width: i32,
    pub min_area: i64,
}

impl Settings {
    pub fn parse(text: &str) -> Result<Self, String> {
        toml::from_str(text).map_err(|e: toml::de::Error| e.to_string())
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings::parse(DEFAULT).expect("the default settings parse")
    }
}
