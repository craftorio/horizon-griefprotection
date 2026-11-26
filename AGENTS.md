# AGENTS.md

Grief Protection (`grief_protection`): land claims for Horizon servers (Minecraft 26.3 in Rust, our fork
`craftorio/horizon`), inspired by GriefPrevention. A WebAssembly server mod (`side = "server"`). It is a draft.

## Layout

No Rust file is longer than 400 lines (`scripts/max-lines.sh`, part of `make check`).

- `src/claims/`: the claims and their rules, without the game: `mod.rs` (a claim, the trust levels),
  `index.rs` (all claims with an index by chunk; adding refuses overlaps, small claims and missing claim blocks),
  `rules.rs` (who may do what), `blocks.rs` (the claim blocks a player earned). Tested in `tests.rs`.
- `src/kind.rs`: what a right click on a block is (access or container).
- `src/store.rs`: `claims.json` in the world's data folder (format version 1).
- `src/config.rs`: `grief_protection.toml`.
- `src/game.rs`: the hooks' way to the rules (the claims of the world in place, the acting player, the refusal
  on the action bar). `src/lib.rs`: the hooks only.

## Next

1. Read `grief_protection.toml` (`register_configs`, `config_loading`).
2. Commands: `/claim [radius]`, `/abandonclaim`, `/trust`, `/containertrust`, `/accesstrust`,
   `/permissiontrust`, `/untrust`, `/claimslist`, `/adjustbonusclaimblocks` (admins).
3. The claim tool (a golden shovel: two corners), and showing a claim's corners.
4. The admin permission from LitePerms (`craftorio/horizon-liteperms`) when the two can link.
5. Fluids, fire, pistons and explosions that enter a claim from outside.

## Building and shipping

As for `craftorio/horizon-carry`: the mod API is `../horizon/crates/horizon-mod-api`, the WebAssembly build is
made inside a Horizon checkout (`.github/workflows/release.yml`, or craftorio-launcher's `make horizon-mods`), and
a release is a tag `v<version>`. The package holds only `Cargo.toml`, `horizon-mod.toml` and `src/`.

## Rules

- All documentation is in Simplified English (in the spirit of ASD-STE100): `AGENTS.md`, `README.md`, doc
  comments and comments in code, commit messages. Short sentences, one idea in each. Common words, and one word
  for one thing. Active voice. No idioms.
- Test a change of the rules in `src/claims/tests.rs`.
- Checks before finishing: `make check`. `make hooks` once per clone: `pre-commit` runs the checks,
  `commit-msg` rejects a `Co-authored-by` trailer.
- Commit only when asked. Never add `Co-Authored-By` (or any other AI attribution line) to commit messages.
