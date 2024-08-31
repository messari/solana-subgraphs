use crate::pb::messari::orca_whirlpool::v1::event::Type::{
    DecreaseLiquidity, IncreaseLiquidity, InitalizePool, Swap, TwoHopSwap,
};
use crate::pb::messari::orca_whirlpool::v1::Events;

use substreams::skip_empty_output;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

#[substreams::handlers::map]
fn graph_out(events: Events) -> Result<EntityChanges, ()> {
    skip_empty_output();

    let mut tables = Tables::new();

    for event in events.data.iter() {
        let id = [event.txn_id.clone(), event.slot.to_string()].join("-");

        match event.r#type.clone().unwrap() {
            InitalizePool(initialize_pool) => {
                let instruction = initialize_pool.instruction.unwrap();

                tables
                    .create_row("DecreaseLiquidity", id.as_str())
                    .set_bigint("bumps", &instruction.bumps.to_string())
                    .set_bigint("tick_spacing", &instruction.tick_spacing.to_string())
                    .set_bigint("initial_sqrt_price", &instruction.initial_sqrt_price)
                    .set_bigint("slot", &event.slot.to_string())
                    .set("txn_id", &event.txn_id)
                    .set_bigint("block_height", &event.block_height.to_string())
                    .set_bigint("block_timestamp", &event.block_timestamp.to_string())
                    .set("block_hash", &event.block_hash);
            }
            DecreaseLiquidity(decrease_liquidity) => {
                let instruction = decrease_liquidity.instruction.unwrap();

                tables
                    .create_row("DecreaseLiquidity", id.as_str())
                    .set_bigint("liquidity_amount", &instruction.liquidity_amount)
                    .set_bigint("token_min_a", &instruction.token_min_a)
                    .set_bigint("token_min_b", &instruction.token_min_b)
                    .set_bigint("slot", &event.slot.to_string())
                    .set("txn_id", &event.txn_id)
                    .set_bigint("block_height", &event.block_height.to_string())
                    .set_bigint("block_timestamp", &event.block_timestamp.to_string())
                    .set("block_hash", &event.block_hash);
            }
            IncreaseLiquidity(increase_liquidity) => {
                let instruction = increase_liquidity.instruction.unwrap();

                tables
                    .create_row("IncreaseLiquidity", id.as_str())
                    .set_bigint("liquidity_amount", &instruction.liquidity_amount)
                    .set_bigint("token_max_a", &instruction.token_max_a)
                    .set_bigint("token_max_b", &instruction.token_max_b)
                    .set_bigint("slot", &event.slot.to_string())
                    .set("txn_id", &event.txn_id)
                    .set_bigint("block_height", &event.block_height.to_string())
                    .set_bigint("block_timestamp", &event.block_timestamp.to_string())
                    .set("block_hash", &event.block_hash);
            }
            TwoHopSwap(two_hop_swap) => {
                let instruction = two_hop_swap.instruction.unwrap();

                tables
                    .create_row("TwoHopSwap", id.as_str())
                    .set_bigint("amount", &instruction.amount)
                    .set_bigint(
                        "other_amount_threshold",
                        &instruction.other_amount_threshold,
                    )
                    .set(
                        "amount_specified_is_input",
                        instruction.amount_specified_is_input,
                    )
                    .set("a_to_b_one", instruction.a_to_b_one)
                    .set("a_to_b_two", instruction.a_to_b_two)
                    .set_bigint("sqrt_price_limit_one", &instruction.sqrt_price_limit_one)
                    .set_bigint("sqrt_price_limit_two", &instruction.sqrt_price_limit_two)
                    .set_bigint("slot", &event.slot.to_string())
                    .set("txn_id", &event.txn_id)
                    .set_bigint("block_height", &event.block_height.to_string())
                    .set_bigint("block_timestamp", &event.block_timestamp.to_string())
                    .set("block_hash", &event.block_hash);
            }
            Swap(swap) => {
                let instruction = swap.instruction.unwrap();

                tables
                    .create_row("Swap", id.as_str())
                    .set_bigint("amount", &instruction.amount)
                    .set_bigint(
                        "other_amount_threshold",
                        &instruction.other_amount_threshold,
                    )
                    .set_bigint("sqrt_price_limit", &instruction.sqrt_price_limit)
                    .set(
                        "amount_specified_is_input",
                        instruction.amount_specified_is_input,
                    )
                    .set("a_to_b", instruction.a_to_b)
                    .set_bigint("slot", &event.slot.to_string())
                    .set("txn_id", &event.txn_id)
                    .set_bigint("block_height", &event.block_height.to_string())
                    .set_bigint("block_timestamp", &event.block_timestamp.to_string())
                    .set("block_hash", &event.block_hash);
            }
        }
    }

    Ok(tables.to_entity_changes())
}
