# claim-tool Specification

## Purpose
The golden shovel and the stick: how a player makes, changes and sees a claim, and the borders that only that
player sees.

## Requirements

### Requirement: Make a claim with the golden shovel

A right click on a block with a golden shovel in the main hand SHALL mark a corner. The first click marks the first
corner. A second click in the same dimension marks the opposite corner and makes the claim, if the claim is allowed.
A click with the shovel does not use the clicked block.

#### Scenario: Two corners make a claim
- **WHEN** a player with enough claim blocks clicks (0, 64, 0) and then (20, 70, 15) with the golden shovel
- **THEN** a claim from x 0 to 20 and z 0 to 15 is made
- **AND** the player sees its borders

#### Scenario: A click on a chest
- **WHEN** a player clicks a chest with the golden shovel
- **THEN** the chest does not open
- **AND** the chest is a corner

#### Scenario: A refused claim
- **WHEN** the second corner makes a claim that overlaps another claim
- **THEN** no claim is made
- **AND** the player is told why, and sees the borders of the other claim

#### Scenario: Another dimension
- **WHEN** a player marks the first corner in the overworld and then goes to the nether
- **THEN** the first corner is forgotten

### Requirement: Change the size of a claim with the golden shovel

A click with the golden shovel on a corner of a claim SHALL start a resize, if the player owns the claim or is an
admin. The next click is the new place of that corner. The opposite corner stays. The changed claim has the same
checks as a new claim (size, overlap, claim blocks). Its old area counts as free.

#### Scenario: A larger claim
- **WHEN** Alex owns a claim from (0, 0) to (10, 10), clicks the corner (10, 10), and then clicks (20, 20)
- **THEN** the claim goes from (0, 0) to (20, 20)
- **AND** it uses 441 claim blocks

#### Scenario: A corner of another player's claim
- **WHEN** Steve clicks a corner of Alex's claim with the golden shovel
- **THEN** no resize starts
- **AND** Steve sees the borders of Alex's claim and is told who owns it

### Requirement: See a claim with a stick

A right click on a block with a stick in the main hand SHALL show the borders of the claim at that block, and tell
who owns it. If no claim is there, the player is told that the land is not claimed.

#### Scenario: A claimed block
- **WHEN** a player clicks a block of Alex's claim with a stick
- **THEN** the player sees the borders of that claim
- **AND** the player is told that Alex owns it

### Requirement: Borders that only one player sees

The borders of a claim SHALL show as blocks that only the player who asked sees: the corners, and a block every
10 blocks along each side, on the top block of each column. The borders of a valid claim and of a refused claim
SHALL look different. The world does not change. The real blocks come back after 60 seconds, or when the player sees
other borders.

#### Scenario: Another player does not see the borders
- **WHEN** Alex sees the borders of his claim
- **THEN** Steve, next to him, sees the real blocks

#### Scenario: The borders go away
- **WHEN** 60 seconds go by after the borders showed
- **THEN** the player sees the real blocks again
