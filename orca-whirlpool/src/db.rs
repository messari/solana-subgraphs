use crate::{
    key_store::StoreKey,
    pb::messari::orca_whirlpool::v1::{Deposits, Pool, Pools, Swaps, Withdraws},
};

use substreams::{
    key,
    pb::substreams::store_delta::Operation,
    scalar::{BigDecimal, BigInt},
    store::{
        DeltaBigInt, DeltaExt, DeltaInt64, Deltas, StoreGet, StoreGetBigInt, StoreGetInt64,
        StoreGetProto,
    },
};
use substreams_entity_change::tables::Tables;

pub fn handle_protocol_entity(
    tables: &mut Tables,
    users_delta: Deltas<DeltaInt64>,
    pools_delta: Deltas<DeltaInt64>,
    protocol_id: &String,
    is_initialized: bool,
) {
    // Create protocol entity if not initialized
    if !is_initialized {
        tables
            .create_row("Protocol", protocol_id)
            .set("cumulativeUniqueUsers", BigInt::zero())
            .set("totalPoolCount", BigInt::zero());

        return;
    }

    // Update cumulative unique users
    users_delta.iter().for_each(|delta| {
        tables
            .update_row("Protocol", protocol_id)
            .set("cumulativeUniqueUsers", delta.new_value);
    });

    // Update total pool count
    pools_delta.iter().for_each(|delta| {
        tables
            .update_row("Protocol", protocol_id)
            .set("totalPoolCount", delta.new_value);
    });
}

pub fn handle_usage_metrics_daily_snapshot_entity(
    tables: &mut Tables,
    active_users_store: StoreGetInt64,
    total_pool_count_store: StoreGetInt64,
    user_activity_deltas: &Deltas<DeltaBigInt>,
    protocol_id: &String,
    block_number: &BigInt,
    timestamp: &BigInt,
) {
    let total_pool_count = total_pool_count_store
        .get_last(StoreKey::TotalPoolCount.unique_id())
        .unwrap_or_default();
    let cumulative_users_count = active_users_store
        .get_last(StoreKey::CumulativeUsers.unique_id())
        .unwrap_or_default();

    user_activity_deltas
        .iter()
        .key_first_segment_eq("UsageMetricsDailySnapshot")
        .operation_not_eq(Operation::Delete)
        .for_each(|delta| {
            let day_id = key::segment_at(&delta.key, 1)
                .parse::<i64>()
                .unwrap_or_default();
            let field_id = key::segment_at(&delta.key, 2);

            let active_users_count = active_users_store
                .get_last(
                    StoreKey::UsageMetricsDailySnapshot(
                        day_id,
                        Some(Box::new(StoreKey::ActiveUsers)),
                    )
                    .get_snapshot_key(None),
                )
                .unwrap_or_default();

            if field_id == StoreKey::TxnCount.unique_id() && delta.new_value == BigInt::one() {
                // Create a new daily snapshot
                let bigint0 = BigInt::zero();

                tables
                    .update_row("UsageMetricsDailySnapshot", day_id.to_string())
                    .set("protocol", protocol_id)
                    .set("dailyActiveUsers", &bigint0)
                    .set("cumulativeUniqueUsers", &bigint0)
                    .set("dailyTransactionCount", &bigint0)
                    .set("dailyDepositCount", &bigint0)
                    .set("dailyWithdrawCount", &bigint0)
                    .set("dailySwapCount", &bigint0)
                    .set("totalPoolCount", total_pool_count)
                    .set("blockNumber", block_number)
                    .set("timestamp", timestamp);
            }

            let field = match field_id {
                x if x == StoreKey::SwapCount.unique_id() => "dailySwapCount",
                x if x == StoreKey::DepositCount.unique_id() => "dailyDepositCount",
                x if x == StoreKey::WithdrawCount.unique_id() => "dailyWithdrawCount",
                x if x == StoreKey::TxnCount.unique_id() => "dailyTransactionCount",
                _ => "",
            };

            tables
                .update_row("UsageMetricsDailySnapshot", day_id.to_string())
                .set("dailyActiveUsers", active_users_count)
                .set("cumulativeUniqueUsers", cumulative_users_count)
                .set(field, &delta.new_value)
                .set("totalPoolCount", total_pool_count)
                .set("blockNumber", block_number)
                .set("timestamp", timestamp);
        });
}

