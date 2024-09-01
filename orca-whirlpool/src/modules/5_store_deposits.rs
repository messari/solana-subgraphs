use substreams::skip_empty_output;
use substreams::store::{StoreNew, StoreSetIfNotExists, StoreSetIfNotExistsProto};

use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::{Deposit, Deposits};

#[substreams::handlers::store]
pub fn store_deposits(pool_deposits: Deposits, store: StoreSetIfNotExistsProto<Deposit>) {
    skip_empty_output();

    for deposit in pool_deposits.data {
        store.set_if_not_exists(0, StoreKey::Deposit.get_unique_key(&deposit.id), &deposit);
    }
}
