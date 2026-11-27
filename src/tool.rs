//! The claim tool, without the game: what a click with the golden shovel does, by the state of the player's tool,
//! and which columns show the borders of a claim.

use crate::claims::Claim;
use crate::claims::index::Claims;

/// The item that makes and changes claims.
pub const SHOVEL: &str = "minecraft:golden_shovel";
/// The item that shows the claim at a block.
pub const STICK: &str = "minecraft:stick";

/// The state of a player's golden shovel.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Tool {
    #[default]
    Idle,
    /// The first corner of a new claim.
    Corner { dimension: String, pos: [i32; 2] },
    /// The player clicked a corner of claim `claim`: the next click is the new place of that corner. `fixed` is
    /// the opposite corner, which stays.
    Resize {
        claim: u64,
        dimension: String,
        fixed: [i32; 2],
    },
}

/// What a click with the golden shovel does.
#[derive(Clone, Debug, PartialEq)]
pub enum Click {
    /// The first corner of a new claim is marked.
    FirstCorner,
    /// A resize of claim `claim` starts.
    ResizeStart { claim: u64 },
    /// Make a claim with these corners.
    Make { a: [i32; 2], b: [i32; 2] },
    /// Give claim `claim` these corners.
    Resize { claim: u64, a: [i32; 2], b: [i32; 2] },
    /// The click is in claim `claim`, but not on a corner the player may move: show the claim.
    Show { claim: u64 },
}

/// The corner of `claim` opposite to `pos`, when `pos` is one of its corners.
fn opposite_corner(claim: &Claim, pos: [i32; 2]) -> Option<[i32; 2]> {
    let x = if pos[0] == claim.min[0] {
        claim.max[0]
    } else if pos[0] == claim.max[0] {
        claim.min[0]
    } else {
        return None;
    };
    let z = if pos[1] == claim.min[1] {
        claim.max[1]
    } else if pos[1] == claim.max[1] {
        claim.min[1]
    } else {
        return None;
    };
    Some([x, z])
}

/// A click of `player` with the golden shovel on block column `pos` of `dimension`. `admin` may resize every
/// claim. Changes `tool` to its next state.
pub fn shovel(tool: &mut Tool, claims: &Claims, player: u128, admin: bool, dimension: &str, pos: [i32; 2]) -> Click {
    match std::mem::take(tool) {
        Tool::Resize {
            claim,
            dimension: there,
            fixed,
        } if there == dimension => {
            return Click::Resize {
                claim,
                a: fixed,
                b: pos,
            };
        }
        Tool::Corner {
            dimension: there,
            pos: first,
        } if there == dimension => {
            return Click::Make { a: first, b: pos };
        }
        _ => {}
    }
    let Some(claim) = claims.at(dimension, pos[0], pos[1]) else {
        *tool = Tool::Corner {
            dimension: dimension.to_owned(),
            pos,
        };
        return Click::FirstCorner;
    };
    match opposite_corner(claim, pos) {
        Some(fixed) if admin || claim.owner == player => {
            *tool = Tool::Resize {
                claim: claim.id,
                dimension: dimension.to_owned(),
                fixed,
            };
            Click::ResizeStart { claim: claim.id }
        }
        _ => Click::Show { claim: claim.id },
    }
}

/// A column of a claim's border, and whether it is a corner.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mark {
    pub column: [i32; 2],
    pub corner: bool,
}

/// The columns that show the border of the rectangle `min`..`max`: its corners, and a column every 10 blocks
/// along each side.
pub fn border(min: [i32; 2], max: [i32; 2]) -> Vec<Mark> {
    let mut marks: Vec<Mark> = Vec::new();
    for x in [min[0], max[0]] {
        for z in [min[1], max[1]] {
            if !marks.iter().any(|m| m.column == [x, z]) {
                marks.push(Mark {
                    column: [x, z],
                    corner: true,
                });
            }
        }
    }
    let side = |marks: &mut Vec<Mark>, column: [i32; 2]| {
        if !marks.iter().any(|m| m.column == column) {
            marks.push(Mark { column, corner: false });
        }
    };
    for x in (min[0] + 10..max[0]).step_by(10) {
        side(&mut marks, [x, min[1]]);
        side(&mut marks, [x, max[1]]);
    }
    for z in (min[1] + 10..max[1]).step_by(10) {
        side(&mut marks, [min[0], z]);
        side(&mut marks, [max[0], z]);
    }
    marks
}
