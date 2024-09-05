use substreams::skip_empty_output;
use substreams::store::{StoreNew, StoreSetIfNotExists, StoreSetIfNotExistsProto};

use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::{Pool, Pools};

#[substreams::handlers::store]
pub fn store_pools(initialized_pools: Pools, store: StoreSetIfNotExistsProto<Pool>) {
    skip_empty_output();

    for pool in initialized_pools.data {
        store.set_if_not_exists(0, StoreKey::Pool.get_unique_key(&pool.address), &pool);
    }
}
