use substreams::store::StoreGet;
use substreams::store::StoreGetProto;
use substreams::store::StoreNew;
use substreams::store::StoreSetIfNotExists;
use substreams::store::StoreSetIfNotExistsProto;

use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::Event;
use crate::pb::messari::orca_whirlpool::v1::Pool;
use crate::pb::messari::orca_whirlpool::v1::{event, Events, Swap};

#[substreams::handlers::store]
pub fn store_swaps(
    raw_events: Events,
    pool_store: StoreGetProto<Pool>,
    store: StoreSetIfNotExistsProto<Swap>,
) {
    for event in raw_events.data {
        match event.r#type.clone().unwrap() {
            event::Type::TwoHopSwap(two_hop_swap_event) => {
                let instruction = two_hop_swap_event.instruction.unwrap();
                let accounts = two_hop_swap_event.accounts.unwrap();

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

                store.set_if_not_exists(0, StoreKey::Swap.get_unique_key(&swap_1.id), &swap_1);
                store.set_if_not_exists(0, StoreKey::Swap.get_unique_key(&swap_2.id), &swap_2);
            }

            event::Type::Swap(orca_swap_event) => {
                let instruction = orca_swap_event.instruction.unwrap();
                let accounts = orca_swap_event.accounts.unwrap();

                let swap = handle_swap(
                    instruction.a_to_b,
                    instruction.amount_a.unwrap_or_default(),
                    instruction.amount_b.unwrap_or_default(),
                    accounts.token_authority,
                    accounts.whirlpool,
                    &pool_store,
                    event,
                );

                store.set_if_not_exists(0, StoreKey::Swap.get_unique_key(&swap.id), &swap);
            }

            _ => {}
        }
    }
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
    let token_in;
    let amount_in;
    let token_out;
    let amount_out;

    let pool = pool_store.must_get_last(StoreKey::Pool.get_unique_key(pool_address.as_str()));

    if a_to_b {
        token_in = pool.token_mint_a;
        amount_in = amount_a;
        token_out = pool.token_mint_b;
        amount_out = amount_b;
    } else {
        token_in = pool.token_mint_b;
        amount_in = amount_b;
        token_out = pool.token_mint_a;
        amount_out = amount_a;
    }

    Swap {
        id: [event.txn_id.clone().clone(), event.slot.clone().to_string()].join("-"),

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
