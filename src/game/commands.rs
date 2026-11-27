//! The commands, as GriefPrevention has them: `/claim`, `/abandonclaim`, `/abandonallclaims`, the trust commands,
//! `/trustlist`, `/claimslist` and `/adjustbonusclaimblocks`.

use super::shovel::{left, make};
use super::{admin, dimension_of, earned, name_of, tell, text_for, with};
use crate::claims::Trust;
use crate::claims::rules::{Action, allowed};
use crate::lang::{Key, Lang, button, say};
use mod_api::{Command, CommandArgumentValue, CommandSource, Flow, RunModCommandEvent, ServerHost, Text};
use std::fmt::Display;

/// The usages the mod registers.
pub fn usages() -> Vec<Command> {
    let mut usages = vec![
        Command::new("claim [radius:integer]"),
        Command::new("abandonclaim"),
        Command::new("abandonallclaims"),
        Command::new("trustlist"),
        Command::new("claimslist"),
        Command::op("claimslist <player:player_name>", 2),
        Command::op("adjustbonusclaimblocks <player:player_name> <amount:integer>", 2),
    ];
    for name in TRUST.iter().map(|(name, _)| name) {
        usages.push(Command::new(&format!("{name} <player:player_name>")));
    }
    usages
}

/// The trust commands and the level each gives (`untrust`: none).
const TRUST: &[(&str, Option<Trust>)] = &[
    ("accesstrust", Some(Trust::Access)),
    ("containertrust", Some(Trust::Container)),
    ("trust", Some(Trust::Build)),
    ("permissiontrust", Some(Trust::Manager)),
    ("untrust", None),
];

/// Who runs a command: a player, or the console (and command blocks and functions).
#[derive(Clone, Copy)]
struct Runner(Option<u128>);

impl Runner {
    fn say(self, host: &mut dyn ServerHost, key: Key, values: &[(&str, &dyn Display)]) {
        match self.0 {
            Some(player) => tell(host, player, key, values),
            None => {
                let _ = host.commands().feedback(Text::literal(say(Lang::En, key, values)));
            }
        }
    }
}

/// `run_mod_command`.
pub fn run(host: &mut dyn ServerHost, event: &mut RunModCommandEvent<'_>) -> Flow {
    let Some(name) = event
        .command
        .literals
        .first()
        .map(|s| s.trim_start_matches('/').to_owned())
    else {
        return Flow::Continue;
    };
    let runner = match event.source {
        CommandSource::Player { player, .. } => Runner(Some(player.uuid())),
        _ => Runner(None),
    };
    let text = |name: &str| match event.command.argument(name) {
        Some(CommandArgumentValue::String(text)) => Some(text.clone()),
        _ => None,
    };
    let integer = |name: &str| match event.command.argument(name) {
        Some(CommandArgumentValue::Integer(value)) => Some(*value),
        _ => None,
    };
    match name.as_str() {
        "claimslist" => claims_list(host, runner, text("player")),
        "adjustbonusclaimblocks" => bonus(host, runner, text("player"), integer("amount").unwrap_or(0)),
        _ => {
            let Runner(Some(player)) = runner else {
                if !usages()
                    .iter()
                    .any(|u| u.usage.split(' ').next() == Some(name.as_str()))
                {
                    return Flow::Continue;
                }
                runner.say(host, Key::PlayersOnly, &[]);
                return Flow::Cancel;
            };
            match name.as_str() {
                "claim" => claim(host, player, integer("radius")),
                "abandonclaim" => abandon(host, player),
                "abandonallclaims" => abandon_all(host, player),
                "trustlist" => trust_list(host, player),
                other => match TRUST.iter().find(|(name, _)| *name == other) {
                    Some((_, level)) => trust(host, player, text("player"), *level),
                    None => return Flow::Continue,
                },
            }
        }
    }
    event.cancelled = true;
    Flow::Cancel
}

/// Where `player` stands: their dimension and block.
fn place(host: &mut dyn ServerHost, player: u128) -> Option<(String, [i32; 3])> {
    let dimension = dimension_of(host, player)?;
    let pos = host.player(player)?.entity().block_position(host)?;
    Some((dimension, pos))
}

/// The id of the claim where `player` stands.
fn claim_here(host: &mut dyn ServerHost, player: u128) -> Option<u64> {
    let (dimension, pos) = place(host, player)?;
    with(|state| state.claims(host).at(&dimension, pos[0], pos[2]).map(|c| c.id))
}

