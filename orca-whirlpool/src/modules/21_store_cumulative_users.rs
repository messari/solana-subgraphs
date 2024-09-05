use substreams::skip_empty_output;
use substreams::store::{DeltaBigInt, Deltas, StoreAddInt64};
use substreams::store::{StoreAdd, StoreNew};

use crate::key_store::StoreKey;

#[substreams::handlers::store]
pub fn store_cumulative_users(unique_users_delta: Deltas<DeltaBigInt>, store: StoreAddInt64) {
    skip_empty_output();

    for _ in unique_users_delta.deltas {
        store.add(0, StoreKey::CumulativeUsers.unique_id(), 1)
    }
}
