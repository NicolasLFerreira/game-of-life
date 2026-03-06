use crate::persistence::DatabaseConnection;
use crate::utilities::bit_packing::unpack_u64_u32;

pub struct StatisticsSearch {
    pub db: DatabaseConnection,
}

impl StatisticsSearch {
    pub fn new() -> Self {
        Self {
            db: DatabaseConnection::open(),
        }
    }
}
