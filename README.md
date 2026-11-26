# Grief Protection

Grief Protection gives a [Horizon](https://playhorizon.dev) server (Minecraft Java Edition 26.3 in Rust) land
claims. It is inspired by [GriefPrevention](https://github.com/GriefPrevention/GriefPrevention): a player claims
land, and in a claim only the players its owner trusts can build, open containers, use doors or hurt animals.

**Status: a draft.** The claims, the trust levels, the rules and the protection are here. The commands that make
claims, the claim tool and the messages with names come next.

## Claims

A claim is a rectangle of whole columns in one dimension, from the bottom of the world to the top. Claims do
not overlap. A player may claim as many columns as they have claim blocks: they start with some and get more
for each hour they play, up to a limit; an admin can give more.

The owner trusts other players, each at a level. Each level allows what the levels below it allow:

| Level | Allows | In GriefPrevention |
|---|---|---|
| Access | doors, trapdoors, gates, buttons, levers, pressure plates, beds | `/accesstrust` |
| Container | chests and every block that holds something, animals, item frames | `/containertrust` |
| Build | breaking and placing blocks | `/trust` |
| Manager | trusting and untrusting other players | `/permissiontrust` |

An admin (an operator of level 2 or more; later the LitePerms permission `griefprotection.admin`) can do
everything in every claim. Monsters can be hit everywhere.

## Settings and data

- The settings are in `grief_protection.toml` (for each world in `serverconfig/`, else in `config/`): the claim
  blocks a new player has, how many they get each hour, the most they get, and the smallest claim.
- The claims of a world are in `<world>/horizon-mod-data/grief_protection/claims.json`, so they are copied and
  backed up with the world.

## Build

You need a checkout of `craftorio/horizon` next to this one, as `../horizon`. `make hooks` once, then
`make check`. A release is a tag `v<version>` (the version in `horizon-mod.toml`): the release workflow builds
`grief_protection.hmod` and attaches it to the release. It is a server mod: players do not need it.

## License

MIT, see [LICENSE](LICENSE). Grief Protection is new code written in Rust. It does not use the code of
GriefPrevention.
