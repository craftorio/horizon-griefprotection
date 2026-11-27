## 1. Settings and data

- [x] 1.1 Read `grief_protection.toml` in `register_configs`, `config_loading` and `config_reloading` (as
  `craftorio/horizon-carry` does). Keep the settings with the world state in `game.rs`.
- [x] 1.2 `store.rs`: format version 2 with `players` (the bonus). Read version 1 as version 2. Tests for both.
- [x] 1.3 `claims/`: the bonus of a player; `Claims::resize` (the old area counts as free). Tests in `tests.rs`.
- [x] 1.4 Earned claim blocks from `player.statistics(host)` (`minecraft:custom` / `minecraft:play_time`). A test of
  the JSON read.

## 2. Messages

- [x] 2.1 `lang.rs`: the table of messages in en, ru and uk, with places for values. A test that every key has
  three texts.
- [x] 2.2 Keep each player's language from `client_information_updated`. Forget it at `player_logged_out`.
- [x] 2.3 Move the refusal in `game.rs` to `lang.rs`, with the owner's name (`profile_by_uuid`).

## 3. Commands

- [x] 3.1 `commands.rs`: register the usages in `register_commands`. Run them from `run_mod_command`.
- [x] 3.2 `/claim [radius]`.
- [x] 3.3 `/abandonclaim`, `/abandonallclaims` (with the confirmation in 30 seconds).
- [x] 3.4 `/accesstrust`, `/containertrust`, `/trust`, `/permissiontrust`, `/untrust` (names through
  `profile_by_name`; in a claim or in all claims of the player).
- [x] 3.5 `/trustlist`, `/claimslist [player]`.
- [x] 3.6 `/adjustbonusclaimblocks <player> <amount>` (level 2, the console too).

## 4. The claim tool

- [x] 4.1 `tool.rs`: the state of each player (idle, first corner, resize). Tests without the game.
- [x] 4.2 The golden shovel in `right_click_block`: corners, a new claim, a resize. Cancel the use of the clicked
  block.
- [x] 4.3 The stick: show the claim at the clicked block and its owner.
- [x] 4.4 Borders with `show_block`: the corners and a block every 10 blocks, two looks (valid, refused). Show them
  again every 5 seconds. Send the real blocks after 60 seconds or before the next borders.
- [x] 4.5 Forget the tool state on a change of dimension and on logout.

## 5. Protection from outside

- [x] 5.1 `outside.rs`: the rule "source and target in claims of different owners" without the game, with tests.
- [x] 5.2 Fluids (`block_spread`, which asks for fluids and fire).
- [x] 5.3 Fire (`block_spread`, `block_burn`); a player's fire (`block_ignite`) needs build trust.
- [x] 5.4 Pistons (`piston_pre`).
- [x] 5.5 Explosions (`explosion_detonate`): take the blocks in claims out of the list.
- [x] 5.6 Mobs (`entity_mob_griefing`, a list of the mobs that break blocks); farmland (`farmland_trample`).
- [x] 5.7 Buckets (`bucket_empty`, `fill_bucket`): build trust.
- [x] 5.8 Item frames and armor stands (`entity_interact`, `entity_interact_specific`, `attack_entity`): container
  trust.
- [x] 5.9 Write each case that a hook cannot do (no cancel, no source) in `AGENTS.md` and in the README.

## 6. Documents and release

- [x] 6.1 `README.md` and `AGENTS.md`: the commands, the tool, the protection; the status is no longer a draft.
  Simplified English.
- [x] 6.2 Version 0.2.0 in `horizon-mod.toml` and `Cargo.toml`.
- [x] 6.3 `make check` (fmt, clippy, tests, `scripts/max-lines.sh`: no file over 400 lines).
- [x] 6.4 `openspec validate --strict finish-claims`.
- [x] 6.5 Build the `.hmod` (craftorio-launcher's `make horizon-mods`). Put it on the Horizon dev server
  (`craftorio/horizon-server`, `mods.toml`). Add the capability that the server asks for, if it asks for one.
- [x] 6.6 Check in game on the dev server, with two accounts: `/claim`, the shovel (a new claim, a resize, a
  refusal), the stick, trust and untrust, `/claimslist`, `/adjustbonusclaimblocks`, lava and fire at a border, TNT
  and a creeper, a piston, messages in uk, ru and en. Say plainly what was checked in game, and by whom.
  Moved to the play test of our small team on the dev server: it fixes what it finds (the owner's decision,
  2026-11-27). Checked in game before it: `/claim`, the shovel, the borders (by the owner).
- [x] 6.7 The tag `v0.2.0` (the release workflow builds the `.hmod`), and `/opsx:archive`. The server builds the mod
  itself (`craftorio/horizon-server`, `scripts/build-server-mods.sh`).
