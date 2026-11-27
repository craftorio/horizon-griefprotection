## Context

Grief Protection is a WebAssembly server mod for our Horizon fork (`craftorio/horizon`, the commit in
`HORIZON_COMMIT`). The rules live in `src/claims/` and do not use the game, so they have tests. The hooks in
`src/lib.rs` go to the rules through `src/game.rs`. The Horizon mod API gives what this change needs:

- `register_commands` and `run_mod_command`: typed commands, on native and WebAssembly mods. The `player_name`
  argument takes the name of a player who is not online too.
- `host.profile_by_name(name)` and `host.profile_by_uuid(uuid)`: names and UUIDs from the server's user cache,
  without a request to Mojang.
- `player.statistics(host)`: all statistics of an online player, as the JSON of `stats/<uuid>.json`.
- `player.show_block(host, pos, state)`: one block that only that player sees. The real block comes back when the
  game sends that block or its chunk again.
- `client_information_updated`: the player's client language, once before `player_logged_in` and after each
  change.
- Hooks for fluids (`fluid_flow`), fire (`block_spread`, `block_ignite`, `block_burn`), pistons (`piston_*`),
  explosions (`explosion_detonate`), mobs (`entity_mob_griefing`), buckets (`bucket_empty`, `fill_bucket`),
  farmland (`farmland_trample`) and entities (`entity_interact`, `entity_interact_specific`).

## Goals / Non-Goals

**Goals:** a player can make, change, list and remove claims, and trust players, with commands and with the
golden shovel. A claim is safe from outside. The messages are in the player's language.

**Non-Goals:** see the proposal.

## Decisions

### Claim blocks: play time from the statistic, the bonus in `claims.json`

`earned = min(initial + hours * per_hour, max) + bonus`, as `claims::blocks::earned` has it now. The hours come
from `minecraft:custom` / `minecraft:play_time` of `player.statistics`. That needs an online player. Only an online
player makes or changes a claim, so that is enough. `/claimslist` for a player who is not online shows the blocks
that they use and their bonus, not the blocks that they earned.

The bonus is saved in `claims.json`, not in the player's persistent data. Reason: an admin can change the bonus
of a player who is not online, and the bonus is backed up with the claims.

Format 2:

```json
{ "version": 2, "claims": [ ... ], "players": { "<uuid>": { "bonus": 500 } } }
```

The mod reads version 1 as version 2 with no players. It always writes version 2.

Rejected: a separate `players.json`. Two files can be saved at different times, and a backup can have one
without the other.

### Commands

Each command is one `Command::new(...)` usage. `run_mod_command` gives the command to `src/commands.rs` by its
literals. Players run the commands at permission level 0. `/adjustbonusclaimblocks` and `/claimslist <player>`
ask for level 2. The console can run `/adjustbonusclaimblocks` and `/claimslist <player>`, and gets the other
commands refused with a message.

- `/claim [radius]`: a square with its centre at the player's column. Its side is `2 * radius + 1`. With no
  radius, it is the smallest such square that `min_width` and `min_area` allow (11 by 11 with the defaults). The
  usual refusals of `Claims::add` apply.
- `/abandonclaim`: the owner, or an admin, removes the claim where the player stands. `/abandonallclaims`
  removes all claims of the player. It asks for the same command again within 30 seconds, as a confirmation.
- The trust commands take a `player_name`. The name goes to a UUID through `profile_by_name`. A name that the
  server never saw is refused: "no player with this name has been on this server".
- To trust, the player needs manager trust in the claim. To give manager trust (`/permissiontrust`), the player
  must be the owner or an admin. Outside a claim, the command changes all claims that the player owns, as in
  GriefPrevention.

### The claim tool

The tool items are fixed: `minecraft:golden_shovel` and `minecraft:stick`. They are not in the settings. The
`right_click_block` hook reads `event.click.stack`. A click with a tool does not use the block (a chest does not
open), so the hook cancels it.

The state of the tool is kept for each player in memory (`src/tool.rs`):

- `Idle`.
- `Corner { dimension, pos }`: the first corner of a new claim.
- `Resize { claim, fixed corner }`: the player clicked a corner of a claim that they own (or any claim for an
  admin). The next click is the new corner. The other corner stays where it is.

The state goes back to `Idle` when the player changes dimension, logs out, or holds another item for one click.

