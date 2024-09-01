use substreams::skip_empty_output;
use substreams::store::{StoreNew, StoreSetIfNotExists, StoreSetIfNotExistsProto};

use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::{Withdraw, Withdraws};

#[substreams::handlers::store]
pub fn store_withdraws(pool_withdraws: Withdraws, store: StoreSetIfNotExistsProto<Withdraw>) {
    skip_empty_output();

    for withdraw in pool_withdraws.data {
        store.set_if_not_exists(
            0,
            StoreKey::Withdraw.get_unique_key(&withdraw.id),
            &withdraw,
        );
    }
}
