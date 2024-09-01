use crate::{
    constants,
    key_store::StoreKey,
    pb::messari::orca_whirlpool::v1::{Pool, Pools},
};

use substreams::{
    key,
    scalar::BigInt,
    store::{DeltaBigInt, DeltaExt, DeltaInt64, Deltas, StoreGetProto},
};
use substreams_entity_change::tables::Tables;

pub fn handle_protocol_entity(
    tables: &mut Tables,
    users_delta: Deltas<DeltaInt64>,
    pools_delta: Deltas<DeltaInt64>,
    is_initialized: bool,
) {
    let id = bs58::encode(constants::ORCA_WHIRLPOOL).into_string();

    // Create protocol entity if not initialized
    if !is_initialized {
        tables
            .create_row("Protocol", &id)
            .set("cumulativeUniqueUsers", BigInt::zero())
            .set("totalPoolCount", BigInt::zero());

        return;
    }

    // Update cumulative unique users
    users_delta.iter().for_each(|delta| {
        tables
            .update_row("Protocol", &id)
            .set("cumulativeUniqueUsers", delta.new_value);
    });

    // Update total pool count
    pools_delta.iter().for_each(|delta| {
        tables
            .update_row("Protocol", &id)
            .set("totalPoolCount", delta.new_value);
    });
}

pub fn handle_pool_entity(
    tables: &mut Tables,
    initialized_pools: Pools,
    pools_store: StoreGetProto<Pool>,
    pool_balances_delta: Deltas<DeltaBigInt>,
    pool_liquidity_delta: Deltas<DeltaBigInt>,
) {
    let protocol_id = bs58::encode(constants::ORCA_WHIRLPOOL).into_string();

    initialized_pools.data.iter().for_each(|pool| {
        tables
            .create_row("Pool", &pool.address)
            .set("protocol", &protocol_id)
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

            let pool = pools_store.must_get_last(StoreKey::Pool.get_unique_key(pool_address));

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
