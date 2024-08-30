use crate::constants;
use crate::orca_instructions::OrcaInstructions;
use crate::pb::messari::orca_whirlpool::v1::decrease_liquidity::{
    DecreaseLiquidityInstruction, DecreaseLiquidityInstructionAccounts,
};
use crate::pb::messari::orca_whirlpool::v1::event::Type::{
    self, DecreaseLiquidity as DecreaseLiquidityType, IncreaseLiquidity as IncreaseLiquidityType,
    InitalizePool as InitalizePoolType, Swap as SwapType, TwoHopSwap as TwoHopSwapType,
};
use crate::pb::messari::orca_whirlpool::v1::increase_liquidity::{
    IncreaseLiquidityInstruction, IncreaseLiquidityInstructionAccounts,
};
use crate::pb::messari::orca_whirlpool::v1::initialize_pool::{
    InitializePoolInstruction, InitializePoolInstructionAccounts,
};
use crate::pb::messari::orca_whirlpool::v1::swap::{SwapInstruction, SwapInstructionAccounts};
use crate::pb::messari::orca_whirlpool::v1::two_hop_swap::{
    TwoHopSwapInstruction, TwoHopSwapInstructionAccounts,
};
use crate::pb::messari::orca_whirlpool::v1::{
    DecreaseLiquidity, Event, Events, IncreaseLiquidity, InitializePool, Swap, TwoHopSwap,
};
use crate::utils::get_transfer_amount;
use substreams::skip_empty_output;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Events, substreams::errors::Error> {
    skip_empty_output();

    let mut data: Vec<Event> = Vec::new();

    for confirmed_txn in block.transactions() {
        let all_instructions = confirmed_txn
            .walk_instructions()
            .into_iter()
            .collect::<Vec<_>>();

        for instruction in &all_instructions {
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
                        let transfer_token_a_instruction = all_instructions.iter().find(|iv| {
                            iv.accounts().eq(&vec![
                                &input_accounts.token_owner_account_a,
                                &input_accounts.token_vault_a,
                                &input_accounts.position_authority,
                            ])
                        });

                        let transfer_token_b_instruction = all_instructions.iter().find(|iv| {
                            iv.accounts().eq(&vec![
                                &input_accounts.token_owner_account_b,
                                &input_accounts.token_vault_b,
                                &input_accounts.position_authority,
                            ])
                        });

                        instruction_data = Some(IncreaseLiquidityType(IncreaseLiquidity {
                            instruction: Some(IncreaseLiquidityInstruction {
                                liquidity_amount: data.liquidity_amount.to_string(),
                                amount_a: get_transfer_amount(transfer_token_a_instruction),
                                token_max_a: data.token_max_a,
                                amount_b: get_transfer_amount(transfer_token_b_instruction),
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
                        let transfer_token_a_instruction = all_instructions.iter().find(|iv| {
                            iv.accounts().eq(&vec![
                                &input_accounts.token_vault_a,
                                &input_accounts.token_owner_account_a,
                                &input_accounts.whirlpool,
                            ])
                        });

                        let transfer_token_b_instruction = all_instructions.iter().find(|iv| {
                            iv.accounts().eq(&vec![
                                &input_accounts.token_vault_b,
                                &input_accounts.token_owner_account_b,
                                &input_accounts.whirlpool,
                            ])
                        });

                        instruction_data = Some(DecreaseLiquidityType(DecreaseLiquidity {
                            instruction: Some(DecreaseLiquidityInstruction {
                                liquidity_amount: data.liquidity_amount.to_string(),
                                amount_a: get_transfer_amount(transfer_token_a_instruction),
                                token_min_a: data.token_min_a,
                                amount_b: get_transfer_amount(transfer_token_b_instruction),
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
                        let transfer_a_one_instr_addresses;
                        let transfer_b_one_instr_addresses;
                        let transfer_a_two_instr_addresses;
                        let transfer_b_two_instr_addresses;

                        if data.a_to_b_one {
                            transfer_a_one_instr_addresses = vec![
                                &input_accounts.token_owner_account_one_a,
                                &input_accounts.token_vault_one_a,
                                &input_accounts.token_authority,
                            ];

                            transfer_b_one_instr_addresses = vec![
                                &input_accounts.token_vault_one_b,
                                &input_accounts.token_owner_account_one_b,
                                &input_accounts.whirlpool_one,
                            ];
                        } else {
                            transfer_a_one_instr_addresses = vec![
                                &input_accounts.token_vault_one_a,
                                &input_accounts.token_owner_account_one_a,
                                &input_accounts.whirlpool_one,
                            ];

                            transfer_b_one_instr_addresses = vec![
                                &input_accounts.token_owner_account_one_b,
                                &input_accounts.token_vault_one_b,
                                &input_accounts.token_authority,
                            ];
                        }

                        if data.a_to_b_two {
                            transfer_a_two_instr_addresses = vec![
                                &input_accounts.token_owner_account_two_a,
                                &input_accounts.token_vault_two_a,
                                &input_accounts.token_authority,
                            ];

                            transfer_b_two_instr_addresses = vec![
                                &input_accounts.token_vault_two_b,
                                &input_accounts.token_owner_account_two_b,
                                &input_accounts.whirlpool_two,
                            ];
                        } else {
                            transfer_a_two_instr_addresses = vec![
                                &input_accounts.token_vault_two_a,
                                &input_accounts.token_owner_account_two_a,
                                &input_accounts.whirlpool_two,
                            ];

                            transfer_b_two_instr_addresses = vec![
                                &input_accounts.token_owner_account_two_b,
                                &input_accounts.token_vault_two_b,
                                &input_accounts.token_authority,
                            ];
                        }

                        let transfer_token_a_one_instruction = all_instructions
                            .iter()
                            .find(|iv| iv.accounts().eq(&transfer_a_one_instr_addresses));
                        let transfer_token_b_one_instruction = all_instructions
                            .iter()
                            .find(|iv| iv.accounts().eq(&transfer_b_one_instr_addresses));

                        let transfer_token_a_two_instruction = all_instructions
                            .iter()
                            .find(|iv| iv.accounts().eq(&transfer_a_two_instr_addresses));
                        let transfer_token_b_two_instruction = all_instructions
                            .iter()
                            .find(|iv| iv.accounts().eq(&transfer_b_two_instr_addresses));

                        instruction_data = Some(TwoHopSwapType(TwoHopSwap {
                            instruction: Some(TwoHopSwapInstruction {
                                amount: data.amount,
                                amount_a_one: get_transfer_amount(transfer_token_a_one_instruction),
                                amount_b_one: get_transfer_amount(transfer_token_b_one_instruction),
                                amount_a_two: get_transfer_amount(transfer_token_a_two_instruction),
                                amount_b_two: get_transfer_amount(transfer_token_b_two_instruction),
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
                        let transfer_a_instr_addresses;
                        let transfer_b_instr_addresses;

                        if data.a_to_b {
                            transfer_a_instr_addresses = vec![
                                &input_accounts.token_owner_account_a,
                                &input_accounts.token_vault_a,
                                &input_accounts.token_authority,
                            ];

                            transfer_b_instr_addresses = vec![
                                &input_accounts.token_vault_b,
                                &input_accounts.token_owner_account_b,
                                &input_accounts.whirlpool,
                            ];
                        } else {
                            transfer_a_instr_addresses = vec![
                                &input_accounts.token_vault_a,
                                &input_accounts.token_owner_account_a,
                                &input_accounts.whirlpool,
                            ];

                            transfer_b_instr_addresses = vec![
                                &input_accounts.token_owner_account_b,
                                &input_accounts.token_vault_b,
                                &input_accounts.token_authority,
                            ];
                        }

                        let transfer_token_a_instruction = all_instructions
                            .iter()
                            .find(|iv| iv.accounts().eq(&transfer_a_instr_addresses));

                        let transfer_token_b_instruction = all_instructions
                            .iter()
                            .find(|iv| iv.accounts().eq(&transfer_b_instr_addresses));

                        instruction_data = Some(SwapType(Swap {
                            instruction: Some(SwapInstruction {
                                amount: data.amount,
                                other_amount_threshold: data.other_amount_threshold,
                                sqrt_price_limit: data.sqrt_price_limit.to_string(),
                                amount_specified_is_input: data.amount_specified_is_input,
                                amount_a: get_transfer_amount(transfer_token_a_instruction),
                                amount_b: get_transfer_amount(transfer_token_b_instruction),
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
