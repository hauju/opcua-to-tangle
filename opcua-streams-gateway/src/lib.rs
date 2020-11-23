#![deny(
    bad_style,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features
)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
extern crate gateway_core;
pub mod tangle_connectivity;
pub mod opcua_connectivity;

use std::time::{SystemTime, UNIX_EPOCH};
fn timestamp_in_sec() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
