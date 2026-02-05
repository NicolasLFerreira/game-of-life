use crate::{Coord, Grid, GRID_HEIGHT, GRID_WIDTH};

pub fn seed(cells: &mut Grid, coords: Vec<Coord>) {
    for (y, x) in coords {
        cells[idx(y, x)] = true;
    }
}

pub fn shape_translate(shape: Vec<Vec<bool>>, ay: usize, ax: usize) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    for y in 0..shape.len() {
        for x in 0..shape[y].len() {
            if shape[y][x] {
                coords.push((y + ay, x + ax));
            }
        }
    }

    coords
}

// RULES:
// 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
// 2. Any live cell with two or three live neighbours lives on to the next generation.
// 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
// 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
pub fn simulation(cells: &mut Grid) {
    let current_state = cells.clone();

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let index = idx(y, x);
            let is_alive = current_state[index];
            let alive_neighbours = neighbours((y, x))
                .iter()
                .filter(|&&(ny, nx)| current_state[idx(ny, nx)])
                .count();

            // Applies rule
            cells[index] = match (is_alive, alive_neighbours) {
                (true, 2..=3) => true,
                (true, _) => false,
                (false, 3) => true,
                (false, _) => false,
            };
        }
    }
}

pub fn neighbours(coord: Coord) -> Vec<Coord> {
    let (y, x) = coord;
    let y = y as i32;
    let x = x as i32;

    let mut neighbours: Vec<Coord> = Vec::new();

    for dy in -1..=1 {
        for dx in -1..=1 {
            // Skip center
            if dy == 0 && dx == 0 {
                continue;
            }

            // Bound check
            let ny = y + dy;
            let nx = x + dx;
            if ny < 0 || ny >= GRID_HEIGHT as i32 {
                continue;
            }
            if nx < 0 || nx >= GRID_WIDTH as i32 {
                continue;
            }

            neighbours.push((ny as usize, nx as usize))
        }
    }

    neighbours
}

#[inline]
// Row major
pub fn idx(y: usize, x: usize) -> usize {
    y * GRID_WIDTH + x
}