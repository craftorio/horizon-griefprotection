//! Claims: their shape, the index, the rules and the claim blocks.

use super::blocks::earned;
use super::index::{Claims, Limits, Refusal};
use super::rules::{Action, Refused, allowed};
use super::{Trust, rectangle};
use crate::config::Settings;

const OWNER: u128 = 1;
const FRIEND: u128 = 2;
const STRANGER: u128 = 3;

#[test]
fn a_claim_covers_whole_columns_between_its_corners() {
    let claim = rectangle(1, OWNER, "minecraft:overworld", [10, -5], [0, 5], 0);
    assert_eq!(
        (claim.min, claim.max, claim.area(), claim.width()),
        ([0, -5], [10, 5], 121, 11)
    );
    assert!(claim.contains("minecraft:overworld", 0, 5) && claim.contains("minecraft:overworld", 10, -5));
    assert!(!claim.contains("minecraft:overworld", 11, 0) && !claim.contains("minecraft:the_nether", 5, 0));
}

#[test]
fn the_index_finds_a_claim_across_chunks_and_forgets_a_removed_one() {
    let mut claims = Claims::default();
    let id = claims
        .add(
            rectangle(0, OWNER, "minecraft:overworld", [-20, -20], [20, 20], 0),
            10_000,
            5,
            100,
        )
        .unwrap();
    for (x, z) in [(-20, -20), (0, 0), (20, 20), (15, -17)] {
        assert_eq!(
            claims.at("minecraft:overworld", x, z).map(|c| c.id),
            Some(id),
            "{x} {z}"
        );
    }
    assert!(claims.at("minecraft:overworld", 21, 0).is_none());
    claims.remove(id);
    assert!(claims.at("minecraft:overworld", 0, 0).is_none());
}

#[test]
fn a_claim_is_refused_when_it_overlaps_is_small_or_costs_too_much() {
    let mut claims = Claims::default();
    let first = claims
        .add(
            rectangle(0, OWNER, "minecraft:overworld", [0, 0], [9, 9], 0),
            1000,
            5,
            100,
        )
        .unwrap();
    let overlap = rectangle(0, STRANGER, "minecraft:overworld", [9, 9], [20, 20], 0);
    assert_eq!(claims.add(overlap.clone(), 1000, 5, 100), Err(Refusal::Overlaps(first)));
    // the same place in another dimension is free
    assert!(
        claims
            .add(
                rectangle(0, STRANGER, "minecraft:the_nether", [9, 9], [20, 20], 0),
                1000,
                5,
                100
            )
            .is_ok()
    );
    let thin = rectangle(0, STRANGER, "minecraft:overworld", [100, 0], [103, 100], 0);
    assert_eq!(
        claims.add(thin, 1000, 5, 100),
        Err(Refusal::TooSmall {
            min_width: 5,
            min_area: 100
        })
    );
    // the owner used 100 of 150 blocks
    let big = rectangle(0, OWNER, "minecraft:overworld", [100, 0], [109, 9], 0);
    assert_eq!(
        claims.add(big, 150, 5, 100),
        Err(Refusal::NotEnoughBlocks { needed: 100, left: 50 })
    );
}

#[test]
fn each_trust_level_allows_the_levels_below() {
    let mut claim = rectangle(1, OWNER, "minecraft:overworld", [0, 0], [9, 9], 0);
    claim.trusted.insert(FRIEND, Trust::Container);
    let may = |player, action| allowed(&claim, player, action, false).is_ok();
    assert!(may(OWNER, Action::Manage) && may(OWNER, Action::Build));
    assert!(may(FRIEND, Action::Access) && may(FRIEND, Action::Container));
    assert!(!may(FRIEND, Action::Build) && !may(FRIEND, Action::Manage));
    assert_eq!(
        allowed(&claim, STRANGER, Action::Access, false),
        Err(Refused { owner: OWNER })
    );
    assert!(
        allowed(&claim, STRANGER, Action::Build, true).is_ok(),
        "an admin may do everything"
    );
}

#[test]
fn claim_blocks_grow_with_the_hours_played_up_to_the_limit() {
    let settings = Settings::default();
    let hour = 20 * 3600;
    assert_eq!(earned(&settings, 0, 0), 600);
    assert_eq!(earned(&settings, 3 * hour + hour / 2, 0), 900);
    assert_eq!(earned(&settings, 10_000 * hour, 0), 80_000);
    assert_eq!(
        earned(&settings, 10_000 * hour, 500),
        80_500,
        "an admin's bonus is over the limit"
    );
}

