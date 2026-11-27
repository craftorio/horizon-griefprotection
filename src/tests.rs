//! The claim tool, the borders, and protection from outside, without the game.

use crate::claims::index::Claims;
use crate::claims::rectangle;
use crate::outside::{PistonMove, may_cross, piston_may_move, spared};
use crate::tool::{Click, Tool, border, shovel};

const ALEX: u128 = 1;
const STEVE: u128 = 2;
const WORLD: &str = "minecraft:overworld";

/// Alex owns x 0 to 10, z 0 to 10 and x 20 to 30, z 0 to 10; Steve owns x 40 to 50, z 0 to 10.
fn claims() -> Claims {
    let mut claims = Claims::default();
    for (owner, a, b) in [
        (ALEX, [0, 0], [10, 10]),
        (ALEX, [20, 0], [30, 10]),
        (STEVE, [40, 0], [50, 10]),
    ] {
        claims.add(rectangle(0, owner, WORLD, a, b, 0), 10_000, 5, 100).unwrap();
    }
    claims
}

#[test]
fn two_clicks_of_the_shovel_make_a_claim() {
    let claims = claims();
    let mut tool = Tool::Idle;
    assert_eq!(
        shovel(&mut tool, &claims, ALEX, false, WORLD, [100, 100]),
        Click::FirstCorner
    );
    assert_eq!(
        shovel(&mut tool, &claims, ALEX, false, WORLD, [120, 90]),
        Click::Make {
            a: [100, 100],
            b: [120, 90]
        }
    );
    assert_eq!(tool, Tool::Idle);
}

#[test]
fn the_first_corner_in_another_dimension_is_forgotten() {
    let claims = claims();
    let mut tool = Tool::Idle;
    shovel(&mut tool, &claims, ALEX, false, WORLD, [100, 100]);
    assert_eq!(
        shovel(&mut tool, &claims, ALEX, false, "minecraft:the_nether", [5, 5]),
        Click::FirstCorner
    );
}

#[test]
fn a_click_on_a_corner_of_your_claim_starts_a_resize() {
    let claims = claims();
    let mut tool = Tool::Idle;
    let id = claims.at(WORLD, 0, 0).unwrap().id;
    assert_eq!(
        shovel(&mut tool, &claims, ALEX, false, WORLD, [10, 10]),
        Click::ResizeStart { claim: id }
    );
    assert_eq!(
        shovel(&mut tool, &claims, ALEX, false, WORLD, [15, 12]),
        Click::Resize {
            claim: id,
            a: [0, 0],
            b: [15, 12]
        },
        "the opposite corner stays"
    );
}

#[test]
fn another_players_claim_is_only_shown_but_an_admin_can_resize_it() {
    let claims = claims();
    let id = claims.at(WORLD, 0, 0).unwrap().id;
    let mut tool = Tool::Idle;
    assert_eq!(
        shovel(&mut tool, &claims, STEVE, false, WORLD, [10, 10]),
        Click::Show { claim: id }
    );
    assert_eq!(tool, Tool::Idle);
    assert_eq!(
        shovel(&mut tool, &claims, STEVE, false, WORLD, [5, 5]),
        Click::Show { claim: id }
    );
    assert_eq!(
        shovel(&mut tool, &claims, STEVE, true, WORLD, [0, 10]),
        Click::ResizeStart { claim: id }
    );
}

#[test]
fn the_border_has_the_corners_and_a_mark_every_10_blocks() {
    let marks = border([0, 0], [25, 5]);
    let corners: Vec<[i32; 2]> = marks.iter().filter(|m| m.corner).map(|m| m.column).collect();
    assert_eq!(corners, vec![[0, 0], [0, 5], [25, 0], [25, 5]]);
    let sides: Vec<[i32; 2]> = marks.iter().filter(|m| !m.corner).map(|m| m.column).collect();
    assert_eq!(sides, vec![[10, 0], [10, 5], [20, 0], [20, 5]]);
    assert_eq!(border([3, 3], [3, 3]).len(), 1, "one column is one corner");
}

#[test]
fn nothing_crosses_into_a_claim_from_another_owners_land() {
    let claims = claims();
    assert!(!may_cross(&claims, WORLD, [-1, 5], [0, 5]), "from unclaimed land");
    assert!(!may_cross(&claims, WORLD, [40, 5], [10, 5]), "from Steve's claim");
    assert!(may_cross(&claims, WORLD, [5, 5], [25, 5]), "between two claims of Alex");
    assert!(may_cross(&claims, WORLD, [5, 5], [-1, 5]), "out of a claim");
    assert!(may_cross(&claims, WORLD, [100, 5], [101, 5]), "on unclaimed land");
    assert!(
        may_cross(&claims, "minecraft:the_nether", [-1, 5], [0, 5]),
        "another dimension"
    );
}

