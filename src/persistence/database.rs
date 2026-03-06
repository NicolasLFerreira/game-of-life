use crate::types::ConfigurationChainNode;
use crate::utilities::bit_packing::unpack_u64_u32;
use std::path::PathBuf;

/// Wrapper for `sled::Db`
pub struct DatabaseConnection {
    db: sled::Db,
}

impl DatabaseConnection {
    pub fn open() -> Self {
        Self {
            db: sled::open(db_path()).unwrap(),
        }
    }

    pub fn get(&self, hash: u128) -> Option<ConfigurationChainNode> {
        let key = hash.to_be_bytes().to_vec();
        self.db
            .get(key)
            .unwrap()
            .map(|v| postcard::from_bytes(&v).unwrap())
    }

    pub fn insert(&self, canonical_configuration: &ConfigurationChainNode) {
        let key = canonical_configuration.hash.to_be_bytes();
        let value = postcard::to_allocvec(&canonical_configuration).unwrap();
        self.db.insert(key, value).unwrap();
    }

    pub fn contains(&self, hash: u128) -> bool {
        let key = hash.to_be_bytes().to_vec();
        self.db.contains_key(key).unwrap()
    }

    pub fn evolution_line(&self, hash: u128) -> Vec<u128> {
        let mut hashes: Vec<u128> = vec![];
        let mut cur_hash = hash;

        while let Some(v) = self.get(cur_hash) {
            hashes.push(v.hash);
            cur_hash = v.next_hash;
        }

        hashes
    }

    pub fn value_dump(&self) -> Vec<ConfigurationChainNode> {
        let mut dump: Vec<ConfigurationChainNode> = Vec::new();

        for entity in self.db.iter() {
            let (_, v) = entity.unwrap();
            let value: ConfigurationChainNode = postcard::from_bytes(&v).unwrap();
            dump.push(value);
        }

        dump
    }

    pub fn hash_dump(&self) -> Vec<u128> {
        let mut dump: Vec<u128> = Vec::new();

        for entity in self.db.iter() {
            let (key, _) = entity.unwrap();
            let hash = u128::from_be_bytes(key.as_ref().try_into().unwrap());
            dump.push(hash);
        }

        dump
    }
}

pub fn db_path() -> PathBuf {
    PathBuf::from("./data")
}
