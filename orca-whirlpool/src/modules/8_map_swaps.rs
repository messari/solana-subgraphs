use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::{event::Type, Event, Events, Pool, Swap, Swaps};
use crate::traits::swap_instructions::SwapInstruction;
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

    let mut swaps: Vec<Swap> = Vec::new();

    for event in raw_events.data {
        if let Some(event_type) = event.r#type.clone() {
            match event_type {
                Type::TwoHopSwap(two_hop_swap_event) => {
                    process_swap(&two_hop_swap_event, &pool_store, &event, &mut swaps);
                }
                Type::TwoHopSwapV2(two_hop_swap_v2_event) => {
                    process_swap(&two_hop_swap_v2_event, &pool_store, &event, &mut swaps);
                }
                Type::Swap(orca_swap_event) => {
                    process_swap(&orca_swap_event, &pool_store, &event, &mut swaps);
                }
                Type::SwapV2(orca_swap_v2_event) => {
                    process_swap(&orca_swap_v2_event, &pool_store, &event, &mut swaps);
                }
                _ => {}
            }
        }
    }

    Ok(Swaps { data: swaps })
}

fn process_swap<T: SwapInstruction>(
    swap_event: &T,
    pool_store: &StoreGetProto<Pool>,
    event: &Event,
    swaps: &mut Vec<Swap>,
) {
    log::info!("Processing swap: {:?}", event.txn_id);

    let swap = handle_swap(
        swap_event.a_to_b(),
        swap_event.amount_a(),
        swap_event.amount_b(),
        swap_event.amount_a_post(),
        swap_event.amount_b_post(),
        swap_event.token_authority(),
        swap_event.whirlpool(),
        pool_store,
        event.clone(),
    );

    if let Some(swap) = swap {
        swaps.push(swap);
    }

    if swap_event.is_two_hop() {
        if let Some(second_hop) = swap_event.second_hop() {
            let second_swap = handle_swap(
                second_hop.a_to_b(),
                second_hop.amount_a(),
                second_hop.amount_b(),
                second_hop.amount_a_post(),
                second_hop.amount_b_post(),
                second_hop.token_authority(),
                second_hop.whirlpool(),
                pool_store,
                event.clone(),
            );

            if let Some(second_swap) = second_swap {
                swaps.push(second_swap);
            }
        }
    }
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
    let pool = match pool_store.get_last(StoreKey::Pool.get_unique_key(&pool_address)) {
        Some(pool) => pool,
        None => {
            log::info!("Pool not found: {:?}", pool_address);
            return None;
        }
    };

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
        id: format!("SWAP-{}-{}", event.txn_id, event.slot),

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
