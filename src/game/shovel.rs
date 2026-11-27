//! The claim tool in the game: a right click on a block with the golden shovel (make or change a claim) or with a
//! stick (see the claim there). The state of each player's shovel is in `crate::tool`.

use super::borders::{self, Look};
use super::{admin, dimension_of, earned, name_of, tell, with};
use crate::claims::index::{Limits, Refusal};
use crate::claims::rectangle;
use crate::lang::Key;
use crate::tool::{Click, SHOVEL, STICK, shovel};
use mod_api::ServerHost;

/// A right click of `player` on the block at `pos`, holding the item `item`. Whether the click was a claim tool's:
/// then the clicked block is not used.
pub fn click(host: &mut dyn ServerHost, player: u128, pos: [i32; 3], item: &str) -> bool {
    match item {
        SHOVEL => {
            golden_shovel(host, player, pos);
            true
        }
        STICK => {
            stick(host, player, pos);
            true
        }
        _ => false,
    }
}

/// Shows `player` the claim `id`, and says whose it is.
fn show_claim(host: &mut dyn ServerHost, player: u128, id: u64, near: i32, look: Look) {
    let Some(claim) = with(|state| state.claims(host).get(id).cloned()) else {
        return;
    };
    borders::show(host, player, claim.min, claim.max, near, look);
    let owner = name_of(host, claim.owner);
    tell(host, player, Key::OwnedBy, &[("owner", &owner)]);
}

fn stick(host: &mut dyn ServerHost, player: u128, pos: [i32; 3]) {
    let Some(dimension) = dimension_of(host, player) else {
        return;
    };
    let id = with(|state| state.claims(host).at(&dimension, pos[0], pos[2]).map(|c| c.id));
    match id {
        Some(id) => show_claim(host, player, id, pos[1], Look::Claim),
        None => tell(host, player, Key::NotClaimed, &[]),
    }
}

fn golden_shovel(host: &mut dyn ServerHost, player: u128, pos: [i32; 3]) {
    let Some(dimension) = dimension_of(host, player) else {
        return;
    };
    let is_admin = admin(host, player);
    let column = [pos[0], pos[2]];
    let click = with(|state| {
        let mut tool = state.tools.remove(&player).unwrap_or_default();
        let click = shovel(&mut tool, state.claims(host), player, is_admin, &dimension, column);
        state.tools.insert(player, tool);
        click
    });
    match click {
        Click::FirstCorner => {
            borders::mark(host, player, pos);
            tell(host, player, Key::FirstCorner, &[]);
        }
        Click::ResizeStart { claim } => {
            show_claim(host, player, claim, pos[1], Look::Claim);
            tell(host, player, Key::ResizeStart, &[]);
        }
        Click::Show { claim } => show_claim(host, player, claim, pos[1], Look::Claim),
        Click::Make { a, b } => make(host, player, &dimension, a, b, pos[1]),
        Click::Resize { claim, a, b } => resize(host, player, claim, a, b, pos[1]),
    }
}

/// The claim blocks `owner` has left after their claims.
pub fn left(host: &mut dyn ServerHost, owner: u128) -> i64 {
    let (bonus, used) = with(|state| {
        let claims = state.claims(host);
        (claims.bonus(owner), claims.used_by(owner))
    });
    earned(host, owner, bonus) - used
}

fn limits(host: &mut dyn ServerHost, owner: u128) -> Limits {
    let bonus = with(|state| state.claims(host).bonus(owner));
    let earned = earned(host, owner, bonus);
    let settings = with(|state| state.settings.clone());
    Limits {
        earned,
        min_width: settings.min_width,
        min_area: settings.min_area,
    }
}

/// Makes a claim of `player` with corners `a` and `b`; says how it went.
pub fn make(host: &mut dyn ServerHost, player: u128, dimension: &str, a: [i32; 2], b: [i32; 2], near: i32) {
    let limits = limits(host, player);
    let claim = rectangle(0, player, dimension, a, b, 0);
    let (min, max, area) = (claim.min, claim.max, claim.area());
    let added = with(|state| {
        let added = state
            .claims(host)
            .add(claim, limits.earned, limits.min_width, limits.min_area);
        if added.is_ok() {
            state.changed();
        }
        added
    });
    match added {
        Ok(_) => {
            borders::show(host, player, min, max, near, Look::Claim);
            let left = left(host, player);
            tell(host, player, Key::Claimed, &[("area", &area), ("left", &left)]);
        }
        Err(refusal) => {
            borders::show(host, player, min, max, near, Look::Refused);
            refused(host, player, refusal, near);
        }
    }
}

fn resize(host: &mut dyn ServerHost, player: u128, id: u64, a: [i32; 2], b: [i32; 2], near: i32) {
    let Some(owner) = with(|state| state.claims(host).get(id).map(|c| c.owner)) else {
        return;
    };
    let limits = limits(host, owner);
    let done = with(|state| {
        let done = state.claims(host).resize(id, a, b, limits);
        if done.is_ok() {
            state.changed();
        }
        done
    });
    match done {
        Ok(()) => {
            show_claim(host, player, id, near, Look::Claim);
            let area = with(|state| state.claims(host).get(id).map_or(0, |c| c.area()));
            let left = left(host, owner);
            tell(host, player, Key::Resized, &[("area", &area), ("left", &left)]);
        }
        Err(refusal) => refused(host, player, refusal, near),
    }
}

/// Tells `player` why a claim cannot be made or changed. An overlap shows the other claim.
pub fn refused(host: &mut dyn ServerHost, player: u128, refusal: Refusal, near: i32) {
    match refusal {
        Refusal::Overlaps(other) => {
            let other = with(|state| state.claims(host).get(other).cloned());
            if let Some(other) = other {
                borders::show(host, player, other.min, other.max, near, Look::Refused);
                let owner = name_of(host, other.owner);
                tell(host, player, Key::Overlaps, &[("owner", &owner)]);
            }
        }
        Refusal::TooSmall { min_width, min_area } => {
            tell(
                host,
                player,
                Key::TooSmall,
                &[("width", &min_width), ("area", &min_area)],
            );
        }
        Refusal::NotEnoughBlocks { needed, left } => {
            tell(
                host,
                player,
                Key::NotEnoughBlocks,
                &[("needed", &needed), ("left", &left)],
            );
        }
        Refusal::NoClaim => {}
    }
}
