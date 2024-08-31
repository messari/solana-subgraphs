use substreams::scalar::BigInt;
use substreams::store::DeltaBigInt;
use substreams::store::Deltas;
use substreams::store::StoreAdd;
use substreams::store::StoreAddBigInt;
use substreams::store::StoreNew;

use crate::key_store::StoreKey;

#[substreams::handlers::store]
pub fn store_cumulative_users(unique_users_delta: Deltas<DeltaBigInt>, store: StoreAddBigInt) {
    for _ in unique_users_delta.deltas {
        store.add(0, StoreKey::CumulativeUsers.unique_id(), BigInt::one())
    }
}
