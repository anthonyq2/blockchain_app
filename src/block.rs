use crate::transaction::Transaction;

use super::*;

#[derive(Debug)]
pub struct Block {
    pub timestamp: u64,
    pub hash: String,
    pub pre_hash: String, // previous Block hash
    pub transaction: Vec<Transaction>,
}

impl Block {
    ///
    /// Creates block from previous block hash and transaction data
    ///
    pub fn new(pre_hash: String, transaction: Vec<Transaction>) -> Self {
        let time = now();
        Block {
            timestamp: time,
            hash: "".to_owned(), // TODO - Replace with actual hash
            pre_hash,
            transaction,
        }
    }
}