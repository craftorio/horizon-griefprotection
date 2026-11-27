//! Grief Protection: land claims for Horizon servers, inspired by GriefPrevention. A player claims land (whole
//! columns of a rectangle) with `/claim` or with a golden shovel; in a claim, only the players its owner trusts may
//! build, open containers, use doors and buttons, or hurt animals. Fluids, fire, pistons, explosions and mobs do not
//! change a claim from outside.
//!
//! The rules are in `claims`, `body`, `outside` and `tool` (tested without the game); `game` is the hooks' way to them.

pub mod body;
pub mod claims;
pub mod config;
mod game;
pub mod kind;
pub mod lang;
pub mod outside;
pub mod store;
#[cfg(test)]
mod tests;
pub mod tool;

use game::{outside as from_outside, protect};
use mod_api::{
    AttackEntityEvent, BlockBurnEvent, BlockIgniteEvent, BlockSpreadEvent, BreakBlockEvent, BucketEmptyEvent,
    ClientInformationUpdatedEvent, Config, ConfigKind, ConfigLoadingEvent, ConfigReloadingEvent, DispenserActsEvent,
    EntityInteractEvent, EntityInteractSpecificEvent, EntityMobGriefingEvent, EntityPlaceEvent, ExplosionDetonateEvent,
    FarmlandTrampleEvent, FillBucketEvent, Flow, Hand, Hooks, Host, HostSide, InteractionResult, LevelHost,
    PistonPreEvent, PlayerChangedDimensionEvent, PlayerLoggedOutEvent, RegisterCommandsEvent, RegisterConfigsEvent,
    RightClickBlockEvent, RunModCommandEvent, ServerHost, ServerTickEvent, ServerTickPhase,
};

pub const NAMESPACE: &str = "grief_protection";

#[derive(Default)]
pub struct Mod;

