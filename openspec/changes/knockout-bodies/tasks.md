## 1. The rule

- [x] 1.1 `src/body.rs`: the tags of Knockout as constants (the skeleton tag in one constant), and `open(kind, tags,
  player)`: not a body, open for everybody, or container trust needed. Without the game.
- [x] 1.2 Tests in `src/tests.rs`: the owner, a stranger, the owner tag in capitals, a body without an owner tag, a
  skeleton by its tag and by its type, an entity that is not a body.

## 2. The game

- [x] 2.1 `protect::refuse_use`: read the entity's tags (`EntityRef::tags`) and ask `body::open` before the check of
  monsters and players. Container trust for a body: refuse with the message `NotYourBody`.
- [x] 2.2 `lang.rs`: `NotYourBody` in en, ru and uk; a refusal (red).

## 3. Docs and the version

- [x] 3.1 `README.md` and `AGENTS.md`: the rule for bodies, and the new module.
- [x] 3.2 The version 0.3.0 in `Cargo.toml` and `horizon-mod.toml`.

## 4. Checks and shipping

- [x] 4.1 `make check`. `openspec validate --strict`.
- [x] 4.2 The WebAssembly build with horizon-server's `scripts/build-server-mods.sh`.
- [x] 4.3 In game, on a dedicated server with Knockout: a claim of A, a body of B in it; C without trust is refused;
  B opens the body; C with container trust opens it; the skeleton opens for C without trust; outside claims C
  opens the body. Done on 2026-11-27 (screenshots in `../horizon-work/screenshots/14-gp-bodies/`).
- [ ] 4.4 Ship as a tag `v0.3.0`; then the server takes the new `grief_protection.hmod`.
