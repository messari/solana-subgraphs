use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams::skip_empty_output;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreDelete, StoreNew};

use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::{Deposits, Swaps, Withdraws};

#[substreams::handlers::store]
pub fn store_activity_counts(
    clock: Clock,
    pool_deposits: Deposits,
    pool_withdraws: Withdraws,
    pool_swaps: Swaps,
    store: StoreAddBigInt,
) {
    skip_empty_output();

    let day_id = clock.timestamp.unwrap().seconds / 86400;

    // Delete previous day's snapshot
    store.delete_prefix(
        0,
        &StoreKey::UsageMetricsDailySnapshot(day_id - 1, None).unique_id(),
    );

    // Update the counts for the day, and the total transaction count.
    let update_counts = |count_type: StoreKey| {
        store.add_many(
            0,
            &vec![
                StoreKey::UsageMetricsDailySnapshot(day_id, Some(Box::new(count_type))).unique_id(),
                StoreKey::UsageMetricsDailySnapshot(day_id, Some(Box::new(StoreKey::TxnCount)))
                    .unique_id(),
            ],
            &BigInt::one(),
        );
    };

    pool_deposits
        .data
        .iter()
        .for_each(|_| update_counts(StoreKey::DepositCount));
    pool_withdraws
        .data
        .iter()
        .for_each(|_| update_counts(StoreKey::WithdrawCount));
    pool_swaps
        .data
        .iter()
        .for_each(|_| update_counts(StoreKey::SwapCount));
}
