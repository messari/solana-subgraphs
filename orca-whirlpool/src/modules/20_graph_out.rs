use crate::pb::messari::orca_whirlpool::v1::event::Type::{
    DecreaseLiquidity as DecreaseLiquidityType, IncreaseLiquidity as IncreaseLiquidityType,
    InitalizePool as InitalizePoolType, Swap as SwapType, TwoHopSwap as TwoHopSwapType,
};
use crate::pb::messari::orca_whirlpool::v1::Events;
use crate::utils::string_to_bigint;
use substreams::scalar::BigInt;
use substreams::skip_empty_output;
use substreams_entity_change::pb::entity::{entity_change, EntityChange, EntityChanges};

#[substreams::handlers::map]
fn graph_out(events: Events) -> Result<EntityChanges, ()> {
    skip_empty_output();

    let mut entity_changes: Vec<EntityChange> = vec![];

    for event in events.data.iter() {
        let id = [event.txn_id.clone(), event.slot.to_string()].join("-");

        match event.r#type.clone().unwrap() {
            InitalizePoolType(initialize_pool) => {
                let instruction = initialize_pool.instruction.unwrap();
                let mut events_entity_change: EntityChange = EntityChange::new(
                    "InitializePool",
                    id.as_str(),
                    0,
                    entity_change::Operation::Create,
                );

                events_entity_change
                    .change("bumps", BigInt::from(instruction.bumps))
                    .change("tick_spacing", BigInt::from(instruction.tick_spacing))
                    .change(
                        "initial_sqrt_price",
                        string_to_bigint(instruction.initial_sqrt_price),
                    )
                    .change("slot", BigInt::from(event.slot))
                    .change("txn_id", event.txn_id.clone())
                    .change("block_height", BigInt::from(event.block_height))
                    .change("block_timestamp", BigInt::from(event.block_timestamp))
                    .change("block_hash", event.block_hash.clone());

                entity_changes.push(events_entity_change);
            }
            DecreaseLiquidityType(decrease_liquidity) => {
                let instruction = decrease_liquidity.instruction.unwrap();
                let mut events_entity_change = EntityChange::new(
                    "DecreaseLiquidity",
                    id.as_str(),
                    0,
                    entity_change::Operation::Create,
                );

                events_entity_change
                    .change(
                        "liquidity_amount",
                        string_to_bigint(instruction.liquidity_amount),
                    )
                    .change("token_min_a", BigInt::from(instruction.token_min_a))
                    .change("token_min_b", BigInt::from(instruction.token_min_b))
                    .change("slot", BigInt::from(event.slot))
                    .change("txn_id", event.txn_id.clone())
                    .change("block_height", BigInt::from(event.block_height))
                    .change("block_timestamp", BigInt::from(event.block_timestamp))
                    .change("block_hash", event.block_hash.clone());

                entity_changes.push(events_entity_change);
            }
            IncreaseLiquidityType(increase_liquidity) => {
                let instruction = increase_liquidity.instruction.unwrap();
                let mut events_entity_change = EntityChange::new(
                    "IncreaseLiquidity",
                    id.as_str(),
                    0,
                    entity_change::Operation::Create,
                );

                events_entity_change
                    .change(
                        "liquidity_amount",
                        string_to_bigint(instruction.liquidity_amount),
                    )
                    .change("token_max_a", BigInt::from(instruction.token_max_a))
                    .change("token_max_b", BigInt::from(instruction.token_max_b))
                    .change("slot", BigInt::from(event.slot))
                    .change("txn_id", event.txn_id.clone())
                    .change("block_height", BigInt::from(event.block_height))
                    .change("block_timestamp", BigInt::from(event.block_timestamp))
                    .change("block_hash", event.block_hash.clone());

                entity_changes.push(events_entity_change);
            }
            TwoHopSwapType(two_hop_swap) => {
                let instruction = two_hop_swap.instruction.unwrap();
                let mut events_entity_change = EntityChange::new(
                    "TwoHopSwap",
                    id.as_str(),
                    0,
                    entity_change::Operation::Create,
                );

                events_entity_change
                    .change("amount", BigInt::from(instruction.amount))
                    .change("other_amount_threshold", instruction.other_amount_threshold)
                    .change(
                        "amount_specified_is_input",
                        instruction.amount_specified_is_input,
                    )
                    .change("a_to_b_one", instruction.a_to_b_one)
                    .change("a_to_b_two", instruction.a_to_b_two)
                    .change(
                        "sqrt_price_limit_one",
                        string_to_bigint(instruction.sqrt_price_limit_one.clone()),
                    )
                    .change(
                        "sqrt_price_limit_two",
                        string_to_bigint(instruction.sqrt_price_limit_two.clone()),
                    )
                    .change("slot", BigInt::from(event.slot))
                    .change("txn_id", event.txn_id.clone())
                    .change("block_height", BigInt::from(event.block_height))
                    .change("block_timestamp", BigInt::from(event.block_timestamp))
                    .change("block_hash", event.block_hash.clone());

                entity_changes.push(events_entity_change);
            }
            SwapType(swap) => {
                let instruction = swap.instruction.unwrap();
                let mut events_entity_change =
                    EntityChange::new("Swap", id.as_str(), 0, entity_change::Operation::Create);

                events_entity_change
                    .change("amount", instruction.amount)
                    .change("other_amount_threshold", instruction.other_amount_threshold)
                    .change(
                        "sqrt_price_limit",
                        string_to_bigint(instruction.sqrt_price_limit.clone()),
                    )
                    .change(
                        "amount_specified_is_input",
                        instruction.amount_specified_is_input,
                    )
                    .change("a_to_b", instruction.a_to_b)
                    .change("slot", BigInt::from(event.slot))
                    .change("txn_id", event.txn_id.clone())
                    .change("block_height", BigInt::from(event.block_height))
                    .change("block_timestamp", BigInt::from(event.block_timestamp))
                    .change("block_hash", event.block_hash.clone());

                entity_changes.push(events_entity_change);
            }
        }
    }

    Ok(EntityChanges { entity_changes })
}
