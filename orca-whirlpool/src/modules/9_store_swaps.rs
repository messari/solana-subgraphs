use substreams::skip_empty_output;
use substreams::store::{StoreNew, StoreSetIfNotExists, StoreSetIfNotExistsProto};

use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::{Swap, Swaps};

#[substreams::handlers::store]
pub fn store_swaps(pool_swaps: Swaps, store: StoreSetIfNotExistsProto<Swap>) {
    skip_empty_output();

    for swap in pool_swaps.data {
        store.set_if_not_exists(0, StoreKey::Swap.get_unique_key(&swap.id), &swap);
    }
}
