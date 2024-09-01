use substreams::scalar::BigInt;
use substreams::skip_empty_output;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreNew};

use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::{Deposits, Withdraws};

#[substreams::handlers::store]
pub fn store_pool_liquidity(
    pool_deposits: Deposits,
    pool_withdraws: Withdraws,
    store: StoreAddBigInt,
) {
    skip_empty_output();

    pool_deposits.data.iter().for_each(|deposit| {
        store.add(
            0,
            StoreKey::PoolLiquidity.get_unique_key(&deposit.to),
            &BigInt::try_from(deposit.output_amount.clone()).unwrap(),
        );
    });

    pool_withdraws.data.iter().for_each(|withdraw| {
        store.add(
            0,
            StoreKey::PoolLiquidity.get_unique_key(&withdraw.to),
            &BigInt::try_from(withdraw.output_amount.clone())
                .unwrap()
                .neg(),
        );
    });
}
