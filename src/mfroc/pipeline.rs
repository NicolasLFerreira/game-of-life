use crate::mfroc::canonical::compute_canonical;
use crate::types::cell_configuration::CellConfiguration;
use wyhash::wyhash;

pub fn process_mfroc(configuration: &CellConfiguration) {
    let canonical = compute_canonical(configuration);
    let hash = compute_hash(&canonical);
}

pub fn compute_hash(canonical: &Vec<(u32, u32)>) -> u128 {
    // seeding for hash halves
    const SEED_1: u64 = 2;
    const SEED_2: u64 = 3;

    // flattens and converts u32->u8
    let mut bytes: Vec<u8> = vec![0; canonical.len() * 8];
    for (i, (x, y)) in canonical.iter().enumerate() {
        bytes[i * 8..i * 8 + 4].copy_from_slice(&x.to_le_bytes());
        bytes[i * 8 + 4..i * 8 + 8].copy_from_slice(&y.to_le_bytes());
    }

    // two-step u64 hash generation
    let h1 = wyhash(&bytes, SEED_1);
    let h2 = wyhash(&bytes, SEED_2);

    // final 128-bit hash
    ((h1 as u128) << 64) | (h2 as u128)
}