pub fn handle_usage_metrics_daily_snapshot_entity(
    tables: &mut Tables,
    active_users_store: StoreGetInt64,
    total_pool_count_store: StoreGetInt64,
    user_activity_deltas: &Deltas<DeltaBigInt>,
    protocol_id: &String,
    block_number: &BigInt,
    timestamp: &BigInt,
) {
    let total_pool_count = total_pool_count_store
        .get_last(StoreKey::TotalPoolCount.unique_id())
        .unwrap_or_default();
    let cumulative_users_count = active_users_store
        .get_last(StoreKey::CumulativeUsers.unique_id())
        .unwrap_or_default();

    user_activity_deltas
        .iter()
        .key_first_segment_eq("UsageMetricsDailySnapshot")
        .operation_not_eq(Operation::Delete)
        .for_each(|delta| {
            let day_id = key::segment_at(&delta.key, 1)
                .parse::<i64>()
                .unwrap_or_default();
            let field_id = key::segment_at(&delta.key, 2);

            let active_users_count = active_users_store
                .get_last(
                    StoreKey::UsageMetricsDailySnapshot(
                        day_id,
                        Some(Box::new(StoreKey::ActiveUsers)),
                    )
                    .get_snapshot_key(None),
                )
                .unwrap_or_default();

            if field_id == StoreKey::TxnCount.unique_id() && delta.new_value == BigInt::one() {
                // Create a new daily snapshot
                let bigint0 = BigInt::zero();

                tables
                    .create_row("UsageMetricsDailySnapshot", day_id.to_string())
                    .set("protocol", protocol_id)
                    .set("dailyActiveUsers", &bigint0)
                    .set("cumulativeUniqueUsers", &bigint0)
                    .set("dailyTransactionCount", &bigint0)
                    .set("dailyDepositCount", &bigint0)
                    .set("dailyWithdrawCount", &bigint0)
                    .set("dailySwapCount", &bigint0)
                    .set("totalPoolCount", total_pool_count)
                    .set("blockNumber", block_number)
                    .set("timestamp", timestamp);
            }

            let field = match field_id {
                x if x == StoreKey::SwapCount.unique_id() => "dailySwapCount",
                x if x == StoreKey::DepositCount.unique_id() => "dailyDepositCount",
                x if x == StoreKey::WithdrawCount.unique_id() => "dailyWithdrawCount",
                x if x == StoreKey::TxnCount.unique_id() => "dailyTransactionCount",
                _ => "",
            };

            tables
                .update_row("UsageMetricsDailySnapshot", day_id.to_string())
                .set("dailyActiveUsers", active_users_count)
                .set("cumulativeUniqueUsers", cumulative_users_count)
                .set(field, &delta.new_value)
                .set("totalPoolCount", total_pool_count)
                .set("blockNumber", block_number)
                .set("timestamp", timestamp);
        });
}

