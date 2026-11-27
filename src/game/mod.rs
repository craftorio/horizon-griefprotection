//! The hooks' way to the rules. The state of the mod is here: the claims of the world (read from its data folder the
//! first time, and saved when they change), the settings, and what the mod knows of each player (their language,
//! their claim tool, the borders they see).

mod borders;
pub mod commands;
pub mod outside;
pub mod protect;
pub mod shovel;

use crate::claims::index::Claims;
use crate::config::Settings;
use crate::lang::{Key, Lang, say, styled};
use crate::store;
use crate::tool::Tool;
use mod_api::{ConfigFile, ConfigPath, ConsoleLevel, Host, HostSide, ServerHost, Text};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::sync::{Mutex, MutexGuard};

#[derive(Default)]
pub struct State {
    /// The claims of the world; `None` until a hook with the server reads them.
    claims: Option<Claims>,
    pub settings: Settings,
    /// The claims changed since they were saved.
    dirty: bool,
    /// Server ticks since the mod started.
    pub now: u64,
    save_at: u64,
    pub langs: BTreeMap<u128, Lang>,
    pub tools: BTreeMap<u128, Tool>,
    pub borders: BTreeMap<u128, borders::Shown>,
    /// The tick until which a player's `/abandonallclaims` waits for its confirmation.
    pub confirm: BTreeMap<u128, u64>,
    /// The tick and the block of each player's last click with the claim tool.
    pub tool_clicks: BTreeMap<u128, (u64, [i32; 3])>,
}

static STATE: Mutex<State> = Mutex::new(State {
    claims: None,
    settings: Settings::FALLBACK,
    dirty: false,
    now: 0,
    save_at: 0,
    langs: BTreeMap::new(),
    tools: BTreeMap::new(),
    borders: BTreeMap::new(),
    confirm: BTreeMap::new(),
    tool_clicks: BTreeMap::new(),
});

fn state() -> MutexGuard<'static, State> {
    STATE.lock().unwrap_or_else(|p| p.into_inner())
}

impl State {
    /// The claims; they are read when this is the first use.
    pub fn claims(&mut self, host: &mut dyn ServerHost) -> &mut Claims {
        self.claims.get_or_insert_with(|| load(host))
    }

    /// The claims after a change: they are saved within 5 seconds.
    pub fn changed(&mut self) {
        self.dirty = true;
    }

    pub fn lang(&self, player: u128) -> Lang {
        self.langs.get(&player).copied().unwrap_or_default()
    }
}

/// Runs `f` with the state of the mod.
pub fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    f(&mut state())
}

/// Runs `f` with the claims, when they were read already. Hooks without the server (fluids, fire, pistons) use
/// this: the claims are read at the first server tick, before such hooks run.
pub fn with_loaded<R>(f: impl FnOnce(&Claims) -> R) -> Option<R> {
    state().claims.as_ref().map(f)
}

fn load(host: &mut dyn ServerHost) -> Claims {
    let read = ConfigPath::new(store::FILE)
        .ok()
        .and_then(|path| host.world_data_files().read(&path).ok());
    let text = read.and_then(|bytes| String::from_utf8(bytes).ok());
    match text.map(|text| store::from_text(&text)) {
        Some(Ok(claims)) => claims,
        Some(Err(why)) => {
            // keep the file for a person to look at: nothing is saved over it until a claim changes
            log(
                host,
                ConsoleLevel::Error,
                &format!("{}: {why}; starting with no claims", store::FILE),
            );
            Claims::default()
        }
        None => Claims::default(),
    }
}

pub fn log(host: &mut dyn ServerHost, level: ConsoleLevel, line: &str) {
    let _ = host.messages().console_log(crate::NAMESPACE, level, line);
}

/// `config_loading` and `config_reloading`: the settings of `grief_protection.toml`.
pub fn take_config(host: HostSide, file: &ConfigFile) {
    if file.mod_id != crate::NAMESPACE {
        return;
    }
    match Settings::parse(&file.text) {
        Ok(settings) => with(|state| state.settings = settings),
        Err(why) => {
            if let HostSide::Server(host) = host {
                log(
                    host,
                    ConsoleLevel::Warn,
                    &format!("{}: {why}; the settings from before stay", file.file),
                );
            }
        }
    }
}

