use crate::db;
use crate::pb::messari::orca_whirlpool::v1::{Pool, Pools};

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
) -> Result<EntityChanges, ()> {
    skip_empty_output();

    let mut tables = Tables::new();
    let is_initialized = clock.number != 124152351;

    db::handle_protocol_entity(
        &mut tables,
        cumulative_users_delta,
        total_pool_count_delta,
        is_initialized,
    );
    db::handle_pool_entity(
        &mut tables,
        initialized_pools,
        pools_store,
        pool_balances_delta,
        pool_liquidity_delta,
    );

    Ok(tables.to_entity_changes())
}
