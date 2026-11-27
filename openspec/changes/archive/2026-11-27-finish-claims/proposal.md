## Why

Grief Protection 0.1.0 is a draft. It has the claims, the trust levels and the protection of building,
containers, doors and animals. But a player cannot make a claim: there are no commands and no claim tool, and the
mod does not read its settings. Also, fire, fluids, pistons and explosions from outside a claim can still damage
it. The owner wants the mod finished, so that it can go on the Horizon server (the owner's decision, 2026-11-27).

## What Changes

- The mod reads `grief_protection.toml` (`register_configs`, `config_loading`, `config_reloading`).
- Claim blocks come from the vanilla `play_time` statistic and from a bonus that an admin gives. The bonus is
  saved with the claims. `claims.json` goes to format version 2. The mod still reads version 1.
- Commands, as in GriefPrevention:
  - `/claim [radius]`: claim a square around the player.
  - `/abandonclaim`, `/abandonallclaims`: remove the claim where the player stands, or all of their claims. The
    claim blocks come back.
  - `/trust`, `/containertrust`, `/accesstrust`, `/permissiontrust <player>`, `/untrust <player>`: in the claim
    where the player stands, or in all of their claims when they stand outside a claim.
  - `/trustlist`: the trusted players of the claim where the player stands.
  - `/claimslist [player]`: the claims of a player, and their claim blocks (earned, used, left). Another player's
    list is for admins only.
  - `/adjustbonusclaimblocks <player> <amount>`: for admins only.
- The claim tool, as in GriefPrevention:
  - A golden shovel. A right click on a block marks the first corner, a second click marks the opposite corner and
    makes the claim.
  - A click on a corner of the player's own claim, and then on a new place, changes the size of that claim.
  - A stick shows the claim at the block that the player clicks.
  - The borders of a claim show as blocks that only that player sees (`show_block`). The blocks go away after
    60 seconds, or when the next borders show.
- Protection from outside a claim:
  - fluids that flow in,
  - fire that spreads in or burns a block,
  - pistons that push or pull blocks across a border,
  - explosions (the blocks in a claim stay; the blocks outside break as in vanilla),
  - mobs that change blocks (an enderman takes a block, a creeper or a ghast explodes),
  - buckets (empty or fill) and farmland that a player tramples,
  - item frames and armor stands (they need container trust).
- Messages to players are in Ukrainian, Russian or English. The server takes the player's language from their
  client (the `client_information_updated` hook) and uses English for other languages. The mod stays a server
  mod: players do not need it (the owner's decision, 2026-11-27: messages in uk, ru and en).
- The version goes to 0.2.0.

## Non-goals

- The admin permission from LitePerms (`griefprotection.admin`). Admins are still operators of level 2 or more.
  This comes in a later change, when a WebAssembly mod can ask the native LitePerms.
- Subdivisions (claims inside a claim), admin claims, claim expiry for players who do not play, selling claim
  blocks, siege mode, PvP rules.
- A map or a client mod that shows claims.
- Moving the claims from any old server: the Horizon world is new.

## Capabilities

### New Capabilities

- `claims`: a claim, its trust levels, claim blocks, and the commands that make, list and remove claims and trust
  players.
- `claim-tool`: the golden shovel, the stick, and the borders that a player sees.
- `protection`: what a claim protects against, from the players inside it and from outside it.

### Modified Capabilities

None. `openspec/specs/` is empty. This change also writes what 0.1.0 already does into the new specs.

## Impact

- Code: `src/lib.rs` (new hooks), `src/game.rs`, new `src/commands.rs`, `src/tool.rs`, `src/outside.rs`,
  `src/lang.rs`, `src/store.rs` (format 2), `src/claims/` (resize, the bonus, tests). No file is longer than
  400 lines.
- `horizon-mod.toml`: version 0.2.0. The mod needs no new capability if commands and `show_block` work with
  `registries` only. If they do not, the task adds the capability that Horizon asks for.
- The Horizon server (`craftorio/horizon-server`, `mods.toml`) gets the new release.
