use crate::types::cell_configuration::CellConfiguration;

/// Computes the canonical representation of a given cell configuration, independent of rotation and mirroring.
/// * `cells` - The original, absolute, non-normalized configuration
pub fn compute_canonical(cells: &CellConfiguration) -> Vec<(u32, u32)> {
    // Converts to vec for easier handling
    let cells: Vec<_> = cells.iter().collect();

    // FIRST STEP: FINDING BOUNDING BOX
    // VERY important, needs to be able to find the minimum rectangle that encompasses the alive cells.
    // This has to be in local space, so subtracting all values by min x and y.
    let mut min_x: i32 = i32::MAX;
    let mut min_y: i32 = i32::MAX;

    for ccoord in cells.iter() {
        let x = ccoord.x;
        let y = ccoord.y;

        // min
        if x < min_x {
            min_x = x
        }
        if y < min_y {
            min_y = y
        }
    }

    // Root configuration for all future rotations/mirror
    let mut normalized: Vec<(u32, u32)> = Vec::new();

    let mut max_x: u32 = 0;
    let mut max_y: u32 = 0;

    // applies normalization
    for ccoord in cells.iter() {
        let new_x = (ccoord.x - min_x) as u32;
        let new_y = (ccoord.y - min_y) as u32;

        // Builds bounding box dimensions
        if max_x < new_x {
            max_x = new_x;
        }
        if max_y < new_y {
            max_y = new_y;
        }

        normalized.push((new_x, new_y));
    }

    // Perform all transformations
    let normalized = normalized;
    let normalized_m: Vec<(u32, u32)> = normalized
        .iter()
        .map(|(x, y)| mirror_h(max_x, *x, *y))
        .collect();

    let transformations: Vec<Vec<(u32, u32)>> = vec![
        normalized.clone(),
        normalized_m.clone(),
        normalized.iter().map(|(x, y)| r90(max_x, *x, *y)).collect(),
        normalized
            .iter()
            .map(|(x, y)| r180(max_x, max_y, *x, *y))
            .collect(),
        normalized
            .iter()
            .map(|(x, y)| r270(max_y, *x, *y))
            .collect(),
        normalized_m
            .iter()
            .map(|(x, y)| r90(max_x, *x, *y))
            .collect(),
        normalized_m
            .iter()
            .map(|(x, y)| r180(max_x, max_y, *x, *y))
            .collect(),
        normalized_m
            .iter()
            .map(|(x, y)| r270(max_y, *x, *y))
            .collect(),
    ];

    let mut ordered: Vec<Vec<(u32, u32)>> = vec![];
    for mut t in transformations {
        t.sort_unstable();
        ordered.push(t);
    }

    let canonical = ordered.iter().min().unwrap();

    canonical.to_vec()
}

// Canonicalization rules:
// rotation and mirror have to be calculated
// 4 rotations: 0, 90, 180, 270
// 2 mirrors: 1, -1

// Rotation functions:
// For M = width, N = height
//
//   0: (x, y)  ->  (    x, y    )
//  90: (x, y)  ->  (    y, M-1-x)
// 180: (x, y)  ->  (M-1-x, N-1-y)
// 270: (x, y)  ->  (N-1-y, x    )
//
// In code, height and width will be subtracted 1 to avoid extra computation inside the functions

#[inline]
fn r90(mx: u32, x: u32, y: u32) -> (u32, u32) {
    (y, mx - x)
}

#[inline]
fn r180(mx: u32, my: u32, x: u32, y: u32) -> (u32, u32) {
    (mx - x, my - y)
}

#[inline]
fn r270(my: u32, x: u32, y: u32) -> (u32, u32) {
    (my - y, x)
}

// Mirroring function
// Mirrors along the horizontal axis,
// which makes it a function of the width and x
#[inline]
fn mirror_h(mx: u32, x: u32, y: u32) -> (u32, u32) {
    (mx - x, y)
}
