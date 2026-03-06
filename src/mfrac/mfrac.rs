use crate::mfrac::canonical::compute_canonical;
use crate::mfrac::hash::canonical_hash;
use crate::persistence::DatabaseConnection;
use crate::types::{
    CellConfiguration, ConfigurationChainNode, MfracOutcome, MfracStatus, MfracTerminationReason,
};

/// Heart of Thanatos
pub struct Mfrac {
    db: DatabaseConnection,
    cache: Vec<(u128, Vec<u64>)>,
    last_hash: u128,
}

impl Mfrac {
    pub fn init() -> Self {
        Self {
            db: DatabaseConnection::open(),
            cache: Default::default(),
            last_hash: 0,
        }
    }

    pub fn run_pipeline(&mut self, configuration: &CellConfiguration) -> MfracStatus {
        let canonical = compute_canonical(configuration);
        let hash = canonical_hash(&canonical);

        if hash == self.last_hash {
            return Some(MfracOutcome::Termination(
                MfracTerminationReason::CanonicalStabilisation,
            ));
        }

        self.last_hash = hash;

        // collision detection
        let collision = self.db.contains(hash);

        match collision {
            // YAY
            true => {
                self.collapse(hash);
                Some(MfracOutcome::Collision(hash))
            }
            // aw
            false => {
                self.cache.push((hash, canonical));
                None
            }
        }
    }

    pub fn force_collapse(&mut self) {
        self.collapse(self.last_hash);
    }

    fn collapse(&mut self, attractor_hash: u128) {
        let mut next_hash: u128 = attractor_hash;
        while let Some((hash, configuration)) = self.cache.pop() {
            self.db.insert(&ConfigurationChainNode {
                hash,
                configuration,
                next_hash,
            });
            next_hash = hash;
        }
    }
}
