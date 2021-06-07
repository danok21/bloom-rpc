use jsonrpc_core::serde::{Serialize, Serializer};
use ethereum_types::U256;

#[derive(Debug, PartialEq)]
pub enum SyncStatus {
    /// Info when syncing
    Info(SyncInfo),
    /// Not syncing
    None,
}

impl Serialize for SyncStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match *self {
            SyncStatus::Info(ref info) => info.serialize(serializer),
            SyncStatus::None => false.serialize(serializer),
        }
    }
}


/// Sync info
#[derive(Default, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SyncInfo {
    /// Starting block
    pub starting_block: U256,
    /// Current block
    pub current_block: U256,
    /// Highest block seen so far
    pub highest_block: U256,
}