/// The UUID of the player named `name`, from the server's user cache; tells `runner` when nobody has this name.
fn uuid_of(host: &mut dyn ServerHost, runner: Runner, name: Option<String>) -> Option<(u128, String)> {
    let name = name?;
    match host.profile_by_name(&name) {
        Some(profile) => Some((profile.uuid, profile.name)),
        None => {
            runner.say(host, Key::UnknownPlayer, &[("name", &name)]);
            None
        }
    }
}

/// `/claim [radius]`: a square around the player. Without a radius, the smallest square the settings allow.
fn claim(host: &mut dyn ServerHost, player: u128, radius: Option<i32>) {
    let Some((dimension, pos)) = place(host, player) else {
        return;
    };
    let settings = with(|state| state.settings.clone());
    let radius = radius.unwrap_or_else(|| default_radius(settings.min_width, settings.min_area));
    let a = [pos[0] - radius, pos[2] - radius];
    let b = [pos[0] + radius, pos[2] + radius];
    make(host, player, &dimension, a, b, pos[1]);
}

/// The radius of the smallest square claim: at least `min_width` blocks on a side and `min_area` columns.
pub fn default_radius(min_width: i32, min_area: i64) -> i32 {
    let mut side = min_width.max(1);
    while i64::from(side) * i64::from(side) < min_area {
        side += 1;
    }
    side / 2
}

fn abandon(host: &mut dyn ServerHost, player: u128) {
    let Some(id) = claim_here(host, player) else {
        tell(host, player, Key::NotInClaim, &[]);
        return;
    };
    let is_admin = admin(host, player);
    let removed = with(|state| {
        let claims = state.claims(host);
        let owner = claims.get(id).map(|c| c.owner)?;
        if owner != player && !is_admin {
            return None;
        }
        claims.remove(id);
        state.changed();
        Some(owner)
    });
    match removed {
        Some(owner) => {
            super::borders::hide(host, player);
            let left = left(host, owner);
            tell(host, player, Key::Abandoned, &[("left", &left)]);
        }
        None => tell(host, player, Key::OnlyOwnerRemoves, &[]),
    }
}

fn abandon_all(host: &mut dyn ServerHost, player: u128) {
    let removed = with(|state| {
        let ids: Vec<u64> = state.claims(host).of_owner(player).iter().map(|c| c.id).collect();
        if ids.is_empty() {
            return None;
        }
        let confirmed = state.confirm.remove(&player).is_some_and(|until| state.now <= until);
        if !confirmed {
            state.confirm.insert(player, state.now + 20 * 30);
            return Some((ids.len(), false));
        }
        for id in &ids {
            state.claims(host).remove(*id);
        }
        state.changed();
        Some((ids.len(), true))
    });
    match removed {
        None => tell(host, player, Key::NoClaims, &[]),
        Some((count, false)) => {
            let lang = with(|state| state.lang(player));
            let line = super::styled_for(player, Key::ConfirmAbandonAll, &[("count", &count)])
                .append(Text::literal(" "))
                .append(button(lang, Key::ConfirmButton, Key::ConfirmHover, "/abandonallclaims"));
            super::tell_text(host, player, line);
        }
        Some((count, true)) => {
            super::borders::hide(host, player);
            let left = left(host, player);
            tell(host, player, Key::AbandonedAll, &[("count", &count), ("left", &left)]);
        }
    }
}

/// The name of a trust level in the language of `player`.
fn level_name(player: u128, level: Trust) -> String {
    let key = match level {
        Trust::Access => Key::TrustAccess,
        Trust::Container => Key::TrustContainer,
        Trust::Build => Key::TrustBuild,
        Trust::Manager => Key::TrustManager,
    };
    text_for(player, key, &[])
}

