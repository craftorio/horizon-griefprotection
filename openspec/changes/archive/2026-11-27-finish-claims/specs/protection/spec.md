## ADDED Requirements

### Requirement: Building needs build trust

In a claim, a player without build trust SHALL NOT break or place a block, empty or fill a bucket, or light a
fire. The player is told so on their action bar (a fire is refused without a message).

#### Scenario: Breaking a block without trust
- **WHEN** Steve has no trust in Alex's claim and breaks a block in it
- **THEN** the block stays
- **AND** Steve is told that the land is claimed by Alex

#### Scenario: A bucket of lava
- **WHEN** Steve has container trust in Alex's claim and empties a lava bucket in it
- **THEN** no lava is placed

### Requirement: Containers and animals need container trust

In a claim, a player without container trust SHALL NOT open a block that holds something (also a composter, a flower
pot, a cake), hurt or use an animal, or use an item frame or an armor stand.

#### Scenario: An item frame
- **WHEN** Steve has access trust in Alex's claim and takes the item out of an item frame in it
- **THEN** the item stays in the frame

### Requirement: Doors and switches need access trust

In a claim, a player without access trust SHALL NOT use a door, a trapdoor, a gate, a button, a lever, a pressure
plate, a repeater, redstone wire, a bed, a bell, a note block or a jukebox (the owner's decision, 2026-11-27: a
stranger may not change the delay of a repeater).

#### Scenario: A door
- **WHEN** Steve has no trust in Alex's claim and clicks a door in it
- **THEN** the door does not open

#### Scenario: A repeater
- **WHEN** Steve has no trust in Alex's claim and clicks a repeater in it
- **THEN** the delay of the repeater does not change

### Requirement: Monsters and players can be hit everywhere

A player SHALL be able to hit monsters and other players in every claim, with no trust.

#### Scenario: A monster
- **WHEN** Steve has no trust in Alex's claim and hits a zombie in it
- **THEN** the zombie takes damage

### Requirement: Nothing from outside changes a claim

A fluid, a fire, a piston or a dispenser whose source is outside a claim SHALL NOT change a block in that claim. "Outside"
means: in no claim, or in a claim of another owner. A source in a claim of the same owner may change the block. No
explosion SHALL break a block in a claim, wherever it starts. No block of a claim SHALL burn away.

#### Scenario: Lava flows to a claim
- **WHEN** lava flows from unclaimed land toward a claim
- **THEN** the lava stops at the border of the claim

#### Scenario: Water between two claims of one owner
- **WHEN** water flows from one of Alex's claims into another claim of Alex
- **THEN** the water flows as in vanilla

#### Scenario: Fire spreads to a claim
- **WHEN** a fire next to a claim spreads
- **THEN** no block of the claim catches fire or burns away

#### Scenario: A piston at the border
- **WHEN** a piston outside a claim pushes blocks into the claim, or pulls a block out of it
- **THEN** the piston does not move

#### Scenario: A dispenser at the border
- **WHEN** a dispenser outside a claim faces the claim and gets a signal, with a lava bucket in it
- **THEN** no lava is placed in the claim
- **AND** the bucket stays in the dispenser

#### Scenario: TNT next to a claim
- **WHEN** TNT explodes outside a claim, near its border
- **THEN** the blocks outside the claim break as in vanilla
- **AND** no block in the claim breaks

### Requirement: Mobs do not change blocks in a claim

The mobs that break, take or burn blocks (endermen, ravagers, silverfish, withers, zombies, creepers, ghasts,
blazes, evokers, the dragon) SHALL NOT change blocks in a claim. Other mobs SHALL change the world as in vanilla.

#### Scenario: An enderman in a claim
- **WHEN** an enderman in a claim tries to take a block
- **THEN** the block stays

#### Scenario: A creeper in a claim
- **WHEN** a creeper explodes in a claim
- **THEN** no block in the claim breaks
- **AND** the players near it take damage

#### Scenario: A sheep and a villager farmer
- **WHEN** a sheep eats grass, and a villager harvests and plants wheat, in a claim
- **THEN** they do as in vanilla

### Requirement: Farmland in a claim is not trampled

Nobody SHALL trample farmland in a claim: no player, the owner too, and no mob.

#### Scenario: A jump on farmland
- **WHEN** Steve jumps on farmland in Alex's claim
- **THEN** the farmland stays
