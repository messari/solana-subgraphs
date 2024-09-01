use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::{event::Type, Event, Events, Pool, Swap, Swaps};
use substreams::{
    skip_empty_output,
    store::{StoreGet, StoreGetProto},
};

#[substreams::handlers::map]
pub fn map_swaps(
    raw_events: Events,
    pool_store: StoreGetProto<Pool>,
) -> Result<Swaps, substreams::errors::Error> {
    skip_empty_output();

    let mut swaps: Vec<Swap> = Vec::new();

    for event in raw_events.data {
        if let Some(event_type) = event.r#type.clone() {
            match event_type {
                Type::TwoHopSwap(two_hop_swap_event) => {
                    if let (Some(instruction), Some(accounts)) =
                        (two_hop_swap_event.instruction, two_hop_swap_event.accounts)
                    {
                        let swap_1 = handle_swap(
                            instruction.a_to_b_one,
                            instruction.amount_a_one.unwrap_or_default(),
                            instruction.amount_b_one.unwrap_or_default(),
                            accounts.token_authority.clone(),
                            accounts.whirlpool_one.clone(),
                            &pool_store,
                            event.clone(),
                        );

                        let swap_2 = handle_swap(
                            instruction.a_to_b_two,
                            instruction.amount_a_two.unwrap_or_default(),
                            instruction.amount_b_two.unwrap_or_default(),
                            accounts.token_authority,
                            accounts.whirlpool_two,
                            &pool_store,
                            event,
                        );

                        swaps.push(swap_1);
                        swaps.push(swap_2);
                    }
                }

                Type::Swap(orca_swap_event) => {
                    if let (Some(instruction), Some(accounts)) =
                        (orca_swap_event.instruction, orca_swap_event.accounts)
                    {
                        let swap = handle_swap(
                            instruction.a_to_b,
                            instruction.amount_a.unwrap_or_default(),
                            instruction.amount_b.unwrap_or_default(),
                            accounts.token_authority,
                            accounts.whirlpool,
                            &pool_store,
                            event,
                        );

                        swaps.push(swap);
                    }
                }

                _ => {}
            }
        }
    }

    Ok(Swaps { data: swaps })
}

fn handle_swap(
    a_to_b: bool,
    amount_a: String,
    amount_b: String,
    signer: String,
    pool_address: String,
    pool_store: &StoreGetProto<Pool>,
    event: Event,
) -> Swap {
    let pool = pool_store.must_get_last(StoreKey::Pool.get_unique_key(&pool_address));

    let (token_in, amount_in, token_out, amount_out) = if a_to_b {
        (pool.token_mint_a, amount_a, pool.token_mint_b, amount_b)
    } else {
        (pool.token_mint_b, amount_b, pool.token_mint_a, amount_a)
    };

    Swap {
        id: format!("{}-{}", event.txn_id, event.slot),
        token_in,
        token_out,
        amount_in,
        amount_out,
        from: signer,
        to: pool_address,
        slot: event.slot,
        txn_id: event.txn_id,
        block_height: event.block_height,
        block_timestamp: event.block_timestamp,
        block_hash: event.block_hash,
    }
}
