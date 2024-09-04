use crate::{
    key_store::StoreKey,
    pb::messari::orca_whirlpool::v1::{event::Type, Deposit, Deposits, Events, Pool},
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

    let zero = "0".to_string();

    let data: Vec<Deposit> = raw_events
        .data
        .into_iter()
        .filter_map(|event| {
            if let Some(Type::IncreaseLiquidity(increase_liquidity_event)) = event.r#type {
                let id = format!("{}-{}", event.txn_id, event.slot);

                let accounts = increase_liquidity_event.accounts?;
                let instruction = increase_liquidity_event.instruction?;

                let pool = pools_store.get_last(StoreKey::Pool.get_unique_key(&accounts.whirlpool));

                if pool.is_none() {
                    log::info!("Pool not found: {:?}", accounts.whirlpool);
                    return None;
                }

                let pool = pool.unwrap();

                Some(Deposit {
                    id,

                    token_a: pool.token_mint_a,
                    token_b: pool.token_mint_b,

                    token_a_balance: instruction.amount_a_post.unwrap_or_else(|| zero.clone()),
                    token_b_balance: instruction.amount_b_post.unwrap_or_else(|| zero.clone()),

                    amount_a: instruction.amount_a.unwrap_or_else(|| zero.clone()),
                    amount_b: instruction.amount_b.unwrap_or_else(|| zero.clone()),

                    output_amount: instruction.liquidity_amount,

                    from: accounts.position_authority,
                    to: accounts.whirlpool,

                    slot: event.slot,
                    txn_id: event.txn_id,
                    block_height: event.block_height,
                    block_timestamp: event.block_timestamp,
                    block_hash: event.block_hash,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(Deposits { data })
}