pub fn handle_pool_entity(
    tables: &mut Tables,
    initialized_pools: Pools,
    pools_store: &StoreGetProto<Pool>,
    pool_balances_delta: &Deltas<DeltaBigInt>,
    pool_liquidity_delta: &Deltas<DeltaBigInt>,
    protocol_id: &String,
) {
    initialized_pools.data.iter().for_each(|pool| {
        tables
            .create_row("LiquidityPool", &pool.address)
            .set("protocol", protocol_id)
            .set(
                "inputTokens",
                vec![pool.token_mint_a.clone(), pool.token_mint_b.clone()],
            )
            .set("outputToken", &pool.address)
            .set("token0Balance", &BigInt::zero())
            .set("token1Balance", &BigInt::zero())
            .set(
                "cumulativeVolumeByTokenAmount",
                vec![BigInt::zero(), BigInt::zero()],
            )
            .set("outputTokenSupply", &BigInt::zero())
            .set("createdTimestamp", pool.created_timestamp)
            .set("createdBlockNumber", pool.created_block_number);
    });

    pool_balances_delta
        .iter()
        .key_first_segment_eq(StoreKey::PoolBalance.unique_id())
        .for_each(|delta| {
            let pool_address = key::segment_at(&delta.key, 1);
            let input_token = key::segment_at(&delta.key, 2);

            let pool = pools_store.get_last(StoreKey::Pool.get_unique_key(pool_address));

            if pool.is_none() {
                log::info!("Pool not found: {pool_address}");
                return;
            }
            let pool = pool.unwrap();

            let balance_field = if input_token == pool.token_mint_a {
                "token0Balance"
            } else if input_token == pool.token_mint_b {
                "token1Balance"
            } else {
                return;
            };

            tables
                .update_row("LiquidityPool", pool_address)
                .set(balance_field, &delta.new_value);
        });

    pool_liquidity_delta
        .iter()
        .key_first_segment_eq(StoreKey::PoolLiquidity.unique_id())
        .for_each(|delta| {
            let pool = key::segment_at(&delta.key, 1);

            tables
                .update_row("LiquidityPool", pool)
                .set("outputTokenSupply", &delta.new_value);
        });
}

pub fn handle_liquidity_pool_daily_snapshot_entity(
    tables: &mut Tables,
    pool_store: &StoreGetProto<Pool>,
    pool_balances_store: &StoreGetBigInt,
    pool_liquidity_store: &StoreGetBigInt,
    volume_by_token_amount_deltas: &Deltas<DeltaBigInt>,
    protocol_id: &String,
    block_number: &BigInt,
    timestamp: &BigInt,
) {
    let bigint0 = BigInt::zero();

    volume_by_token_amount_deltas
        .iter()
        .key_first_segment_eq("PoolDailySnapshot")
        .operation_not_eq(Operation::Delete)
        .for_each(|delta| {
            let day_id = key::segment_at(&delta.key, 1);
            let pool_address = key::segment_at(&delta.key, 3);
            let token_address = key::segment_at(&delta.key, 4);
            let pool = pool_store.must_get_last(StoreKey::Pool.get_unique_key(pool_address));

            if delta.old_value == BigInt::zero() {
                tables
                    .update_row(
                        "LiquidityPoolDailySnapshot",
                        format!("{pool_address}-{day_id}"),
                    )
                    .set("protocol", protocol_id)
                    .set("pool", pool_address)
                    .set("blockNumber", block_number)
                    .set("timestamp", timestamp)
                    .set("dailyVolumeByToken0Amount", &bigint0)
                    .set("dailyVolumeByToken1Amount", &bigint0)
                    .set("token0Balances", &bigint0)
                    .set("token1Balances", &bigint0)
                    .set("outputTokenSupply", &bigint0);
            }

            let (volume_field, balance_field, token_balance) = if token_address == pool.token_mint_a
            {
                (
                    "dailyVolumeByToken0Amount",
                    "token0Balances",
                    pool_balances_store
                        .get_last(
                            StoreKey::PoolBalance.get_unique_keys(pool_address, token_address),
                        )
                        .unwrap_or_default(),
                )
            } else if token_address == pool.token_mint_b {
                (
                    "dailyVolumeByToken1Amount",
                    "token1Balances",
                    pool_balances_store
                        .get_last(
                            StoreKey::PoolBalance.get_unique_keys(pool_address, token_address),
                        )
                        .unwrap_or_default(),
                )
            } else {
                return;
            };

            let output_token_supply = pool_liquidity_store
                .get_last(StoreKey::PoolLiquidity.get_unique_key(pool_address))
                .unwrap_or_default();

            tables
                .update_row(
                    "LiquidityPoolDailySnapshot",
                    format!("{pool_address}-{day_id}"),
                )
                .set(volume_field, &delta.new_value)
                .set(balance_field, &token_balance)
                .set("outputTokenSupply", &output_token_supply)
                .set("blockNumber", block_number)
                .set("timestamp", timestamp);
        });
}

