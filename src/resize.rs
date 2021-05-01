use std::collections::HashSet;
use std::iter;

use crate::*;
use cassowary::strength::{REQUIRED, STRONG};
use cassowary::WeightedRelation::*;
use cassowary::{Constraint, Solver, Variable};

#[derive(Debug, Copy, Clone)]
struct FlexItem {
    pos: Variable,
    size: Variable,
}

pub fn resize_horizontally(width: f32, panes: &[Pane]) -> Vec<Pane> {
    let boundaries = row_boundaries(panes);
    let panes: Vec<(&Pane, FlexItem)> = panes
        .into_iter()
        .zip(iter::repeat_with(|| FlexItem {
            pos: Variable::new(),
            size: Variable::new(),
        }))
        .collect();
    let mut rows: Vec<Vec<_>> = Vec::new();
    for boundary in boundaries {
        rows.push(panes_in_row(panes.clone(), boundary));
    }
    let constraints = rows
        .iter()
        .fold(HashSet::new(), |c, p| &c | &constrain_row(width, p.clone()));
    //let constraints = constrain_row(width, constrained_panes.first().unwrap());
    let mut solver = Solver::new();
    solver.add_constraints(constraints.iter()).unwrap();
    let mut new_panes = Vec::new();
    for row in rows {
        for (pane, fi) in row {
            let mut pane = pane.to_owned();
            pane.pos.0 = solver.get_value(fi.pos) as f32;
            pane.size.0 = solver.get_value(fi.size) as f32;
            new_panes.push(pane);
        }
    }
    new_panes
}

fn row_boundaries(panes: &[Pane]) -> Vec<(f32, f32)> {
    let mut top = 0.0;
    let mut bounds = Vec::new();
    loop {
        let mut panes_in_row: Vec<_> = panes
            .iter()
            .filter(|&p| (p.pos.1 - top).abs() < GAP)
            .collect();
        // sort_unstable_by_key in the real thing
        panes_in_row.sort_unstable_by(|&a, &b| a.size.1.partial_cmp(&b.size.1).unwrap());
        if let Some(&bottom) = panes_in_row.first() {
            let bottom = top + bottom.size.1;
            bounds.push((top, bottom));
            top = bottom + GAP;
        } else {
            break;
        }
    }
    bounds
}

fn panes_in_row(panes: Vec<(&Pane, FlexItem)>, boundary: (f32, f32)) -> Vec<(&Pane, FlexItem)> {
    let (top, bot) = boundary;
    let bwn = |v| top <= v && v <= bot;
    let mut row: Vec<_> = panes
        .into_iter()
        .filter(|(p, _)| bwn(p.pos.1) || bwn(p.pos.1 + p.size.1))
        .collect();
    // sort_unstable_by_key in the real thing
    row.sort_unstable_by(|(a, _), (b, _)| a.pos.0.partial_cmp(&b.pos.0).unwrap());
    row //.iter().map(|p| p.id).collect()
}

fn constrain_row(width: f32, row: Vec<(&Pane, FlexItem)>) -> HashSet<Constraint> {
    let mut constraints = HashSet::new();
    // The first pane needs to start at x = 0
    constraints.insert(row[0].1.pos | EQ(REQUIRED) | 0.0);

    let gap_space = GAP * (row.len() - 1) as f32;
    let old_flex_space = row
        .iter()
        .fold(0.0, |w, (p, _)| if p.flex { w + p.size.0 } else { w });
    // Could also be used to calculate `old` if I had the old width handy
    let new_flex_space = row.iter().fold(
        width - gap_space,
        |w, (p, _)| if !p.flex { w - p.size.0 } else { w },
    );

    // Keep panes stuck together
    // FIXME: Rubbish, make panes implement Copy
    for fi in row.iter().map(|(_, fi)| fi).collect::<Vec<_>>().windows(2) {
        let (lfi, rfi) = (fi[0], fi[1]);
        constraints.insert((lfi.pos + lfi.size + GAP) | EQ(REQUIRED) | rfi.pos);
    }

    // Try to maintain ratios and lock non-flexible sizes
    for (pane, fi) in &row {
        if pane.flex {
            let ratio = pane.size.0 / old_flex_space;
            constraints.insert((fi.size / new_flex_space) | EQ(STRONG) | ratio);
        } else {
            constraints.insert(fi.size | EQ(REQUIRED) | pane.size.0);
        }
    }

    // The last pane needs to end at width
    let (_, last) = row.last().unwrap();
    constraints.insert((last.pos + last.size) | EQ(REQUIRED) | width);

    constraints
}
