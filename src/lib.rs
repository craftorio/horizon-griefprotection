//! Grief Protection: land claims for Horizon servers, inspired by GriefPrevention. A player claims land (whole
//! columns of a rectangle); in a claim, only the players its owner trusts may build, open containers, use doors
//! and buttons, or hurt animals. Status: a draft. The rules and the claims are here; the commands that make
//! claims (`/claim`, `/trust`...) and the claim tool come next.
//!
//! The rules are in `claims` (tested without the game); `game.rs` is the hooks' way to them.

pub mod claims;
pub mod config;
mod game;
pub mod kind;
pub mod store;

use mod_api::{
    AttackEntityEvent, BreakBlockEvent, EntityPlaceEvent, Flow, Hand, Hooks, HostSide, InteractionResult,
    RightClickBlockEvent, ServerHost, ServerTickEvent, ServerTickPhase,
};

pub const NAMESPACE: &str = "grief_protection";

#[derive(Default)]
pub struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn break_block(&self, host: &mut dyn ServerHost, event: &mut BreakBlockEvent<'_>) -> Flow {
        game::refuse_build(host, event.pos, &mut event.cancelled)
    }

    fn entity_place(&self, host: &mut dyn ServerHost, event: &mut EntityPlaceEvent) -> Flow {
        let pos = [event.pos.x, event.pos.y, event.pos.z];
        match event.placer.and_then(|placer| placer.uuid(host)) {
            Some(player) => game::refuse_build_by(host, player, pos, &mut event.cancelled),
            None => Flow::Continue,
        }
    }

    fn right_click_block(&self, host: HostSide, event: &mut RightClickBlockEvent<'_>) -> Flow {
        let (HostSide::Server(host), Hand::Main) = (host, event.click.hand) else {
            return Flow::Continue;
        };
        if !game::refuse_click(host, event.click.player, event.click.pos) {
            return Flow::Continue;
        }
        *event.cancellation_result = InteractionResult::Fail;
        event.cancelled = true;
        Flow::Cancel
    }

    fn attack_entity(&self, host: &mut dyn ServerHost, event: &mut AttackEntityEvent<'_>) -> Flow {
        game::refuse_hit(host, event.kind, event.pos, &mut event.cancelled)
    }

    fn server_tick(&self, host: &mut dyn ServerHost, event: &mut ServerTickEvent<'_>) {
        if matches!(event.phase, ServerTickPhase::ServerTickEnd { .. }) {
            game::tick(host);
        }
    }
}
