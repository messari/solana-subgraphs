use crate::{
    key_store::StoreKey,
    pb::messari::orca_whirlpool::v1::{event::Type, Deposit, Deposits, Event, Events, Pool},
    traits::deposit_instructions::DepositInstruction,
};
use substreams::{
    log, skip_empty_output,
    store::{StoreGet, StoreGetProto},
};

#[substreams::handlers::map]
pub fn map_deposits(
    raw_events: Events,
    pools_store: StoreGetProto<Pool>,
) -> Result<Deposits, substreams::errors::Error> {
    skip_empty_output();
    let mut deposits: Vec<Deposit> = Vec::new();

    for event in raw_events.data {
        if let Some(event_type) = event.r#type.clone() {
            match event_type {
                Type::IncreaseLiquidity(instruction) => {
                    process_deposit(&instruction, &pools_store, &event, &mut deposits);
                }
                Type::IncreaseLiquidityV2(instruction) => {
                    process_deposit(&instruction, &pools_store, &event, &mut deposits);
                }
                _ => {}
            }
        }
    }

    Ok(Deposits { data: deposits })
}

fn process_deposit<T: DepositInstruction>(
    deposit_event: &T,
    pool_store: &StoreGetProto<Pool>,
    event: &Event,
    deposits: &mut Vec<Deposit>,
) {
    let pool = match pool_store.get_last(StoreKey::Pool.get_unique_key(&deposit_event.whirlpool()))
    {
        Some(pool) => pool,
        None => {
            log::info!("Pool not found: {:?}", deposit_event.whirlpool());
            return;
        }
    };

    deposits.push(Deposit {
        id: format!("{}-{}", event.txn_id, event.slot),

        token_a: pool.token_mint_a,
        token_b: pool.token_mint_b,

        token_a_balance: deposit_event.amount_a_post(),
        token_b_balance: deposit_event.amount_b_post(),

        amount_a: deposit_event.amount_a(),
        amount_b: deposit_event.amount_b(),

        output_amount: deposit_event.liquidity_amount(),

        from: deposit_event.position_authority(),
        to: deposit_event.whirlpool(),

        slot: event.slot,
        txn_id: event.txn_id.clone(),
        block_height: event.block_height,
        block_timestamp: event.block_timestamp,
        block_hash: event.block_hash.clone(),
    })
}
