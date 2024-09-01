use substreams::scalar::BigInt;
use substreams::skip_empty_output;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreNew};

use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::{Deposits, Swaps, Withdraws};

#[substreams::handlers::store]
pub fn store_pool_balances(
    pool_deposits: Deposits,
    pool_withdraws: Withdraws,
    pool_swaps: Swaps,
    store: StoreAddBigInt,
) {
    skip_empty_output();

    pool_deposits.data.iter().for_each(|deposit| {
        store.add(
            0,
            StoreKey::PoolBalance.get_unique_keys(&deposit.to, &deposit.token_a),
            &BigInt::try_from(deposit.amount_a.clone()).unwrap(),
        );
        store.add(
            0,
            StoreKey::PoolBalance.get_unique_keys(&deposit.to, &deposit.token_b),
            &BigInt::try_from(deposit.amount_b.clone()).unwrap(),
        );
    });

    pool_withdraws.data.iter().for_each(|withdraw| {
        store.add(
            0,
            StoreKey::PoolBalance.get_unique_keys(&withdraw.to, &withdraw.token_a),
            &BigInt::try_from(withdraw.amount_a.clone()).unwrap().neg(),
        );
        store.add(
            0,
            StoreKey::PoolBalance.get_unique_keys(&withdraw.to, &withdraw.token_b),
            &BigInt::try_from(withdraw.amount_b.clone()).unwrap().neg(),
        );
    });

    pool_swaps.data.iter().for_each(|swap| {
        store.add(
            0,
            StoreKey::PoolBalance.get_unique_keys(&swap.to, &swap.token_in),
            &BigInt::try_from(swap.amount_in.clone()).unwrap(),
        );
        store.add(
            0,
            StoreKey::PoolBalance.get_unique_keys(&swap.to, &swap.token_out),
            &BigInt::try_from(swap.amount_out.clone()).unwrap().neg(),
        );
    });
}
