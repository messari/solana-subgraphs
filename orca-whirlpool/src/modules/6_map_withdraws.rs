use crate::{
    key_store::StoreKey,
    pb::messari::orca_whirlpool::v1::{event::Type, Events, Pool, Withdraw, Withdraws},
};
use substreams::{
    skip_empty_output,
    store::{StoreGet, StoreGetProto},
};

#[substreams::handlers::map]
pub fn map_withdraws(
    raw_events: Events,
    pools_store: StoreGetProto<Pool>,
) -> Result<Withdraws, substreams::errors::Error> {
    skip_empty_output();

    let data: Vec<Withdraw> = raw_events
        .data
        .into_iter()
        .filter_map(|event| {
            if let Type::DecreaseLiquidity(decrease_liquidity_event) = event.r#type.unwrap() {
                let id = format!("{}-{}", event.txn_id, event.slot);

                let accounts = decrease_liquidity_event.accounts.unwrap();
                let instruction = decrease_liquidity_event.instruction.unwrap();

                let pool =
                    pools_store.must_get_last(StoreKey::Pool.get_unique_key(&accounts.whirlpool));

                Some(Withdraw {
                    id,
                    token_a: pool.token_mint_a,
                    token_b: pool.token_mint_b,
                    amount_a: instruction.amount_a.unwrap_or_default(),
                    amount_b: instruction.amount_b.unwrap_or_default(),
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

    Ok(Withdraws { data })
}
