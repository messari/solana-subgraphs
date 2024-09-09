use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams::skip_empty_output;
use substreams::store::{StoreDelete, StoreNew, StoreSetIfNotExists, StoreSetIfNotExistsBigInt};

use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::{event::Type, Events};

#[substreams::handlers::store]
pub fn store_unique_users(clock: Clock, raw_events: Events, store: StoreSetIfNotExistsBigInt) {
    skip_empty_output();

    let bigint1 = BigInt::one();
    let day_id = clock.timestamp.unwrap().seconds / 86400;

    // Delete previous day's snapshot
    store.delete_prefix(
        0,
        &StoreKey::UsageMetricsDailySnapshot(day_id - 1, None).unique_id(),
    );

    for event in raw_events.data {
        if let Some(user_address) = get_user_address(event.r#type.unwrap()) {
            set_user_activity(&store, day_id, &user_address, &bigint1);
        }
    }
}

fn get_user_address(event: Type) -> Option<String> {
    match event {
        // increase liquidity
        Type::IncreaseLiquidity(e) => Some(e.accounts.as_ref()?.position_authority.clone()),
        Type::IncreaseLiquidityV2(e) => Some(e.accounts.as_ref()?.position_authority.clone()),

        // decrease liquidity
        Type::DecreaseLiquidity(e) => Some(e.accounts.as_ref()?.position_authority.clone()),
        Type::DecreaseLiquidityV2(e) => Some(e.accounts.as_ref()?.position_authority.clone()),

        // two hop swap
        Type::TwoHopSwap(e) => Some(e.accounts.as_ref()?.token_authority.clone()),
        Type::TwoHopSwapV2(e) => Some(e.accounts.as_ref()?.token_authority.clone()),

        // swap
        Type::Swap(e) => Some(e.accounts.as_ref()?.token_authority.clone()),
        Type::SwapV2(e) => Some(e.accounts.as_ref()?.token_authority.clone()),
        _ => None,
    }
}

/// Sets user activity in the store for a given day and user address.
///
/// This function records user activity in two places:
/// 1. A general user record
/// 2. A daily snapshot of active users
///
/// # Arguments
///
/// * `store` - The store to set the values in
/// * `day_id` - The ID of the day for which to record the activity
/// * `user_address` - The address of the user whose activity is being recorded
/// * `value` - The value to set (typically 1, indicating activity)
///
/// # Note
///
/// This function uses `set_if_not_exists_many` to ensure that each user is only counted once per day,
/// even if they perform multiple activities.
fn set_user_activity(
    store: &StoreSetIfNotExistsBigInt,
    day_id: i64,
    user_address: &str,
    value: &BigInt,
) {
    store.set_if_not_exists_many(
        0,
        &vec![
            StoreKey::User.get_unique_key(user_address),
            StoreKey::UsageMetricsDailySnapshot(day_id, Some(Box::new(StoreKey::ActiveUsers)))
                .get_snapshot_key(Some(user_address)),
        ],
        value,
    );
}
