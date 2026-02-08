use crate::{Coord, Grid};
use std::collections::HashMap;

pub fn simulation(cells: &mut Grid) {
    let current_state = cells.clone();
    let mut neighbours: HashMap<Coord, u8> = HashMap::new();

    // init alive cells with 0 neighbours
    for coord in current_state.iter() {
        neighbours.insert(*coord, 0);
    }

    // compute neighbours
    for (x, y) in current_state.iter() {
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let key = (x + dx, y + dy);
                let entry = neighbours.entry(key).or_insert(0);
                *entry += 1;
            }
        }
    }

    // apply survival rules
    for (coord, count) in neighbours.iter() {
        if current_state.contains(coord) {
            if *count != 2 && *count != 3 {
                cells.remove(coord);
            }
        } else {
            if *count == 3 {
                cells.insert(*coord);
            }
        }
    }
}
