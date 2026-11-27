## Why

Knockout (`craftorio/horizon-knockout`) leaves a body when a player dies. The body holds the player's items. It is
a `minecraft:mannequin`. After an hour it becomes a skeleton, a `minecraft:skeleton`. A right click opens the body.

Today Grief Protection treats the mannequin as any other entity that is not a monster. So in a claim only players
with container trust open it. The owner of the body cannot open their own body when the claim's owner does not
trust them. This is a bug.

Knockout does not know claims. Its README says what Grief Protection must add. Knockout listens to
`entity_interact_specific` at the priority -100, so Grief Protection (the priority 0) decides first. When Grief
Protection cancels the click, Knockout does not open the body.

## What Changes

The owner's decisions, 2026-11-27:

- A Knockout body is an entity with the tag `knockout_body`. It also has the tags `knockout_body_<id>` and
  `knockout_owner_<uuid>` (the owner's UUID as 32 hex digits).
- In a claim, only the owner of the body (the UUID in the tag) and the players that the claim trusts with
  containers may open the body. An admin may open it too, as everything in a claim. Other players are refused, and
  a message on their action bar tells them why.
- Outside claims, every player may open a body. Grief Protection does nothing there.
- A skeleton body may be opened by every player, everywhere. Grief Protection does not protect it. Knockout will
  mark a skeleton with a tag (probably `knockout_skeleton`). Until that tag is confirmed, a body entity of the type
  `minecraft:skeleton` is a skeleton too. The tag name is in one constant.

In the code:

- A new module `src/body.rs` reads the tags and the type, without the game, and says what a click on the entity
  needs. Tests in `src/tests.rs`.
- `protect::refuse_use` (the hooks `entity_interact` and `entity_interact_specific`) asks `body.rs` first.
- A new message `NotYourBody` in en, ru and uk.
- The version goes to 0.3.0.

## Non-goals

- No change to Knockout. Grief Protection reads only the tags that Knockout sets.
- No new trust level and no command for bodies.
- No protection of a body from damage: Knockout makes the body take no damage.
- No change to who may hit or use other monsters.

## Impact

- `src/body.rs` (new), `src/game/protect.rs`, `src/lang.rs`, `src/lib.rs`, `src/tests.rs`.
- `README.md`, `AGENTS.md`: the rule for bodies.
- The spec `protection`: a new requirement.
