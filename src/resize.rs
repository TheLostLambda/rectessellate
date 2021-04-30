use crate::*;

pub fn row_boundaries(panes: &[Pane]) -> Vec<(f32, f32)> {
    let mut top = 0.0;
    let mut bounds = Vec::new();
    loop {
        let mut panes_in_row: Vec<_> = panes
            .iter()
            .filter(|&p| (p.pos.1 - top).abs() < f32::EPSILON)
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
