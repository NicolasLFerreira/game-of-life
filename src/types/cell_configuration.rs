use crate::types::cell_coord::CellCoord;
use std::collections::HashSet;

pub struct CellConfiguration {
    internal_cells: HashSet<CellCoord>,
}

// Instantiation
impl CellConfiguration {
    pub fn new() -> Self {
        Self {
            internal_cells: HashSet::new(),
        }
    }
}

// Crud stuff
impl CellConfiguration {
    pub fn get(&self, coord: CellCoord) -> bool {
        self.internal_cells.contains(&coord)
    }

    pub fn spawn(&mut self, coord: CellCoord) {
        self.internal_cells.insert(coord);
    }

    pub fn despawn(&mut self, coord: CellCoord) {
        self.internal_cells.remove(&coord);
    }

    pub fn iter(&self) -> impl Iterator<Item = &CellCoord> {
        self.internal_cells.iter()
    }
}
