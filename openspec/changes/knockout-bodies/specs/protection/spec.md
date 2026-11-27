## ADDED Requirements

### Requirement: A Knockout body opens for its owner and for container trust

A Knockout body is an entity with the tag `knockout_body`. Its owner is the UUID in its tag `knockout_owner_<uuid>`.
In a claim, only the owner of the body, the players with container trust in the claim, and admins SHALL open the
body. Grief Protection SHALL refuse other players and tell them so on their action bar. Outside claims, Grief
Protection SHALL NOT refuse a click on a body.

#### Scenario: The owner opens their body in another player's claim
- **WHEN** Steve dies in Alex's claim, Alex has not trusted Steve, and Steve clicks his body
- **THEN** the body opens

#### Scenario: A stranger clicks a body in a claim
- **WHEN** Steve's body lies in Alex's claim and Bob, without trust in it, clicks the body
- **THEN** the body does not open
- **AND** Bob is told that the body is not his and that the land is claimed by Alex

#### Scenario: A player with container trust
- **WHEN** Steve's body lies in Alex's claim and Alex gave Bob container trust there, and Bob clicks the body
- **THEN** the body opens

#### Scenario: A body outside claims
- **WHEN** Steve's body lies on unclaimed land and Bob clicks it
- **THEN** the body opens

### Requirement: A skeleton body opens for every player

A skeleton body (the tag `knockout_skeleton`, or a body entity of the type `minecraft:skeleton`) SHALL open for
every player, everywhere (the owner's decision, 2026-11-27).

#### Scenario: A skeleton body in a claim
- **WHEN** Steve's body in Alex's claim became a skeleton and Bob, without trust in it, clicks the skeleton
- **THEN** the skeleton opens
