use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::{event::Type, Event, Events, Pool, Swap, Swaps};
use substreams::log;
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

    let zero = "0".to_string();
    let mut swaps: Vec<Swap> = Vec::new();

    for event in raw_events.data {
        if let Some(event_type) = event.r#type.clone() {
            match event_type {
                Type::TwoHopSwap(two_hop_swap_event) => {
                    if let (Some(instruction), Some(accounts)) =
                        (two_hop_swap_event.instruction, two_hop_swap_event.accounts)
                    {
                        let swap_a_to_b_one = handle_swap(
                            instruction.a_to_b_one,
                            instruction
                                .amount_a_one
                                .clone()
                                .unwrap_or_else(|| zero.clone()),
                            instruction
                                .amount_b_one
                                .clone()
                                .unwrap_or_else(|| zero.clone()),
                            instruction
                                .amount_a_one_post
                                .clone()
                                .unwrap_or_else(|| zero.clone()),
                            instruction
                                .amount_b_one_post
                                .clone()
                                .unwrap_or_else(|| zero.clone()),
                            accounts.token_authority.clone(),
                            accounts.whirlpool_one.clone(),
                            &pool_store,
                            event.clone(),
                        );

                        let swap_a_to_b_two = handle_swap(
                            instruction.a_to_b_two,
                            instruction
                                .amount_a_two
                                .clone()
                                .unwrap_or_else(|| zero.clone()),
                            instruction
                                .amount_b_two
                                .clone()
                                .unwrap_or_else(|| zero.clone()),
                            instruction
                                .amount_a_two_post
                                .clone()
                                .unwrap_or_else(|| zero.clone()),
                            instruction
                                .amount_b_two_post
                                .clone()
                                .unwrap_or_else(|| zero.clone()),
                            accounts.token_authority,
                            accounts.whirlpool_two,
                            &pool_store,
                            event,
                        );

                        if let Some(swap) = swap_a_to_b_one {
                            swaps.push(swap);
                        }

                        if let Some(swap) = swap_a_to_b_two {
                            swaps.push(swap);
                        }
                    }
                }

                Type::Swap(orca_swap_event) => {
                    if let (Some(instruction), Some(accounts)) =
                        (orca_swap_event.instruction, orca_swap_event.accounts)
                    {
                        let swap_a_to_b = handle_swap(
                            instruction.a_to_b,
                            instruction.amount_a.clone().unwrap_or_else(|| zero.clone()),
                            instruction.amount_b.clone().unwrap_or_else(|| zero.clone()),
                            instruction
                                .amount_a_post
                                .clone()
                                .unwrap_or_else(|| zero.clone()),
                            instruction
                                .amount_b_post
                                .clone()
                                .unwrap_or_else(|| zero.clone()),
                            accounts.token_authority,
                            accounts.whirlpool,
                            &pool_store,
                            event,
                        );

                        if let Some(swap) = swap_a_to_b {
                            swaps.push(swap);
                        }
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
    token_a_balance: String,
    token_b_balance: String,
    signer: String,
    pool_address: String,
    pool_store: &StoreGetProto<Pool>,
    event: Event,
) -> Option<Swap> {
    let pool = pool_store.get_last(StoreKey::Pool.get_unique_key(&pool_address));

    if pool.is_none() {
        log::info!("Pool not found: {:?}", pool_address);
        return None;
    }

    let pool = pool.unwrap();

    let (token_in, amount_in, token_in_balance, token_out, amount_out, token_out_balance) =
        if a_to_b {
            (
                pool.token_mint_a,
                amount_a,
                token_a_balance,
                pool.token_mint_b,
                amount_b,
                token_b_balance,
            )
        } else {
            (
                pool.token_mint_b,
                amount_b,
                token_b_balance,
                pool.token_mint_a,
                amount_a,
                token_a_balance,
            )
        };

    Some(Swap {
        id: format!("{}-{}", event.txn_id, event.slot),

        token_in,
        token_out,

        token_in_balance,
        token_out_balance,

        amount_in,
        amount_out,

        from: signer,
        to: pool_address,

        slot: event.slot,
        txn_id: event.txn_id,
        block_height: event.block_height,
        block_timestamp: event.block_timestamp,
        block_hash: event.block_hash,
    })
}
