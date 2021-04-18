use std::time::{SystemTime, UNIX_EPOCH};

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