pub fn _handle_deposit_entity(tables: &mut Tables, map_deposits: Deposits, protocol_id: &String) {
    map_deposits.data.iter().for_each(|deposit| {
        tables
            .create_row("Deposit", &deposit.id)
            .set("blockHash", &deposit.block_hash)
            .set("protocol", protocol_id)
            .set("to", &deposit.to)
            .set("from", &deposit.from)
            .set("slot", BigInt::from(deposit.slot))
            .set("blockNumber", BigInt::from(deposit.block_height))
            .set("timestamp", BigInt::from(deposit.block_timestamp))
            .set(
                "inputTokens",
                vec![deposit.token_a.clone(), deposit.token_b.clone()],
            )
            .set("outputToken", &deposit.to)
            .set(
                "inputTokenAmounts",
                vec![
                    BigInt::try_from(deposit.amount_a.clone()).unwrap(),
                    BigInt::try_from(deposit.amount_b.clone()).unwrap(),
                ],
            )
            .set(
                "outputTokenAmount",
                BigInt::try_from(deposit.output_amount.clone()).unwrap(),
            )
            .set("amountUSD", &BigDecimal::zero())
            .set("pool", &deposit.to);
    });
}

pub fn _handle_withdraw_entity(
    tables: &mut Tables,
    map_withdraws: Withdraws,
    protocol_id: &String,
) {
    map_withdraws.data.iter().for_each(|withdraw| {
        tables
            .create_row("Withdraw", &withdraw.id)
            .set("blockHash", &withdraw.block_hash)
            .set("protocol", protocol_id)
            .set("to", &withdraw.to)
            .set("from", &withdraw.from)
            .set("slot", BigInt::from(withdraw.slot))
            .set("blockNumber", BigInt::from(withdraw.block_height))
            .set("timestamp", BigInt::from(withdraw.block_timestamp))
            .set(
                "inputTokens",
                vec![withdraw.token_a.clone(), withdraw.token_b.clone()],
            )
            .set("outputToken", &withdraw.to)
            .set(
                "inputTokenAmounts",
                vec![
                    BigInt::try_from(withdraw.amount_a.clone()).unwrap(),
                    BigInt::try_from(withdraw.amount_b.clone()).unwrap(),
                ],
            )
            .set(
                "outputTokenAmount",
                BigInt::try_from(withdraw.output_amount.clone()).unwrap(),
            )
            .set("amountUSD", &BigDecimal::zero())
            .set("pool", &withdraw.to);
    });
}

pub fn _handle_swap_entity(tables: &mut Tables, map_swaps: Swaps, protocol_id: &String) {
    map_swaps.data.iter().for_each(|swap| {
        tables
            .create_row("Swap", &swap.id)
            .set("blockHash", &swap.block_hash)
            .set("protocol", protocol_id)
            .set("to", &swap.to)
            .set("from", &swap.from)
            .set("slot", BigInt::from(swap.slot))
            .set("blockNumber", BigInt::from(swap.block_height))
            .set("timestamp", BigInt::from(swap.block_timestamp))
            .set("tokenIn", &swap.token_in)
            .set_bigint("amountIn", &swap.amount_in)
            .set("amountInUSD", &BigDecimal::zero())
            .set("tokenOut", &swap.token_out)
            .set_bigint("amountOut", &swap.amount_out)
            .set("amountOutUSD", &BigDecimal::zero())
            .set("pool", &swap.to);
    });
}