#[test]
fn claims_and_bonus_blocks_are_saved_and_read_back() {
    let mut claim = rectangle(4, OWNER, "minecraft:overworld", [0, 0], [9, 9], 1_795_700_000);
    claim.trusted.insert(u128::MAX - 1, Trust::Build);
    let mut claims = Claims::from_list(vec![claim.clone()]);
    claims.add_bonus(FRIEND, 500);
    let text = crate::store::to_text(&claims);
    let read = crate::store::from_text(&text).unwrap();
    assert_eq!(
        (read.list(), read.bonus(FRIEND), read.bonus(OWNER)),
        (vec![claim], 500, 0)
    );
    assert!(crate::store::from_text(r#"{"version": 9, "claims": []}"#).is_err());
}

#[test]
fn a_claims_file_of_version_1_is_read_with_no_bonus() {
    let text = format!(
        r#"{{"version": 1, "claims": [{{"id": 3, "owner": {OWNER}, "dimension": "minecraft:overworld",
            "min": [0, 0], "max": [9, 9], "trusted": {{"{FRIEND}": "access"}}}}]}}"#
    );
    let claims = crate::store::from_text(&text).unwrap();
    let claim = claims.at("minecraft:overworld", 5, 5).unwrap();
    assert_eq!(
        (claim.id, claim.trust_of(FRIEND), claims.bonus(OWNER)),
        (3, Some(Trust::Access), 0)
    );
    assert_eq!(claims.next_id(), 4);
}

#[test]
fn the_bonus_adds_and_takes_away() {
    let mut claims = Claims::default();
    assert_eq!(claims.add_bonus(OWNER, 300), 300);
    assert_eq!(claims.add_bonus(OWNER, -500), -200);
    assert_eq!(claims.add_bonus(OWNER, 200), 0);
    assert!(claims.bonuses().is_empty(), "a bonus of 0 is not kept");
}

#[test]
fn a_resize_counts_the_old_area_as_free_and_keeps_the_checks() {
    let mut claims = Claims::default();
    let id = claims
        .add(
            rectangle(0, OWNER, "minecraft:overworld", [0, 0], [10, 10], 0),
            500,
            5,
            100,
        )
        .unwrap();
    let other = claims
        .add(
            rectangle(0, STRANGER, "minecraft:overworld", [40, 0], [50, 10], 0),
            500,
            5,
            100,
        )
        .unwrap();
    let limits = |earned| Limits {
        earned,
        min_width: 5,
        min_area: 100,
    };
    // 121 used of 500: a claim of 441 fits, because its own 121 are free
    assert_eq!(claims.resize(id, [0, 0], [20, 20], limits(500)), Ok(()));
    assert_eq!(claims.used_by(OWNER), 441);
    assert!(claims.at("minecraft:overworld", 20, 20).is_some());
    assert_eq!(
        claims.resize(id, [0, 0], [30, 30], limits(500)),
        Err(Refusal::NotEnoughBlocks { needed: 961, left: 500 })
    );
    assert_eq!(
        claims.resize(id, [0, 0], [45, 10], limits(5000)),
        Err(Refusal::Overlaps(other))
    );
    assert_eq!(
        claims.resize(id, [0, 0], [3, 20], limits(500)),
        Err(Refusal::TooSmall {
            min_width: 5,
            min_area: 100
        })
    );
    assert_eq!(
        claims.get(id).map(|c| (c.min, c.max)),
        Some(([0, 0], [20, 20])),
        "a refused resize changes nothing"
    );
}

#[test]
fn the_play_time_comes_from_the_statistics() {
    use super::blocks::play_ticks;
    let json =
        r#"{"stats": {"minecraft:custom": {"minecraft:play_time": 144000, "minecraft:jump": 3}}, "DataVersion": 4000}"#;
    assert_eq!(play_ticks(json), 144_000);
    assert_eq!(play_ticks("{}"), 0);
    assert_eq!(play_ticks("not json"), 0);
}

#[test]
fn a_click_on_a_door_is_access_and_on_a_chest_is_container() {
    use crate::kind::click;
    assert_eq!(click("minecraft:oak_door", false), Some(Action::Access));
    assert_eq!(click("minecraft:stone_button", false), Some(Action::Access));
    assert_eq!(click("minecraft:lever", false), Some(Action::Access));
    assert_eq!(click("minecraft:chest", true), Some(Action::Container));
    assert_eq!(click("minecraft:repeater", false), Some(Action::Access));
    assert_eq!(click("minecraft:redstone_wire", false), Some(Action::Access));
    assert_eq!(click("minecraft:potted_poppy", false), Some(Action::Container));
    assert_eq!(click("minecraft:composter", false), Some(Action::Container));
    assert_eq!(click("minecraft:stone", false), None);
}
