use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams::skip_empty_output;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreDelete, StoreNew};

use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::Swaps;

#[substreams::handlers::store]
pub fn store_volume_by_token_amount(clock: Clock, pool_swaps: Swaps, store: StoreAddBigInt) {
    skip_empty_output();

    let day_id = clock.timestamp.unwrap().seconds / 86400;

    // Delete previous day's snapshot
    store.delete_prefix(
        0,
        &StoreKey::PoolDailySnapshot(day_id - 1, None).unique_id(),
    );

    pool_swaps.data.iter().for_each(|swap| {
        store.add(
            0,
            StoreKey::PoolDailySnapshot(day_id, Some(Box::new(StoreKey::DailyVolumeByTokenAmount)))
                .get_unique_keys(&swap.to, &swap.token_in),
            BigInt::try_from(&swap.amount_in).unwrap_or_default(),
        );

        store.add(
            0,
            StoreKey::PoolDailySnapshot(day_id, Some(Box::new(StoreKey::DailyVolumeByTokenAmount)))
                .get_unique_keys(&swap.to, &swap.token_out),
            BigInt::try_from(&swap.amount_out).unwrap_or_default(),
        );
    });
}
