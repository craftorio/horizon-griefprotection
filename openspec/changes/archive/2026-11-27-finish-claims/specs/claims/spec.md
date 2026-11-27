## ADDED Requirements

### Requirement: A claim covers whole columns

A claim SHALL be a rectangle of block columns in one dimension, from the bottom of the world to the top. It has one
owner. Two claims SHALL NOT overlap.

#### Scenario: A block above or below the claim
- **WHEN** a claim covers x 0 to 9 and z 0 to 9 in the overworld
- **THEN** the blocks at (5, -60, 5) and (5, 300, 5) are in the claim
- **AND** the block at (5, 64, 5) in the nether is not in the claim

#### Scenario: An overlap is refused
- **WHEN** a player makes a claim that shares one column with another claim
- **THEN** the claim is not made
- **AND** the player is told that the land is claimed already

### Requirement: Claims are large enough

A new or changed claim SHALL be at least `min_width` blocks on each side and at least `min_area` columns, from
`grief_protection.toml`.

#### Scenario: A narrow claim
- **WHEN** `min_width` is 5 and a player makes a claim 4 blocks wide
- **THEN** the claim is not made
- **AND** the player is told the smallest width and area

### Requirement: Claim blocks

A player SHALL claim no more columns than their claim blocks. The claim blocks are: `initial_blocks`, and
`blocks_per_hour` for each full hour of play (the vanilla `play_time` statistic), together no more than
`max_blocks`; and then the bonus that admins gave, added on top. Each claim uses as many claim blocks as it has
columns. When a claim is removed, its blocks come back.

#### Scenario: A new player
- **WHEN** a player who played less than one hour has no claims, and `initial_blocks` is 600 (the default: the
  owner's decision, 2026-11-27)
- **THEN** the player can claim 600 columns

#### Scenario: Not enough claim blocks
- **WHEN** a player has 40 claim blocks left and makes a claim of 100 columns
- **THEN** the claim is not made
- **AND** the player is told that the claim needs 100 blocks and they have 40

#### Scenario: The bonus stays after a restart
- **WHEN** an admin runs `/adjustbonusclaimblocks Alex 500`, and the server restarts
- **THEN** Alex still has the 500 bonus blocks

### Requirement: Trust levels

The owner SHALL trust other players in a claim at one of four levels: access, container, build, manager. Each level
allows what the levels below it allow. The owner has every level. An admin (an operator of level 2 or more) may do
everything in every claim.

#### Scenario: Build trust allows containers
- **WHEN** Steve has build trust in Alex's claim
- **THEN** Steve can break blocks, open chests and use doors in it

#### Scenario: Access trust does not allow chests
- **WHEN** Steve has access trust in Alex's claim
- **THEN** Steve can open the doors in it
- **AND** Steve cannot open the chests in it

### Requirement: The settings are read from the settings file

The mod SHALL read `grief_protection.toml` (for each world in `serverconfig/`, else in `config/`) when the world
loads and when the settings are reloaded. A file that is not there is made with the default values.

#### Scenario: A changed setting
- **WHEN** the owner sets `initial_blocks = 1000` and reloads
- **THEN** a new player can claim 1000 columns

### Requirement: The claims are saved with the world

The claims, the trust and the bonus blocks SHALL be saved in `<world>/horizon-mod-data/grief_protection/claims.json`.
The mod SHALL read a file of format version 1 (from 0.1.0) without a loss. If the file cannot be read, the mod
starts with no claims, logs the error, and does not write over the file until a claim changes.

#### Scenario: A 0.1.0 file
- **WHEN** the server starts with a `claims.json` of version 1
- **THEN** all its claims and their trust are in place
- **AND** every player has a bonus of 0

### Requirement: Make a claim with a command

`/claim [radius]` SHALL claim a square whose centre is the player's column. The side is `2 * radius + 1` blocks. With
no radius, it is the smallest such square that `min_width` and `min_area` allow.

#### Scenario: A claim without a radius
- **WHEN** `min_width` is 5, `min_area` is 100, and a player at x 0, z 0 runs `/claim`
- **THEN** a claim from x -5 to 5 and z -5 to 5 is made

#### Scenario: A claim with a radius
- **WHEN** a player at x 100, z 200 runs `/claim 5`
- **THEN** a claim from x 95 to 105 and z 195 to 205 is made
- **AND** the player sees its borders

### Requirement: Remove claims

`/abandonclaim` SHALL remove the claim where the player stands, if the player owns it or is an admin.
`/abandonallclaims` SHALL remove all claims of the player after the player runs it a second time within 30 seconds.

#### Scenario: Not the owner
- **WHEN** Steve stands in Alex's claim and runs `/abandonclaim`
- **THEN** the claim stays
- **AND** Steve is told that only its owner can remove it

#### Scenario: All claims need a confirmation
- **WHEN** a player runs `/abandonallclaims` once
- **THEN** no claim is removed
- **AND** the player is told to run it again to confirm

### Requirement: Trust and untrust with commands

`/accesstrust`, `/containertrust`, `/trust` and `/permissiontrust <player>` SHALL give that level, and `/untrust
<player>` SHALL take away any level. In a claim, they change that claim, and the player needs manager trust in it.
Only the owner or an admin can give manager trust. Outside a claim, they change every claim that the player owns. The
name can be of a player who is not online, if that player was on the server once.

#### Scenario: Trust outside a claim
- **WHEN** Alex owns two claims, stands outside them, and runs `/trust Steve`
- **THEN** Steve has build trust in both claims

#### Scenario: A manager cannot make managers
- **WHEN** Steve has manager trust in Alex's claim and runs `/permissiontrust Bob` in it
- **THEN** Bob gets no trust
- **AND** Steve is told that only the owner can give this level

#### Scenario: An unknown name
- **WHEN** a player runs `/trust Nobody` and no player of that name was ever on the server
- **THEN** nothing changes
- **AND** the player is told that no player with this name was on this server

### Requirement: List claims and trust

`/trustlist` SHALL show the owner and the trusted players of the claim where the player stands, by level.
`/claimslist` SHALL show the player's claims (dimension, corners, area) and their claim blocks (earned, bonus, used,
left). `/claimslist <player>` SHALL show the same for another player, to admins only.

#### Scenario: A player lists their claims
- **WHEN** a player with one claim of 121 columns and 300 claim blocks runs `/claimslist`
- **THEN** they see the claim and "121 used, 179 left"

### Requirement: Admins give bonus blocks

`/adjustbonusclaimblocks <player> <amount>` SHALL add `amount` (it can be negative) to the player's bonus. Only an
admin or the console can run it. The player does not need to be online.

#### Scenario: A player runs the admin command
- **WHEN** a player who is not an admin runs `/adjustbonusclaimblocks Alex 100`
- **THEN** the command is refused as a command of a higher permission level

### Requirement: Messages in the player's language

Every message of the mod to a player SHALL be in Ukrainian, Russian or English. The language is the language of the
player's game. For any other language, the message is in English. The player does not need the mod on their client.

#### Scenario: A Ukrainian player
- **WHEN** a player whose game is in Ukrainian tries to open a chest in a claim without trust
- **THEN** the refusal on their action bar is in Ukrainian