A resize uses `Claims::resize`. It checks the size, the overlap and the claim blocks in the same way as `add`,
but it does not count the claim's old area as used.

### Borders that one player sees

`show_block` shows a block only to one player, for a short time: the real block comes back when its chunk is
sent again. The mod keeps the shown blocks of each player for 60 seconds and shows them again every 5 seconds.
After 60 seconds, or when the player sees other borders, the mod sends the real block state again.

The blocks: the corners in glowstone, and a block every 10 blocks along the sides in gold. For a claim that is
being made or is not valid (an overlap), the corners are redstone ore and the sides netherrack, so the player sees
the problem. The API has no height map. So the y of each border block is the top block of its column (the highest
block that is not air, with air above it) within 24 blocks above or below the clicked block.

### Protection from outside

A thing from outside a claim may not change a block in it. "From outside" means that the source block is in no
claim, or in a claim of another owner. A source in the same claim, or in a claim of the same owner, may change the
block. This is the GriefPrevention rule.

| Source | Hook | What happens |
|---|---|---|
| fluid, fire | `block_spread` (it asks for both) | the flow or the fire does not go into the claim from outside |
| fire | `block_burn` | no block of a claim burns away |
| fire without a player | `block_ignite` | does not start in a claim |
| a player's fire | `block_ignite` | needs build trust; the hook has no server, so admins are not known and need build trust too |
| piston | `piston_pre` | the piston does not move when a block next to it or next to a moved block (on both sides along the direction) is in another owner's claim |
| dispenser | `dispenser_acts` | a dispenser does not put out its item into another owner's claim: the item stays, the dispenser plays the fail sound |
| explosion | `explosion_detonate` | the blocks in claims are taken out of the list of blocks that break, for every explosion; entities still take damage |
| mob | `entity_mob_griefing` | the mobs of a list (endermen, ravagers, silverfish, withers, zombies, creepers, ghasts, blazes, evokers, the dragon) do not change blocks in a claim |
| bucket | `bucket_empty`, `fill_bucket` | needs build trust |
| farmland | `farmland_trample` | nobody tramples farmland in a claim |
| item frame, armor stand, animal, villager | `entity_interact`, `entity_interact_specific`, `attack_entity` | needs container trust |

Explosions: every explosion spares the blocks in claims, also one that starts in a claim (an owner's TNT). The hook
does not say what started the explosion. GriefPrevention also stops explosions in claims by default.

Mobs: a list of the mobs that break, take or burn blocks, not "all mobs". The hook is asked also for sheep that eat
grass, foxes that take berries, allays that pick up items and villagers that farm. Those must still work in a claim.

Pistons: the doc of `piston_pre` does not say clearly whether `direction` is where the piston faces or where the
blocks move when it pulls. So the check takes the columns on both sides along the direction. It can refuse a piston
right next to another owner's claim that moves away from it.

Farmland: `farmland_trample` has no host, no dimension and only an entity number. So the rule is "nobody tramples
farmland in a claim", and a claim at the same column in any dimension counts.

These limits are in the README ("Limits of the Horizon hooks").

### Messages in the player's language

`src/lang.rs` has one table: a key and its text in en, ru and uk. Texts can have places for values (`{owner}`,
`{blocks}`). `client_information_updated` saves each player's language in memory (`uk_ua`, `ru_ru`, other). The mod
sends `Text::literal` in that language. A server mod has no lang files on the client, so `Text::translatable`
would show the key.

Rejected: lang files in a client resource pack. Then every player needs the mod's client part.

## Risks / Trade-offs

- `show_block` blocks can stay on a client until their chunk is sent again, if the player logs out first. The real
  blocks come back on the next login. This is the same as in GriefPrevention.
- A WebAssembly mod pays for each host call. The outside checks run for every fluid flow and fire spread.
  Mitigation: the check first looks up the claim at the target (an index by chunk, no host call). It reads the
  source only when the target is in a claim.
- `profile_by_name` knows only players who were on this server. A player cannot trust a friend who has never
  joined. That is the same as in GriefPrevention.

## Migration Plan

`claims.json` version 1 is read as version 2. The first save after a change writes version 2. Rollback to 0.1.0
reads version 2 as an error and starts with no claims. The file is not overwritten until a claim changes. So, to
go back, the owner restores the file from a backup.

## Open Questions

- Whether `show_block` and `register_commands` need a capability in `horizon-mod.toml` besides `registries`. The
  first build in game answers this.
