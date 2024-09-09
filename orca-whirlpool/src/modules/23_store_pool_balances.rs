use substreams::scalar::BigInt;
use substreams::skip_empty_output;
use substreams::store::{StoreNew, StoreSet, StoreSetBigInt};

use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::{Deposits, Swaps, Withdraws};

#[substreams::handlers::store]
pub fn store_pool_balances(
    pool_deposits: Deposits,
    pool_withdraws: Withdraws,
    pool_swaps: Swaps,
    store: StoreSetBigInt,
) {
    skip_empty_output();

    process_pool_balances(&pool_deposits.data, &store, |item| {
        vec![
            (
                item.to.clone(),
                item.token_a.clone(),
                item.token_a_balance.clone(),
            ),
            (
                item.to.clone(),
                item.token_b.clone(),
                item.token_b_balance.clone(),
            ),
        ]
    });

    process_pool_balances(&pool_withdraws.data, &store, |item| {
        vec![
            (
                item.to.clone(),
                item.token_a.clone(),
                item.token_a_balance.clone(),
            ),
            (
                item.to.clone(),
                item.token_b.clone(),
                item.token_b_balance.clone(),
            ),
        ]
    });

    process_pool_balances(&pool_swaps.data, &store, |item| {
        vec![
            (
                item.to.clone(),
                item.token_in.clone(),
                item.token_in_balance.clone(),
            ),
            (
                item.to.clone(),
                item.token_out.clone(),
                item.token_out_balance.clone(),
            ),
        ]
    });
}

/// Processes pool balances for a given set of items.
///
/// This function takes a collection of items, a store to update, and a closure that extracts
/// balance information from each item. It then updates the store with the extracted balance
/// information for each pool and token combination.
///
/// # Arguments
///
/// * `items` - A slice of items to process
/// * `store` - The store to update with balance information
/// * `f` - A closure that takes an item and returns a vector of (pool, token, balance) tuples
///
/// # Type Parameters
///
/// * `T` - The type of items being processed
/// * `F` - The type of the closure for extracting balance information
fn process_pool_balances<T, F>(items: &[T], store: &StoreSetBigInt, f: F)
where
    F: Fn(&T) -> Vec<(String, String, String)>,
{
    items.iter().for_each(|item| {
        f(item).iter().for_each(|(to, token, balance)| {
            store.set(
                0,
                StoreKey::PoolBalance.get_unique_keys(to, token),
                &BigInt::try_from(balance.clone()).unwrap_or(BigInt::zero()),
            );
        });
    });
}
