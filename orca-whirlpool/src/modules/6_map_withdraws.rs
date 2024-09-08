use crate::{
    key_store::StoreKey,
    pb::messari::orca_whirlpool::v1::{event::Type, Event, Events, Pool, Withdraw, Withdraws},
    traits::withdraw_instructions::WithdrawInstruction,
};
use substreams::{
    log, skip_empty_output,
    store::{StoreGet, StoreGetProto},
};

#[substreams::handlers::map]
pub fn map_withdraws(
    raw_events: Events,
    pools_store: StoreGetProto<Pool>,
) -> Result<Withdraws, substreams::errors::Error> {
    skip_empty_output();
    let mut withdraws: Vec<Withdraw> = Vec::new();

    raw_events.data.into_iter().for_each(|event| {
        if let Some(event_type) = event.r#type.clone() {
            match event_type {
                Type::DecreaseLiquidity(instruction) => {
                    process_withdraw(&instruction, &pools_store, &event, &mut withdraws);
                }
                Type::DecreaseLiquidityV2(instruction) => {
                    process_withdraw(&instruction, &pools_store, &event, &mut withdraws);
                }
                _ => {}
            }
        }
    });

    Ok(Withdraws { data: withdraws })
}

fn process_withdraw<T: WithdrawInstruction>(
    withdraw_event: &T,
    pool_store: &StoreGetProto<Pool>,
    event: &Event,
    withdraws: &mut Vec<Withdraw>,
) {
    let pool = match pool_store.get_last(StoreKey::Pool.get_unique_key(&withdraw_event.whirlpool()))
    {
        Some(pool) => pool,
        None => {
            log::info!("Pool not found: {:?}", withdraw_event.whirlpool());
            return;
        }
    };

    withdraws.push(Withdraw {
        id: format!("{}-{}", event.txn_id.clone(), event.slot),

        token_a: pool.token_mint_a,
        token_b: pool.token_mint_b,

        token_a_balance: withdraw_event.amount_a_post(),
        token_b_balance: withdraw_event.amount_b_post(),

        amount_a: withdraw_event.amount_a(),
        amount_b: withdraw_event.amount_b(),

        output_amount: withdraw_event.liquidity_amount(),

        from: withdraw_event.position_authority(),
        to: withdraw_event.whirlpool(),

        slot: event.slot,
        txn_id: event.txn_id.clone(),
        block_height: event.block_height,
        block_timestamp: event.block_timestamp,
        block_hash: event.block_hash.clone(),
    });
}
