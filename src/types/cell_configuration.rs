use crate::types::cell_coord::CellCoord;
use crate::utilities::bit_packing::unpack_u64_u32;
use fxhash::FxBuildHasher;
use rand::prelude::StdRng;
use rand::{RngExt, SeedableRng};

type InternalCells = hashbrown::HashSet<CellCoord, FxBuildHasher>;

#[derive(Default, PartialEq, Eq)]
pub struct CellConfiguration {
    internal_cells: InternalCells,
}

// Init
impl CellConfiguration {
    pub fn new() -> Self {
        Self {
            internal_cells: InternalCells::default(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            internal_cells: InternalCells::with_capacity_and_hasher(
                capacity,
                FxBuildHasher::default(),
            ),
        }
    }

    pub fn from_soup(soup: Vec<CellCoord>) -> Self {
        Self {
            internal_cells: soup.into_iter().collect(),
        }
    }

    pub fn from_packed(packed: Vec<u64>) -> Self {
        let mut cconf = CellConfiguration::new();
        for bpc in packed {
            let (l, r) = unpack_u64_u32(bpc);
            cconf.spawn(CellCoord {
                x: l as i32,
                y: r as i32,
            });
        }

        cconf
    }

    pub fn from_tuple(tuple: Vec<(i32, i32)>) -> Self {
        let mut cconf = CellConfiguration::new();
        for pair in tuple {
            cconf.spawn(CellCoord {
                x: pair.0,
                y: pair.1
            })
        }

        cconf
    }
}

// Interaction
impl CellConfiguration {
    pub fn is_alive(&self, coord: CellCoord) -> bool {
        self.internal_cells.contains(&coord)
    }

    pub fn spawn(&mut self, coord: CellCoord) {
        self.internal_cells.insert(coord);
    }

    pub fn despawn(&mut self, coord: CellCoord) {
        self.internal_cells.remove(&coord);
    }

    pub fn iter(&self) -> impl Iterator<Item = CellCoord> {
        self.internal_cells.iter().copied()
    }

    pub fn len(&self) -> usize {
        self.internal_cells.len()
    }
}

// Utility
impl CellConfiguration {
    pub fn cook_soup(seed: u64, width: usize, height: usize, density: f64) -> Vec<CellCoord> {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut cells = Vec::new();

        for y in 0..height {
            for x in 0..width {
                if rng.random::<f64>() < density {
                    cells.push(CellCoord::new(x as i32, y as i32));
                }
            }
        }

        cells
    }
}

// Clone
impl Clone for CellConfiguration {
    fn clone(&self) -> Self {
        Self {
            internal_cells: self.internal_cells.clone(),
        }
    }
}