#[test]
fn a_piston_does_not_move_blocks_across_a_border() {
    let claims = claims();
    let push = |piston, direction, moved: &[[i32; 3]]| {
        let piston = PistonMove {
            piston,
            direction,
            moved,
            destroyed: &[],
        };
        piston_may_move(&claims, WORLD, piston)
    };
    // outside, pushing toward the claim at x 0
    assert!(!push([-3, 64, 5], [1, 0, 0], &[[-2, 64, 5], [-1, 64, 5]]));
    // outside, pulling a block out of the claim
    assert!(!push([-2, 64, 5], [-1, 0, 0], &[[0, 64, 5]]));
    // in the claim, with all its blocks in the claim
    assert!(push([5, 64, 5], [1, 0, 0], &[[6, 64, 5]]));
    // far from claims
    assert!(push([100, 64, 5], [1, 0, 0], &[[101, 64, 5]]));
    // straight up in a claim
    assert!(push([5, 64, 5], [0, 1, 0], &[[5, 65, 5]]));
}

#[test]
fn a_dispenser_outside_does_not_put_out_into_a_claim() {
    use crate::outside::dispenser_may_reach;
    let claims = claims();
    assert!(
        !dispenser_may_reach(&claims, WORLD, [-1, 64, 5], [1, 0, 0]),
        "from unclaimed land"
    );
    assert!(
        !dispenser_may_reach(&claims, WORLD, [11, 64, 5], [-1, 0, 0]),
        "from between two claims"
    );
    assert!(
        dispenser_may_reach(&claims, WORLD, [41, 64, 5], [1, 0, 0]),
        "in Steve's own claim"
    );
    assert!(
        dispenser_may_reach(&claims, WORLD, [5, 64, 5], [1, 0, 0]),
        "in Alex's claim"
    );
    assert!(
        dispenser_may_reach(&claims, WORLD, [-1, 64, 5], [0, -1, 0]),
        "down, outside"
    );
    assert!(
        dispenser_may_reach(&claims, WORLD, [-1, 64, 5], [-1, 0, 0]),
        "away from the claim"
    );
}

#[test]
fn explosions_spare_the_blocks_in_claims() {
    let claims = claims();
    assert!(spared(&claims, WORLD, [5, -40, 5]));
    assert!(!spared(&claims, WORLD, [-1, 64, 5]));
}

#[test]
fn farmland_in_a_claim_of_any_dimension_is_kept() {
    let claims = claims();
    assert!(claims.in_any_dimension(5, 5));
    assert!(!claims.in_any_dimension(15, 5));
}

#[test]
fn claim_without_a_radius_is_the_smallest_square_allowed() {
    use crate::game::commands::default_radius;
    assert_eq!(
        default_radius(5, 100),
        5,
        "side 11: 10 x 10 is 100, and the side is odd"
    );
    assert_eq!(default_radius(5, 25), 2, "side 5");
    assert_eq!(default_radius(8, 10), 4, "side 9");
}

mod bodies {
    use crate::body::{Open, open, owner};

    const MANNEQUIN: &str = "minecraft:mannequin";
    const STEVE: u128 = 0x1234_5678_9abc_def0_1234_5678_9abc_def0;
    const BOB: u128 = 7;

    /// The tags that Knockout gives the body of Steve, and `more`.
    fn tags(more: &[&str]) -> Vec<String> {
        let mut tags = vec![
            "knockout_body".to_owned(),
            "knockout_body_3".to_owned(),
            format!("knockout_owner_{STEVE:032x}"),
        ];
        tags.extend(more.iter().map(|t| (*t).to_owned()));
        tags
    }

    #[test]
    fn the_owner_opens_the_body() {
        assert_eq!(owner(&tags(&[])), Some(STEVE));
        assert_eq!(open(MANNEQUIN, &tags(&[]), STEVE), Open::Anyone);
    }

    #[test]
    fn another_player_needs_container_trust() {
        assert_eq!(open(MANNEQUIN, &tags(&[]), BOB), Open::NeedsTrust);
    }

    #[test]
    fn the_owner_tag_in_capitals_names_the_owner() {
        let tags = vec!["knockout_body".to_owned(), format!("knockout_owner_{STEVE:032X}")];
        assert_eq!(open(MANNEQUIN, &tags, STEVE), Open::Anyone);
    }

    #[test]
    fn a_body_without_an_owner_needs_container_trust() {
        let tags = vec!["knockout_body".to_owned(), "knockout_owner_xyz".to_owned()];
        assert_eq!(owner(&tags), None);
        assert_eq!(open(MANNEQUIN, &tags, STEVE), Open::NeedsTrust);
    }

    #[test]
    fn a_skeleton_body_opens_for_everybody() {
        assert_eq!(open("minecraft:skeleton", &tags(&[]), BOB), Open::Anyone);
        assert_eq!(open(MANNEQUIN, &tags(&["knockout_skeleton"]), BOB), Open::Anyone);
        assert_eq!(
            open("minecraft:skeleton", &["knockout_skeleton".to_owned()], BOB),
            Open::Anyone
        );
    }

    #[test]
    fn other_entities_are_not_bodies() {
        assert_eq!(open(MANNEQUIN, &[], BOB), Open::NotBody);
        assert_eq!(open("minecraft:skeleton", &[], BOB), Open::NotBody);
        assert_eq!(
            open("minecraft:cow", &["knockout_body_3".to_owned()], BOB),
            Open::NotBody
        );
    }
}
