mod constants;
mod instructions;
mod orca_instructions;
mod pb;
mod traits;
mod utils;

use orca_instructions::OrcaInstructions;
use pb::messari::orca_whirlpool::v1::decrease_liquidity::{
    DecreaseLiquidityInstruction, DecreaseLiquidityInstructionAccounts,
};
use pb::messari::orca_whirlpool::v1::event::Type::{
    self, DecreaseLiquidity as DecreaseLiquidityType, IncreaseLiquidity as IncreaseLiquidityType,
    InitalizePool as InitalizePoolType, Swap as SwapType, TwoHopSwap as TwoHopSwapType,
};
use pb::messari::orca_whirlpool::v1::increase_liquidity::{
    IncreaseLiquidityInstruction, IncreaseLiquidityInstructionAccounts,
};
use pb::messari::orca_whirlpool::v1::initialize_pool::{
    InitializePoolInstruction, InitializePoolInstructionAccounts,
};
use pb::messari::orca_whirlpool::v1::swap::{SwapInstruction, SwapInstructionAccounts};
use pb::messari::orca_whirlpool::v1::two_hop_swap::{
    TwoHopSwapInstruction, TwoHopSwapInstructionAccounts,
};
use pb::messari::orca_whirlpool::v1::{
    DecreaseLiquidity, Event, Events, IncreaseLiquidity, InitializePool, Swap, TwoHopSwap,
};
use substreams::scalar::BigInt;
use substreams::skip_empty_output;
use substreams_entity_change::pb::entity::{entity_change, EntityChange, EntityChanges};
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use utils::string_to_bigint;

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Events, substreams::errors::Error> {
    skip_empty_output();

    let mut data: Vec<Event> = Vec::new();

    for confirmed_txn in block.transactions() {
        for instruction in confirmed_txn.walk_instructions() {
            if instruction.program_id() != constants::ORCA_WHIRLPOOL {
                continue;
            }

            let mut instruction_data: Option<Type> = None;

            if let Some(decoded_instruction) = OrcaInstructions::from(&instruction) {
                match decoded_instruction {
                    OrcaInstructions::InitializePool {
                        data,
                        input_accounts,
                    } => {
                        instruction_data = Some(InitalizePoolType(InitializePool {
                            instruction: Some(InitializePoolInstruction {
                                bumps: data.bumps.whirlpool_bump as u32,
                                tick_spacing: data.tick_spacing as u32,
                                initial_sqrt_price: data.initial_sqrt_price.to_string(),
                            }),
                            accounts: Some(InitializePoolInstructionAccounts {
                                whirlpools_config: input_accounts.whirlpools_config.to_string(),
                                token_mint_a: input_accounts.token_mint_a.to_string(),
                                token_mint_b: input_accounts.token_mint_b.to_string(),
                                funder: input_accounts.funder.to_string(),
                                whirlpool: input_accounts.whirlpool.to_string(),
                                token_vault_a: input_accounts.token_vault_a.to_string(),
                                token_vault_b: input_accounts.token_vault_b.to_string(),
                                fee_tier: input_accounts.fee_tier.to_string(),
                                token_program: input_accounts.token_program.to_string(),
                                system_program: input_accounts.system_program.to_string(),
                                rent: input_accounts.rent.to_string(),
                            }),
                        }))
                    }
                    OrcaInstructions::IncreaseLiquidity {
                        data,
                        input_accounts,
                    } => {
                        instruction_data = Some(IncreaseLiquidityType(IncreaseLiquidity {
                            instruction: Some(IncreaseLiquidityInstruction {
                                liquidity_amount: data.liquidity_amount.to_string(),
                                token_max_a: data.token_max_a,
                                token_max_b: data.token_max_b,
                            }),
                            accounts: Some(IncreaseLiquidityInstructionAccounts {
                                whirlpool: input_accounts.whirlpool.to_string(),
                                token_program: input_accounts.token_program.to_string(),
                                position_authority: input_accounts.position_authority.to_string(),
                                position: input_accounts.position.to_string(),
                                position_token_account: input_accounts
                                    .position_token_account
                                    .to_string(),
                                token_owner_account_a: input_accounts
                                    .token_owner_account_a
                                    .to_string(),
                                token_owner_account_b: input_accounts
                                    .token_owner_account_b
                                    .to_string(),
                                token_vault_a: input_accounts.token_vault_a.to_string(),
                                token_vault_b: input_accounts.token_vault_b.to_string(),
                                tick_array_lower: input_accounts.tick_array_lower.to_string(),
                                tick_array_upper: input_accounts.tick_array_upper.to_string(),
                            }),
                        }))
                    }
                    OrcaInstructions::DecreaseLiquidity {
                        data,
                        input_accounts,
                    } => {
                        instruction_data = Some(DecreaseLiquidityType(DecreaseLiquidity {
                            instruction: Some(DecreaseLiquidityInstruction {
                                liquidity_amount: data.liquidity_amount.to_string(),
                                token_min_a: data.token_min_a,
                                token_min_b: data.token_min_b,
                            }),
                            accounts: Some(DecreaseLiquidityInstructionAccounts {
                                whirlpool: input_accounts.whirlpool.to_string(),
                                token_program: input_accounts.token_program.to_string(),
                                position_authority: input_accounts.position_authority.to_string(),
                                position: input_accounts.position.to_string(),
                                position_token_account: input_accounts
                                    .position_token_account
                                    .to_string(),
                                token_owner_account_a: input_accounts
                                    .token_owner_account_a
                                    .to_string(),
                                token_owner_account_b: input_accounts
                                    .token_owner_account_b
                                    .to_string(),
                                token_vault_a: input_accounts.token_vault_a.to_string(),
                                token_vault_b: input_accounts.token_vault_b.to_string(),
                                tick_array_lower: input_accounts.tick_array_lower.to_string(),
                                tick_array_upper: input_accounts.tick_array_upper.to_string(),
                            }),
                        }))
                    }
                    OrcaInstructions::TwoHopSwap {
                        data,
                        input_accounts,
                    } => {
                        instruction_data = Some(TwoHopSwapType(TwoHopSwap {
                            instruction: Some(TwoHopSwapInstruction {
                                amount: data.amount,
                                other_amount_threshold: data.other_amount_threshold,
                                amount_specified_is_input: data.amount_specified_is_input,
                                a_to_b_one: data.a_to_b_one,
                                a_to_b_two: data.a_to_b_two,
                                sqrt_price_limit_one: data.sqrt_price_limit_one.to_string(),
                                sqrt_price_limit_two: data.sqrt_price_limit_two.to_string(),
                            }),
                            accounts: Some(TwoHopSwapInstructionAccounts {
                                token_program: input_accounts.token_program.to_string(),
                                token_authority: input_accounts.token_authority.to_string(),
                                whirlpool_one: input_accounts.whirlpool_one.to_string(),
                                whirlpool_two: input_accounts.whirlpool_two.to_string(),
                                token_owner_account_one_a: input_accounts
                                    .token_owner_account_one_a
                                    .to_string(),
                                token_vault_one_a: input_accounts.token_vault_one_a.to_string(),
                                token_owner_account_one_b: input_accounts
                                    .token_owner_account_one_b
                                    .to_string(),
                                token_vault_one_b: input_accounts.token_vault_one_b.to_string(),
                                token_owner_account_two_a: input_accounts
                                    .token_owner_account_two_a
                                    .to_string(),
                                token_vault_two_a: input_accounts.token_vault_two_a.to_string(),
                                token_owner_account_two_b: input_accounts
                                    .token_owner_account_two_b
                                    .to_string(),
                                token_vault_two_b: input_accounts.token_vault_two_b.to_string(),
                                tick_array_one0: input_accounts.tick_array_one0.to_string(),
                                tick_array_one1: input_accounts.tick_array_one1.to_string(),
                                tick_array_one2: input_accounts.tick_array_one2.to_string(),
                                tick_array_two0: input_accounts.tick_array_two0.to_string(),
                                tick_array_two1: input_accounts.tick_array_two1.to_string(),
                                tick_array_two2: input_accounts.tick_array_two2.to_string(),
                                oracle_one: input_accounts.oracle_one.to_string(),
                                oracle_two: input_accounts.oracle_two.to_string(),
                            }),
                        }))
                    }
                    OrcaInstructions::Swap {
                        data,
                        input_accounts,
                    } => {
                        instruction_data = Some(SwapType(Swap {
                            instruction: Some(SwapInstruction {
                                amount: data.amount,
                                other_amount_threshold: data.other_amount_threshold,
                                sqrt_price_limit: data.sqrt_price_limit.to_string(),
                                amount_specified_is_input: data.amount_specified_is_input,
                                a_to_b: data.a_to_b,
                            }),
                            accounts: Some(SwapInstructionAccounts {
                                token_program: input_accounts.token_program.to_string(),
                                token_authority: input_accounts.token_authority.to_string(),
                                whirlpool: input_accounts.whirlpool.to_string(),
                                token_owner_account_a: input_accounts
                                    .token_owner_account_a
                                    .to_string(),
                                token_vault_a: input_accounts.token_vault_a.to_string(),
                                token_owner_account_b: input_accounts
                                    .token_owner_account_b
                                    .to_string(),
                                token_vault_b: input_accounts.token_vault_b.to_string(),
                                tick_array_0: input_accounts.tick_array_0.to_string(),
                                tick_array_1: input_accounts.tick_array_1.to_string(),
                                tick_array_2: input_accounts.tick_array_2.to_string(),
                                oracle: input_accounts.oracle.to_string(),
                            }),
                        }))
                    }
                }
            };

            if instruction_data.is_none() {
                continue;
            }

            data.push(Event {
                slot: block.slot.clone(),
                txn_id: confirmed_txn.id().clone(),
                block_height: block.block_height.clone().unwrap_or_default().block_height,
                block_timestamp: block.block_time.clone().unwrap_or_default().timestamp,
                block_hash: block.blockhash.clone(),
                r#type: instruction_data,
            });
        }
    }

    Ok(Events { data })
}

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