/// The trust commands: `level` for the player named `name`, or no trust (`untrust`). In a claim: that claim, with
/// manager trust. Outside a claim: every claim of `player`.
fn trust(host: &mut dyn ServerHost, player: u128, name: Option<String>, level: Option<Trust>) {
    let Some((target, name)) = uuid_of(host, Runner(Some(player)), name) else {
        return;
    };
    let is_admin = admin(host, player);
    let here = claim_here(host, player);
    let changed = with(|state| {
        let claims = state.claims(host);
        let ids: Vec<u64> = match here {
            Some(id) => {
                let claim = claims.get(id)?;
                if allowed(claim, player, Action::Manage, is_admin).is_err() {
                    return Some(Err(Key::NeedManager));
                }
                if level == Some(Trust::Manager) && claim.owner != player && !is_admin {
                    return Some(Err(Key::OnlyOwnerManager));
                }
                vec![id]
            }
            None => claims.of_owner(player).iter().map(|c| c.id).collect(),
        };
        if ids.is_empty() {
            return Some(Err(Key::NoClaims));
        }
        for id in &ids {
            if let Some(claim) = claims.get_mut(*id) {
                match level {
                    Some(level) => claim.trusted.insert(target, level),
                    None => claim.trusted.remove(&target),
                };
            }
        }
        state.changed();
        Some(Ok(ids.len()))
    });
    match changed {
        None => {}
        Some(Err(key)) => tell(host, player, key, &[]),
        Some(Ok(count)) => match level {
            Some(level) => {
                let level = level_name(player, level);
                tell(
                    host,
                    player,
                    Key::Trusted,
                    &[("name", &name), ("level", &level), ("count", &count)],
                );
            }
            None => tell(host, player, Key::Untrusted, &[("name", &name), ("count", &count)]),
        },
    }
}

fn trust_list(host: &mut dyn ServerHost, player: u128) {
    let Some(id) = claim_here(host, player) else {
        tell(host, player, Key::NotInClaim, &[]);
        return;
    };
    let Some(claim) = with(|state| state.claims(host).get(id).cloned()) else {
        return;
    };
    let owner = name_of(host, claim.owner);
    tell(host, player, Key::TrustListHead, &[("owner", &owner)]);
    if claim.trusted.is_empty() {
        tell(host, player, Key::TrustListEmpty, &[]);
    }
    for level in [Trust::Manager, Trust::Build, Trust::Container, Trust::Access] {
        let names: Vec<String> = claim
            .trusted
            .iter()
            .filter(|(_, l)| **l == level)
            .map(|(uuid, _)| name_of(host, *uuid))
            .collect();
        if !names.is_empty() {
            let level = level_name(player, level);
            let names = names.join(", ");
            tell(
                host,
                player,
                Key::TrustListLine,
                &[("level", &level), ("names", &names)],
            );
        }
    }
}

/// `/claimslist [player]`: the claims and the claim blocks of the runner, or of another player (admins).
fn claims_list(host: &mut dyn ServerHost, runner: Runner, name: Option<String>) {
    let (target, name) = match (name, runner.0) {
        (Some(name), _) => match uuid_of(host, runner, Some(name)) {
            Some(found) => found,
            None => return,
        },
        (None, Some(player)) => (player, name_of(host, player)),
        (None, None) => {
            runner.say(host, Key::PlayersOnly, &[]);
            return;
        }
    };
    let (claims, bonus, used) = with(|state| {
        let claims = state.claims(host);
        let list: Vec<_> = claims.of_owner(target).into_iter().cloned().collect();
        (list, claims.bonus(target), claims.used_by(target))
    });
    if host.player(target).is_some() {
        let earned = earned(host, target, bonus);
        let left = earned - used;
        let values: [(&str, &dyn Display); 5] = [
            ("name", &name),
            ("earned", &earned),
            ("bonus", &bonus),
            ("used", &used),
            ("left", &left),
        ];
        runner.say(host, Key::BlocksOnline, &values);
    } else {
        runner.say(
            host,
            Key::BlocksOffline,
            &[("name", &name), ("bonus", &bonus), ("used", &used)],
        );
    }
    for claim in claims {
        let area = claim.area();
        let values: [(&str, &dyn Display); 6] = [
            ("dimension", &claim.dimension),
            ("x1", &claim.min[0]),
            ("z1", &claim.min[1]),
            ("x2", &claim.max[0]),
            ("z2", &claim.max[1]),
            ("area", &area),
        ];
        runner.say(host, Key::ClaimLine, &values);
    }
}

/// `/adjustbonusclaimblocks <player> <amount>` (admins and the console).
fn bonus(host: &mut dyn ServerHost, runner: Runner, name: Option<String>, amount: i32) {
    let Some((target, name)) = uuid_of(host, runner, name) else {
        return;
    };
    let bonus = with(|state| {
        let bonus = state.claims(host).add_bonus(target, i64::from(amount));
        state.changed();
        bonus
    });
    runner.say(host, Key::BonusSet, &[("name", &name), ("bonus", &bonus)]);
}
