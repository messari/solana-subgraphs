use substreams::pb::substreams::store_delta::Operation;
use substreams::pb::substreams::Clock;
use substreams::skip_empty_output;
use substreams::store::{DeltaBigInt, Deltas, StoreAddInt64, StoreDelete};
use substreams::store::{StoreAdd, StoreNew};

use crate::key_store::StoreKey;

#[substreams::handlers::store]
pub fn store_cumulative_users(
    clock: Clock,
    unique_users_delta: Deltas<DeltaBigInt>,
    store: StoreAddInt64,
) {
    skip_empty_output();

    let day_id = clock.timestamp.unwrap().seconds / 86400;

    // Delete previous day's snapshot
    store.delete_prefix(
        0,
        &StoreKey::UsageMetricsDailySnapshot(day_id - 1, None).unique_id(),
    );
    for delta in unique_users_delta.deltas {
        let key = delta.get_key();

        if key.starts_with(&StoreKey::User.unique_id())
            && delta.get_operation() == Operation::Create
        {
            // Add to cumulative users
            store.add(0, StoreKey::CumulativeUsers.unique_id(), 1);
        } else {
            // Add to daily active users
            store.add(
                0,
                StoreKey::UsageMetricsDailySnapshot(day_id, Some(Box::new(StoreKey::ActiveUsers)))
                    .get_snapshot_key(None),
                1,
            );
        }
    }
}
