use crate::pb::messari::orca_whirlpool::v1::{Deposits, Pool, Pools, Swaps, Withdraws};
use crate::{constants, db};

use substreams::pb::substreams::Clock;
use substreams::skip_empty_output;
use substreams::store::{DeltaBigInt, DeltaInt64, Deltas, StoreGet, StoreGetProto};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

#[substreams::handlers::map]
fn graph_out(
    clock: Clock,
    initialized_pools: Pools,
    pools_store: StoreGetProto<Pool>,
    cumulative_users_delta: Deltas<DeltaInt64>,
    total_pool_count_delta: Deltas<DeltaInt64>,
    pool_balances_delta: Deltas<DeltaBigInt>,
    pool_liquidity_delta: Deltas<DeltaBigInt>,
    map_deposits: Deposits,
    map_withdraws: Withdraws,
    map_swaps: Swaps,
) -> Result<EntityChanges, ()> {
    skip_empty_output();

    let mut tables = Tables::new();
    let is_initialized = clock.number != 124152351;
    let protocol_id = bs58::encode(constants::ORCA_WHIRLPOOL).into_string();

    db::handle_protocol_entity(
        &mut tables,
        cumulative_users_delta,
        total_pool_count_delta,
        &protocol_id,
        is_initialized,
    );
    db::handle_pool_entity(
        &mut tables,
        initialized_pools,
        pools_store,
        pool_balances_delta,
        pool_liquidity_delta,
        &protocol_id,
    );

    db::handle_deposit_entity(&mut tables, map_deposits, &protocol_id);
    db::handle_withdraw_entity(&mut tables, map_withdraws, &protocol_id);
    db::handle_swap_entity(&mut tables, map_swaps, &protocol_id);

    Ok(tables.to_entity_changes())
}
