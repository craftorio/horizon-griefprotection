# Grief Protection

Grief Protection gives a [Horizon](https://playhorizon.dev) server (Minecraft Java Edition 26.3 in Rust) land
claims. It is inspired by [GriefPrevention](https://github.com/GriefPrevention/GriefPrevention): a player claims
land, and in a claim only the players its owner trusts can build, open containers, use doors or hurt animals.
Fluids, fire, pistons, explosions and mobs do not change a claim from outside.

It is a server mod: players do not need it. Its messages are in English, Russian and Ukrainian: each player gets
the language of their game (English for other languages). A refused `show_block` (the borders) is written to the
server's log.

## Claims

A claim is a rectangle of whole columns in one dimension, from the bottom of the world to the top. Claims do
not overlap. A player may claim as many columns as they have claim blocks: they start with some and get more
for each hour they play (the vanilla `play_time` statistic), up to a limit; an admin can give more.

The owner trusts other players, each at a level. Each level allows what the levels below it allow:

| Level | Allows | Command |
|---|---|---|
| Access | doors, trapdoors, gates, buttons, levers, pressure plates, repeaters, redstone wire, beds, bells, note blocks, jukeboxes | `/accesstrust <player>` |
| Container | chests and every block that holds something (also composters, flower pots, cakes), animals, villagers, item frames, armor stands | `/containertrust <player>` |
| Build | breaking and placing blocks, buckets, fire | `/trust <player>` |
| Manager | trusting and untrusting other players | `/permissiontrust <player>` |

An admin (an operator of level 2 or more; later the LitePerms permission `griefprotection.admin`) can do
everything in every claim. Monsters and players can be hit everywhere.

## Making claims

- **The golden shovel.** Right-click a block: the first corner. Right-click the opposite corner: the claim. To
  change the size of a claim, right-click one of its corners, then the new place of that corner.
- **The stick.** Right-click a block: you see the claim there and who owns it.
- **`/claim [radius]`**: a square around you, `2 * radius + 1` blocks on a side. Without a radius: the smallest
  square that the settings allow.

The first corner shows as a pillar: glowstone in the place of the top block, and end rods 64 blocks up. You see the borders of a
claim as such a pillar over each corner, and gold blocks every 10 blocks along the sides (a redstone block and
netherrack when a claim cannot be made, or for the claim in the way). The rods take only the place of air.
Only you see them, for 60 seconds. The world does not change.

The messages in chat have colours, as in GriefDefender: a refusal is red, a thing done is green, information is
aqua, and the names and numbers in them have their own colour. `/abandonallclaims` asks for a confirmation with a
button to click.

## Commands

| Command | Who | What it does |
|---|---|---|
| `/claim [radius]` | players | claims a square around the player |
| `/abandonclaim` | the owner, admins | removes the claim where the player stands; its claim blocks come back |
| `/abandonallclaims` | players | removes all claims of the player (run it twice within 30 seconds) |
| `/accesstrust`, `/containertrust`, `/trust`, `/permissiontrust <player>` | managers | trusts a player in the claim where you stand; outside a claim, in all your claims |
| `/untrust <player>` | managers | takes the trust away, in the same way |
| `/trustlist` | players | the owner and the trusted players of the claim where you stand |
| `/claimslist` | players | your claims and claim blocks (earned, bonus, used, left) |
| `/claimslist <player>` | admins, the console | the same for another player |
| `/adjustbonusclaimblocks <player> <amount>` | admins, the console | adds bonus claim blocks (the amount can be negative) |

Only the owner (or an admin) can give manager trust. A player name works for players who are not online, if they
were on the server once.

## Protection from outside

- A fluid or a fire does not spread into a claim from land of another owner, or from unclaimed land.
- No block of a claim burns away.
- Fire that starts without a player (lava, lightning) does not start in a claim.
- A piston does not move blocks into or out of a claim of another owner.
- A dispenser does not put out its item (lava, water, fire, a block) into a claim of another owner: the item stays,
  and the dispenser clicks as when it is empty.
- Explosions do not break blocks in claims. Entities still take damage.
- Endermen, ravagers, silverfish, withers, zombies (doors), creepers, ghasts, blazes, evokers and the dragon do not
  change blocks in a claim. Other mobs (sheep that eat grass, villagers that farm) do as in vanilla.
- Nobody tramples farmland in a claim.

Limits of the Horizon hooks:

- `farmland_trample` does not say the dimension or who lands. So a claim at the same column in any dimension keeps
  the farmland.
- `block_ignite` has no server. So it does not know admins: an admin needs build trust to light a fire in another
  player's claim.
- `piston_pre` checks the blocks next to the piston and next to each moved block on both sides along the direction.
  So a piston right next to the border of another owner's claim can be refused when it moves away from it.

## Settings and data

- The settings are in `grief_protection.toml` (for each world in `serverconfig/`, else in `config/`): the claim
  blocks a new player has, how many they get each hour, the most they get, and the smallest claim.
- The claims of a world and the bonus claim blocks are in `<world>/horizon-mod-data/grief_protection/claims.json`,
  so they are copied and backed up with the world. The mod writes format 2, and reads format 1 of version 0.1.0.

## Build

You need a checkout of `craftorio/horizon` next to this one, as `../horizon`. `make hooks` once, then
`make check`. A release is a tag `v<version>` (the version in `horizon-mod.toml`): the release workflow builds
`grief_protection.hmod` and attaches it to the release.

## License

MIT, see [LICENSE](LICENSE). Grief Protection is new code written in Rust. It does not use the code of
GriefPrevention.
