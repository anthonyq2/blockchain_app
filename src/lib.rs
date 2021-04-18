use std::time::{SystemTime, UNIX_EPOCH};

use transaction::Transaction;
use sha2::{Sha256, Digest};

mod block;
pub mod transaction;

///
/// Returns the time elapsed since the Unix Epoch.
///
pub fn now() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Invalid duration!");

    since_the_epoch.as_secs()
}

/// Returns a block hash calculated from the provided values.
///
/// # Arguments
/// * `pre_hash` - String reference of the previous Block hash.
/// * `transactions` - Vec<Transaction> reference containing the Transaction data for the block.
/// * `timestamp` - The timestamp in seconds for the Block.
///
pub fn calculate_hash(pre_hash: &String,
    transactions: &Vec<Transaction>, timestamp: u64) -> String {
    let mut bytes = vec![];
    bytes.extend(&timestamp.to_ne_bytes());

    // Flatten all transactions data into a single Vec<u8>
    bytes.extend(transactions
        .iter()
        .flat_map(|transaction| transaction.bytes())
        .collect::<Vec<u8>>(),
    );
    bytes.extend(pre_hash.as_bytes());

    let result = Sha256::new().chain(bytes).finalize();
    String::from_utf8_lossy(&result[..]).into_owned()
}