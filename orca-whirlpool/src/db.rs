use crate::{
    key_store::StoreKey,
    pb::messari::orca_whirlpool::v1::{Deposits, Pool, Pools, Swaps, Withdraws},
};

use substreams::{
    key, log,
    scalar::{BigDecimal, BigInt},
    store::{DeltaBigInt, DeltaExt, DeltaInt64, Deltas, StoreGet, StoreGetProto},
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

pub fn handle_pool_entity(
    tables: &mut Tables,
    initialized_pools: Pools,
    pools_store: StoreGetProto<Pool>,
    pool_balances_delta: Deltas<DeltaBigInt>,
    pool_liquidity_delta: Deltas<DeltaBigInt>,
    protocol_id: &String,
) {
    initialized_pools.data.iter().for_each(|pool| {
        tables
            .create_row("Pool", &pool.address)
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
                .update_row("Pool", pool_address)
                .set(balance_field, &delta.new_value);
        });

    pool_liquidity_delta
        .iter()
        .key_first_segment_eq(StoreKey::PoolLiquidity.unique_id())
        .for_each(|delta| {
            let pool = key::segment_at(&delta.key, 1);

            tables
                .update_row("Pool", pool)
                .set("outputTokenSupply", &delta.new_value);
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
