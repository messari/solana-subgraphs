use substreams::skip_empty_output;
use substreams::store::{DeltaProto, Deltas, StoreAddInt64};
use substreams::store::{StoreAdd, StoreNew};

use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::Pool;

#[substreams::handlers::store]
pub fn store_total_pool_count(pools_delta: Deltas<DeltaProto<Pool>>, store: StoreAddInt64) {
    skip_empty_output();

    for _ in pools_delta.deltas {
        store.add(0, StoreKey::TotalPoolCount.unique_id(), 1)
    }
}
