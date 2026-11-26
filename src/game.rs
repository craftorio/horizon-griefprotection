//! The hooks' way to the rules: the claims of the world in place (read from its data folder the first time, and
//! saved when they change), the player who acts, and a refusal said on their action bar.

use crate::claims::index::Claims;
use crate::claims::rules::{Action, allowed};
use crate::store;
use mod_api::{ConfigPath, Flow, Host, HostBlockStates, ServerHost, Text};
use std::sync::Mutex;

struct World {
    claims: Claims,
    /// The claims changed since they were saved.
    dirty: bool,
    ticks: u32,
}

static WORLD: Mutex<Option<World>> = Mutex::new(None);

fn with_claims<R>(host: &mut dyn ServerHost, f: impl FnOnce(&mut World) -> R) -> R {
    let mut guard = WORLD.lock().unwrap_or_else(|p| p.into_inner());
    let world = guard.get_or_insert_with(|| World {
        claims: load(host),
        dirty: false,
        ticks: 0,
    });
    f(world)
}

fn load(host: &mut dyn ServerHost) -> Claims {
    let read = ConfigPath::new(store::FILE)
        .ok()
        .and_then(|path| host.world_data_files().read(&path).ok());
    let list = read
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .map(|text| store::from_text(&text));
    match list {
        Some(Ok(list)) => Claims::from_list(list),
        Some(Err(why)) => {
            // keep the file for a person to look at: nothing is saved over it until a claim changes
            let line = format!("{}: {why}; starting with no claims", store::FILE);
            let _ = host
                .messages()
                .console_log(crate::NAMESPACE, mod_api::ConsoleLevel::Error, &line);
            Claims::default()
        }
        None => Claims::default(),
    }
}

/// Saves the claims every 5 seconds when they changed.
pub fn tick(host: &mut dyn ServerHost) {
    let text = with_claims(host, |world| {
        world.ticks = world.ticks.saturating_sub(1);
        if !world.dirty || world.ticks > 0 {
            return None;
        }
        world.ticks = 100;
        world.dirty = false;
        Some(store::to_text(world.claims.list()))
    });
    if let (Some(text), Ok(path)) = (text, ConfigPath::new(store::FILE)) {
        let _ = host.world_data_files().write(&path, text.as_bytes());
    }
}

/// An operator of level 2 or more (the level of `/gamemode`) is an admin. LitePerms will add the permission
/// `griefprotection.admin`.
fn admin(host: &dyn Host, player: u128) -> bool {
    host.player(player)
        .and_then(|p| p.permission_level(host))
        .is_some_and(|level| level >= 2)
}

/// Whether `player` may not do `action` at `pos` in their level; says so on their action bar.
fn refused(host: &mut dyn ServerHost, player: u128, pos: [i32; 3], action: Action) -> bool {
    let Some(handle) = host.player(player) else {
        return false;
    };
    let dimension = handle.dimension(host).map(|id| id.to_string()).unwrap_or_default();
    let is_admin = admin(host, player);
    let refused = with_claims(host, |world| {
        world
            .claims
            .at(&dimension, pos[0], pos[2])
            .is_some_and(|claim| allowed(claim, player, action, is_admin).is_err())
    });
    if refused {
        let _ = handle.action_bar(
            host,
            Text::literal("This land is claimed: its owner has not trusted you here."),
        );
    }
    refused
}

fn cancel(refused: bool, cancelled: &mut bool) -> Flow {
    if !refused {
        return Flow::Continue;
    }
    *cancelled = true;
    Flow::Cancel
}

/// `break_block`: the acting player breaks the block at `pos`.
pub fn refuse_build(host: &mut dyn ServerHost, pos: [i32; 3], cancelled: &mut bool) -> Flow {
    let Some(player) = host.players().acting_player().map(|p| p.uuid()) else {
        return Flow::Continue;
    };
    refuse_build_by(host, player, pos, cancelled)
}

pub fn refuse_build_by(host: &mut dyn ServerHost, player: u128, pos: [i32; 3], cancelled: &mut bool) -> Flow {
    cancel(refused(host, player, pos, Action::Build), cancelled)
}

/// `right_click_block`: whether the click of `player` on the block at `pos` uses something they may not use.
pub fn refuse_click(host: &mut dyn ServerHost, player: u128, pos: [i32; 3]) -> bool {
    let Some(level) = host.player(player).and_then(|p| p.level(host)) else {
        return false;
    };
    let Some(state) = level.block_state(host, pos) else {
        return false;
    };
    let Some(text) = host.block_state_string(state) else {
        return false;
    };
    let id = text.split('[').next().unwrap_or(&text).to_owned();
    let has_contents = host.block_states().has_block_entity(state) == Some(true);
    match crate::kind::click(&id, has_contents) {
        Some(action) => refused(host, player, pos, action),
        None => false,
    }
}

/// `attack_entity`: the acting player hits an entity of `kind` at `pos`. Monsters may be hit anywhere.
pub fn refuse_hit(host: &mut dyn ServerHost, kind: &str, pos: [f64; 3], cancelled: &mut bool) -> Flow {
    if kind == "minecraft:player" || host.entity_type_is_enemy(kind) {
        return Flow::Continue;
    }
    let Some(player) = host.players().acting_player().map(|p| p.uuid()) else {
        return Flow::Continue;
    };
    let block = pos.map(|c| c.floor() as i32);
    cancel(refused(host, player, block, Action::Container), cancelled)
}