/// Each server tick: reads the claims the first time, saves them every 5 seconds when they changed, and keeps the
/// borders that players see.
pub fn tick(host: &mut dyn ServerHost) {
    let text = with(|state| {
        state.claims(host);
        state.now += 1;
        if !state.dirty || state.now < state.save_at {
            return None;
        }
        state.save_at = state.now + 100;
        state.dirty = false;
        state.claims.as_ref().map(store::to_text)
    });
    if let (Some(text), Ok(path)) = (text, ConfigPath::new(store::FILE)) {
        let _ = host.world_data_files().write(&path, text.as_bytes());
    }
    borders::tick(host);
}

/// `player_logged_out`: forget what the mod knows of the player.
pub fn logged_out(player: u128) {
    with(|state| {
        state.langs.remove(&player);
        state.tools.remove(&player);
        state.borders.remove(&player);
        state.confirm.remove(&player);
        state.tool_clicks.remove(&player);
    });
}

/// `player_changed_dimension`: the first corner or the resize of the claim tool is forgotten.
pub fn changed_dimension(player: u128) {
    with(|state| state.tools.remove(&player));
}

/// An operator of level 2 or more (the level of `/gamemode`) is an admin. LitePerms will add the permission
/// `griefprotection.admin`.
pub fn admin(host: &dyn Host, player: u128) -> bool {
    host.player(player)
        .and_then(|p| p.permission_level(host))
        .is_some_and(|level| level >= 2)
}

/// The dimension the player is in: `minecraft:overworld`.
pub fn dimension_of(host: &dyn Host, player: u128) -> Option<String> {
    host.player(player)?.dimension(host).map(|id| id.to_string())
}

/// The name of the player with `uuid`, from the server's user cache; the UUID when it is not known.
pub fn name_of(host: &dyn ServerHost, uuid: u128) -> String {
    host.profile_by_uuid(uuid)
        .map(|profile| profile.name)
        .unwrap_or_else(|| format!("{uuid:032x}"))
}

/// The text of `key` in the language of `player`.
pub fn text_for(player: u128, key: Key, values: &[(&str, &dyn Display)]) -> String {
    let lang = with(|state| state.lang(player));
    say(lang, key, values)
}

/// The message `key` in the language of `player`, coloured for chat.
pub fn styled_for(player: u128, key: Key, values: &[(&str, &dyn Display)]) -> Text {
    let lang = with(|state| state.lang(player));
    styled(lang, key, values)
}

/// Says `line` to `player` in chat.
pub fn tell_text(host: &mut dyn ServerHost, player: u128, line: Text) {
    if let Some(handle) = host.player(player) {
        let _ = handle.tell(host, line);
    }
}

/// Says `key` to `player` in chat.
pub fn tell(host: &mut dyn ServerHost, player: u128, key: Key, values: &[(&str, &dyn Display)]) {
    tell_text(host, player, styled_for(player, key, values));
}

/// Says `key` to `player` on their action bar.
pub fn action_bar(host: &mut dyn ServerHost, player: u128, key: Key, values: &[(&str, &dyn Display)]) {
    let line = styled_for(player, key, values);
    if let Some(handle) = host.player(player) {
        let _ = handle.action_bar(host, line);
    }
}

/// The claim blocks `player` earned (with their bonus). Only an online player has a play time; for another one,
/// this is only the bonus and the blocks of a new player.
pub fn earned(host: &dyn Host, player: u128, bonus: i64) -> i64 {
    let ticks = host
        .player(player)
        .and_then(|p| p.statistics(host))
        .map_or(0, |json| crate::claims::blocks::play_ticks(&json));
    let settings = with(|state| state.settings.clone());
    crate::claims::blocks::earned(&settings, ticks, bonus)
}