fn xyz(pos: mod_api::BlockPos) -> [i32; 3] {
    [pos.x, pos.y, pos.z]
}

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn register_configs(&self, _host: HostSide, event: &mut RegisterConfigsEvent<'_>) {
        event
            .configs
            .push(Config::new(NAMESPACE, ConfigKind::Server, config::DEFAULT).file(config::FILE));
    }

    fn config_loading(&self, host: HostSide, event: &mut ConfigLoadingEvent<'_>) {
        game::take_config(host, event.config);
    }

    fn config_reloading(&self, host: HostSide, event: &mut ConfigReloadingEvent<'_>) {
        game::take_config(host, event.config);
    }

    fn register_commands(&self, _host: &mut dyn ServerHost, event: &mut RegisterCommandsEvent<'_>) {
        event.commands.extend(game::commands::usages());
    }

    fn run_mod_command(&self, host: &mut dyn ServerHost, event: &mut RunModCommandEvent<'_>) -> Flow {
        game::commands::run(host, event)
    }

    fn client_information_updated(&self, _host: &mut dyn ServerHost, event: &mut ClientInformationUpdatedEvent<'_>) {
        let lang = lang::Lang::of(&event.new.language);
        game::with(|state| state.langs.insert(event.player, lang));
    }

    fn player_logged_out(&self, _host: &mut dyn ServerHost, event: &mut PlayerLoggedOutEvent<'_>) {
        game::logged_out(event.player);
    }

    fn player_changed_dimension(&self, _host: &mut dyn ServerHost, event: &mut PlayerChangedDimensionEvent<'_>) {
        game::changed_dimension(event.player);
    }

    fn break_block(&self, host: &mut dyn ServerHost, event: &mut BreakBlockEvent<'_>) -> Flow {
        protect::refuse_build(host, event.pos, &mut event.cancelled)
    }

    fn entity_place(&self, host: &mut dyn ServerHost, event: &mut EntityPlaceEvent) -> Flow {
        let pos = [event.pos.x, event.pos.y, event.pos.z];
        match event.placer.and_then(|placer| placer.uuid(host)) {
            Some(player) => protect::refuse_build_by(host, player, pos, &mut event.cancelled),
            None => Flow::Continue,
        }
    }

    /// A click with the claim tool, and a click that uses what the player may not use, are refused. After a refused
    /// click of the main hand, the client clicks the same block with the off hand: it is checked too, and the off
    /// hand's click after the claim tool's is refused, so that the game does not send the clicked block back.
    fn right_click_block(&self, host: HostSide, event: &mut RightClickBlockEvent<'_>) -> Flow {
        let HostSide::Server(host) = host else {
            return Flow::Continue;
        };
        let (player, pos) = (event.click.player, event.click.pos);
        let refuse = match event.click.hand {
            Hand::Main => {
                let item = event
                    .click
                    .stack
                    .map(|stack| stack.item().id(&*host))
                    .unwrap_or_default();
                let tool = game::shovel::click(host, player, pos, &item);
                if tool {
                    game::with(|state| state.tool_clicks.insert(player, (state.now, pos)));
                }
                tool || protect::refuse_click(host, player, pos)
            }
            _ => {
                let after_tool = game::with(|state| state.tool_clicks.get(&player) == Some(&(state.now, pos)));
                after_tool || protect::refuse_click(host, player, pos)
            }
        };
        if !refuse {
            return Flow::Continue;
        }
        *event.cancellation_result = InteractionResult::Fail;
        event.cancelled = true;
        Flow::Cancel
    }

    fn attack_entity(&self, host: &mut dyn ServerHost, event: &mut AttackEntityEvent<'_>) -> Flow {
        protect::refuse_hit(host, event.kind, event.pos, &mut event.cancelled)
    }

    fn entity_interact(&self, host: HostSide, event: &mut EntityInteractEvent<'_>) -> Option<InteractionResult> {
        let HostSide::Server(host) = host else {
            return None;
        };
        let player = host.players().acting_player().map(|p| p.uuid())?;
        protect::refuse_use(host, player, event.entity, event.kind).then_some(InteractionResult::Fail)
    }

    fn entity_interact_specific(&self, host: &mut dyn ServerHost, event: &mut EntityInteractSpecificEvent<'_>) -> Flow {
        let Some(player) = host.players().acting_player().map(|p| p.uuid()) else {
            return Flow::Continue;
        };
        let refuse = protect::refuse_use(host, player, event.entity, event.kind);
        protect::cancel(refuse, &mut event.cancelled)
    }

    fn bucket_empty(&self, host: &mut dyn ServerHost, event: &mut BucketEmptyEvent<'_>) -> Flow {
        protect::refuse_build_by(host, event.player, event.pos, &mut event.cancelled)
    }

    fn fill_bucket(&self, host: &mut dyn ServerHost, event: &mut FillBucketEvent<'_>) -> Flow {
        protect::refuse_acting_build(host, event.pos, &mut event.cancelled)
    }

    fn block_spread(&self, _level: &dyn LevelHost, event: &mut BlockSpreadEvent<'_>) -> Flow {
        let dimension = event.dimension.key().id();
        from_outside::spread(dimension, xyz(event.from), xyz(event.to), &mut event.cancelled)
    }

    fn block_burn(&self, _level: &dyn LevelHost, event: &mut BlockBurnEvent) -> Flow {
        from_outside::burn(event.dimension.key().id(), xyz(event.pos), &mut event.cancelled)
    }

    fn block_ignite(&self, _level: &dyn LevelHost, event: &mut BlockIgniteEvent<'_>) -> Flow {
        from_outside::ignite(
            event.dimension.key().id(),
            xyz(event.pos),
            event.player,
            &mut event.cancelled,
        )
    }

    fn piston_pre(&self, _level: &dyn LevelHost, event: &mut PistonPreEvent<'_>) -> Flow {
        let moved: Vec<[i32; 3]> = event.push.iter().copied().map(xyz).collect();
        let destroyed: Vec<[i32; 3]> = event.destroy.iter().copied().map(xyz).collect();
        let piston = outside::PistonMove {
            piston: xyz(event.piston),
            direction: event.direction,
            moved: &moved,
            destroyed: &destroyed,
        };
        from_outside::piston(event.dimension.key().id(), piston, &mut event.cancelled)
    }

    fn farmland_trample(&self, _context: &dyn mod_api::LoopContext, event: &mut FarmlandTrampleEvent) -> Flow {
        from_outside::trample(event.pos, &mut event.cancelled)
    }

    /// A dispenser outside a claim does not put out its item into the claim: the item stays, and the dispenser
    /// clicks as when it is empty.
    fn dispenser_acts(
        &self,
        _host: &mut dyn ServerHost,
        event: &mut DispenserActsEvent<'_>,
    ) -> Option<mod_api::Dispensed> {
        from_outside::dispense(event.dimension, event.pos, event.facing.normal())
            .then(|| (Some(event.stack.clone()), 1001))
    }

    fn explosion_detonate(&self, _host: &mut dyn ServerHost, event: &mut ExplosionDetonateEvent<'_>) {
        from_outside::explosion(event.dimension, event.blocks);
    }

    fn entity_mob_griefing(&self, host: &dyn Host, event: &mut EntityMobGriefingEvent) {
        from_outside::mob_griefing(host, event.entity, &mut event.can_grief);
    }

    fn server_tick(&self, host: &mut dyn ServerHost, event: &mut ServerTickEvent<'_>) {
        if matches!(event.phase, ServerTickPhase::ServerTickEnd { .. }) {
            game::tick(host);
        }
    }
}
