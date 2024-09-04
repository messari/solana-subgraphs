use crate::constants;
use crate::orca_instructions::OrcaInstructions;
use crate::traits::balance_of::BalanceOf;
use crate::utils;
use crate::{
    instructions::{
        decrease_liquidity::{DecreaseLiquidityInstruction, DecreaseLiquidityInstructionAccounts},
        increase_liquidity::{IncreaseLiquidityInstruction, IncreaseLiquidityInstructionAccounts},
        initialize_pool::{InitializePoolInstruction, InitializePoolInstructionAccounts},
        initialize_pool_v2::{InitializePoolInstructionAccountsV2, InitializePoolInstructionV2},
        swap::{SwapInstruction, SwapInstructionAccounts},
        two_hop_swap::{TwoHopSwapInstruction, TwoHopSwapInstructionAccounts},
    },
    pb::messari::orca_whirlpool::v1::{
        decrease_liquidity, event::Type, increase_liquidity, initialize_pool, initialize_pool_v2,
        orca_swap, two_hop_swap, DecreaseLiquidity, Event, Events, IncreaseLiquidity,
        InitializePool, InitializePoolV2, OrcaSwap, TwoHopSwap,
    },
};
use substreams::skip_empty_output;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
    Address,
};

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Events, substreams::errors::Error> {
    skip_empty_output();

    let data: Vec<Event> = block
        .transactions()
        .flat_map(|confirmed_txn| process_txn(confirmed_txn, &block))
        .collect();

    Ok(Events { data })
}

fn process_txn(confirmed_txn: &ConfirmedTransaction, block: &Block) -> Vec<Event> {
    let all_instructions = confirmed_txn.walk_instructions().collect::<Vec<_>>();

    all_instructions
        .iter()
        .filter(|instr| instr.program_id() == constants::ORCA_WHIRLPOOL)
        .filter_map(|instr| {
            OrcaInstructions::from(instr).and_then(|decoded_instr| {
                process_instruction(decoded_instr, &all_instructions, confirmed_txn, block)
            })
        })
        .collect()
}

fn process_instruction(
    decoded_instr: OrcaInstructions,
    all_instructions: &[InstructionView],
    confirmed_txn: &ConfirmedTransaction,
    block: &Block,
) -> Option<Event> {
    let instr_type = match decoded_instr {
        OrcaInstructions::InitializePool(data, input_accounts) => {
            process_initialize_pool(data, input_accounts)
        }
        OrcaInstructions::InitializePoolV2(data, input_accounts) => {
            process_initialize_pool_v2(data, input_accounts)
        }
        OrcaInstructions::IncreaseLiquidity(data, input_accounts) => {
            process_increase_liquidity(data, input_accounts, confirmed_txn)
        }
        OrcaInstructions::DecreaseLiquidity(data, input_accounts) => {
            process_decrease_liquidity(data, input_accounts, confirmed_txn)
        }
        OrcaInstructions::TwoHopSwap(data, input_accounts) => {
            process_two_hop_swap(data, input_accounts, all_instructions, confirmed_txn)
        }
        OrcaInstructions::Swap(data, input_accounts) => {
            process_swap(data, input_accounts, confirmed_txn)
        }
    };

    instr_type.map(|r#type| Event {
        slot: block.slot,
        txn_id: confirmed_txn.id().clone(),
        block_height: block.block_height.clone().unwrap_or_default().block_height,
        block_timestamp: block.block_time.clone().unwrap_or_default().timestamp,
        block_hash: block.blockhash.clone(),
        r#type: Some(r#type),
    })
}

fn process_initialize_pool(
    data: InitializePoolInstruction,
    input_accounts: InitializePoolInstructionAccounts,
) -> Option<Type> {
    Some(Type::InitalizePool(InitializePool {
        instruction: Some(initialize_pool::Instruction {
            bumps: data.bumps.whirlpool_bump as u32,
            tick_spacing: data.tick_spacing as u32,
            initial_sqrt_price: data.initial_sqrt_price.to_string(),
        }),
        accounts: Some(initialize_pool::Accounts {
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

fn process_initialize_pool_v2(
    data: InitializePoolInstructionV2,
    input_accounts: InitializePoolInstructionAccountsV2,
) -> Option<Type> {
    Some(Type::InitalizePoolV2(InitializePoolV2 {
        instruction: Some(initialize_pool_v2::Instruction {
            tick_spacing: data.tick_spacing as u32,
            initial_sqrt_price: data.initial_sqrt_price.to_string(),
        }),
        accounts: Some(initialize_pool_v2::Accounts {
            whirlpools_config: input_accounts.whirlpools_config.to_string(),
            token_mint_a: input_accounts.token_mint_a.to_string(),
            token_mint_b: input_accounts.token_mint_b.to_string(),
            funder: input_accounts.funder.to_string(),
            whirlpool: input_accounts.whirlpool.to_string(),
            token_vault_a: input_accounts.token_vault_a.to_string(),
            token_vault_b: input_accounts.token_vault_b.to_string(),
            fee_tier: input_accounts.fee_tier.to_string(),
            token_program_a: input_accounts.token_program_a.to_string(),
            token_program_b: input_accounts.token_program_b.to_string(),
            system_program: input_accounts.system_program.to_string(),
            rent: input_accounts.rent.to_string(),
        }),
    }))
}

fn process_increase_liquidity(
    data: IncreaseLiquidityInstruction,
    input_accounts: IncreaseLiquidityInstructionAccounts,
    confirmed_txn: &ConfirmedTransaction,
) -> Option<Type> {
    let (token_a_pre_bal, token_a_post_bal) =
        confirmed_txn.balance_of(&input_accounts.whirlpool, &input_accounts.token_vault_a);
    let (token_b_pre_bal, token_b_post_bal) =
        confirmed_txn.balance_of(&input_accounts.whirlpool, &input_accounts.token_vault_b);

    Some(Type::IncreaseLiquidity(IncreaseLiquidity {
        instruction: Some(increase_liquidity::Instruction {
            liquidity_amount: data.liquidity_amount.to_string(),

            token_max_a: data.token_max_a.to_string(),
            token_max_b: data.token_max_b.to_string(),

            amount_a: utils::balance_difference(token_a_pre_bal.clone(), token_a_post_bal.clone()),
            amount_a_pre: token_a_pre_bal.clone(),
            amount_a_post: token_a_post_bal.clone(),

            amount_b: utils::balance_difference(token_b_pre_bal.clone(), token_b_post_bal.clone()),
            amount_b_pre: token_b_pre_bal.clone(),
            amount_b_post: token_b_post_bal.clone(),
        }),
        accounts: Some(increase_liquidity::Accounts {
            whirlpool: input_accounts.whirlpool.to_string(),
            token_program: input_accounts.token_program.to_string(),
            position_authority: input_accounts.position_authority.to_string(),
            position: input_accounts.position.to_string(),
            position_token_account: input_accounts.position_token_account.to_string(),
            token_owner_account_a: input_accounts.token_owner_account_a.to_string(),
            token_owner_account_b: input_accounts.token_owner_account_b.to_string(),
            token_vault_a: input_accounts.token_vault_a.to_string(),
            token_vault_b: input_accounts.token_vault_b.to_string(),
            tick_array_lower: input_accounts.tick_array_lower.to_string(),
            tick_array_upper: input_accounts.tick_array_upper.to_string(),
        }),
    }))
}

fn process_decrease_liquidity(
    data: DecreaseLiquidityInstruction,
    input_accounts: DecreaseLiquidityInstructionAccounts,
    confirmed_txn: &ConfirmedTransaction,
) -> Option<Type> {
    let (token_a_pre_bal, token_a_post_bal) =
        confirmed_txn.balance_of(&input_accounts.whirlpool, &input_accounts.token_vault_a);
    let (token_b_pre_bal, token_b_post_bal) =
        confirmed_txn.balance_of(&input_accounts.whirlpool, &input_accounts.token_vault_b);

    Some(Type::DecreaseLiquidity(DecreaseLiquidity {
        instruction: Some(decrease_liquidity::Instruction {
            liquidity_amount: data.liquidity_amount.to_string(),

            token_min_a: data.token_min_a.to_string(),
            token_min_b: data.token_min_b.to_string(),

            amount_a: utils::balance_difference(token_a_pre_bal.clone(), token_a_post_bal.clone()),
            amount_a_pre: token_a_pre_bal.clone(),
            amount_a_post: token_a_post_bal.clone(),

            amount_b: utils::balance_difference(token_b_pre_bal.clone(), token_b_post_bal.clone()),
            amount_b_pre: token_b_pre_bal.clone(),
            amount_b_post: token_b_post_bal.clone(),
        }),
        accounts: Some(decrease_liquidity::Accounts {
            whirlpool: input_accounts.whirlpool.to_string(),
            token_program: input_accounts.token_program.to_string(),
            position_authority: input_accounts.position_authority.to_string(),
            position: input_accounts.position.to_string(),
            position_token_account: input_accounts.position_token_account.to_string(),
            token_owner_account_a: input_accounts.token_owner_account_a.to_string(),
            token_owner_account_b: input_accounts.token_owner_account_b.to_string(),
            token_vault_a: input_accounts.token_vault_a.to_string(),
            token_vault_b: input_accounts.token_vault_b.to_string(),
            tick_array_lower: input_accounts.tick_array_lower.to_string(),
            tick_array_upper: input_accounts.tick_array_upper.to_string(),
        }),
    }))
}

fn process_two_hop_swap(
    data: TwoHopSwapInstruction,
    input_accounts: TwoHopSwapInstructionAccounts,
    all_instructions: &[InstructionView],
    confirmed_txn: &ConfirmedTransaction,
) -> Option<Type> {
    let transfer_a_one_instr_addresses = get_transfer_addresses(
        data.a_to_b_one,
        &input_accounts.token_owner_account_one_a,
        &input_accounts.token_vault_one_a,
        &input_accounts.token_authority,
        &input_accounts.whirlpool_one,
    );
    let transfer_b_one_instr_addresses = get_transfer_addresses(
        !data.a_to_b_one,
        &input_accounts.token_owner_account_one_b,
        &input_accounts.token_vault_one_b,
        &input_accounts.token_authority,
        &input_accounts.whirlpool_one,
    );
    let transfer_a_two_instr_addresses = get_transfer_addresses(
        data.a_to_b_two,
        &input_accounts.token_owner_account_two_a,
        &input_accounts.token_vault_two_a,
        &input_accounts.token_authority,
        &input_accounts.whirlpool_two,
    );
    let transfer_b_two_instr_addresses = get_transfer_addresses(
        !data.a_to_b_two,
        &input_accounts.token_owner_account_two_b,
        &input_accounts.token_vault_two_b,
        &input_accounts.token_authority,
        &input_accounts.whirlpool_two,
    );

    let transfer_token_a_one_instruction =
        find_transfer_instruction(all_instructions, &transfer_a_one_instr_addresses);
    let transfer_token_b_one_instruction =
        find_transfer_instruction(all_instructions, &transfer_b_one_instr_addresses);
    let transfer_token_a_two_instruction =
        find_transfer_instruction(all_instructions, &transfer_a_two_instr_addresses);
    let transfer_token_b_two_instruction =
        find_transfer_instruction(all_instructions, &transfer_b_two_instr_addresses);

    let (token_a_one_pre_bal, token_a_one_post_bal) = confirmed_txn.balance_of(
        &input_accounts.whirlpool_one,
        &input_accounts.token_vault_one_a,
    );
    let (token_b_one_pre_bal, token_b_one_post_bal) = confirmed_txn.balance_of(
        &input_accounts.whirlpool_one,
        &input_accounts.token_vault_one_b,
    );
    let (token_a_two_pre_bal, token_a_two_post_bal) = confirmed_txn.balance_of(
        &input_accounts.whirlpool_two,
        &input_accounts.token_vault_two_a,
    );
    let (token_b_two_pre_bal, token_b_two_post_bal) = confirmed_txn.balance_of(
        &input_accounts.whirlpool_two,
        &input_accounts.token_vault_two_b,
    );

    Some(Type::TwoHopSwap(TwoHopSwap {
        instruction: Some(two_hop_swap::Instruction {
            amount: data.amount.to_string(),

            amount_a_one: utils::get_transfer_amount(transfer_token_a_one_instruction),
            amount_b_one: utils::get_transfer_amount(transfer_token_b_one_instruction),

            amount_a_one_pre: token_a_one_pre_bal.clone(),
            amount_a_one_post: token_a_one_post_bal.clone(),

            amount_b_one_pre: token_b_one_pre_bal.clone(),
            amount_b_one_post: token_b_one_post_bal.clone(),

            amount_a_two: utils::get_transfer_amount(transfer_token_a_two_instruction),
            amount_b_two: utils::get_transfer_amount(transfer_token_b_two_instruction),

            amount_a_two_pre: token_a_two_pre_bal.clone(),
            amount_a_two_post: token_a_two_post_bal.clone(),

            amount_b_two_pre: token_b_two_pre_bal.clone(),
            amount_b_two_post: token_b_two_post_bal.clone(),

            other_amount_threshold: data.other_amount_threshold.to_string(),

            amount_specified_is_input: data.amount_specified_is_input,
            a_to_b_one: data.a_to_b_one,
            a_to_b_two: data.a_to_b_two,

            sqrt_price_limit_one: data.sqrt_price_limit_one.to_string(),
            sqrt_price_limit_two: data.sqrt_price_limit_two.to_string(),
        }),
        accounts: Some(two_hop_swap::Accounts {
            token_program: input_accounts.token_program.to_string(),
            token_authority: input_accounts.token_authority.to_string(),
            whirlpool_one: input_accounts.whirlpool_one.to_string(),
            whirlpool_two: input_accounts.whirlpool_two.to_string(),
            token_owner_account_one_a: input_accounts.token_owner_account_one_a.to_string(),
            token_vault_one_a: input_accounts.token_vault_one_a.to_string(),
            token_owner_account_one_b: input_accounts.token_owner_account_one_b.to_string(),
            token_vault_one_b: input_accounts.token_vault_one_b.to_string(),
            token_owner_account_two_a: input_accounts.token_owner_account_two_a.to_string(),
            token_vault_two_a: input_accounts.token_vault_two_a.to_string(),
            token_owner_account_two_b: input_accounts.token_owner_account_two_b.to_string(),
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

fn process_swap(
    data: SwapInstruction,
    input_accounts: SwapInstructionAccounts,
    confirmed_txn: &ConfirmedTransaction,
) -> Option<Type> {
    let (token_a_pre_bal, token_a_post_bal) =
        confirmed_txn.balance_of(&input_accounts.whirlpool, &input_accounts.token_vault_a);
    let (token_b_pre_bal, token_b_post_bal) =
        confirmed_txn.balance_of(&input_accounts.whirlpool, &input_accounts.token_vault_b);

    Some(Type::Swap(OrcaSwap {
        instruction: Some(orca_swap::Instruction {
            amount: data.amount.to_string(),

            amount_a: utils::balance_difference(token_a_pre_bal.clone(), token_a_post_bal.clone()),
            amount_a_pre: token_a_pre_bal.clone(),
            amount_a_post: token_a_post_bal.clone(),

            amount_b: utils::balance_difference(token_b_pre_bal.clone(), token_b_post_bal.clone()),
            amount_b_pre: token_b_pre_bal.clone(),
            amount_b_post: token_b_post_bal.clone(),

            other_amount_threshold: data.other_amount_threshold.to_string(),
            sqrt_price_limit: data.sqrt_price_limit.to_string(),

            amount_specified_is_input: data.amount_specified_is_input,
            a_to_b: data.a_to_b,
        }),
        accounts: Some(orca_swap::Accounts {
            token_program: input_accounts.token_program.to_string(),
            token_authority: input_accounts.token_authority.to_string(),
            whirlpool: input_accounts.whirlpool.to_string(),
            token_owner_account_a: input_accounts.token_owner_account_a.to_string(),
            token_vault_a: input_accounts.token_vault_a.to_string(),
            token_owner_account_b: input_accounts.token_owner_account_b.to_string(),
            token_vault_b: input_accounts.token_vault_b.to_string(),
            tick_array_0: input_accounts.tick_array_0.to_string(),
            tick_array_1: input_accounts.tick_array_1.to_string(),
            tick_array_2: input_accounts.tick_array_2.to_string(),
            oracle: input_accounts.oracle.to_string(),
        }),
    }))
}

fn find_transfer_instruction<'a>(
    all_instructions: &'a [InstructionView],
    accounts: &[&'a Address<'a>],
) -> Option<&'a InstructionView<'a>> {
    all_instructions.iter().find(|iv| {
        iv.accounts()
            .iter()
            .map(|acc| acc.to_string())
            .eq(accounts.iter().map(|s| s.to_string()))
    })
}

fn get_transfer_addresses<'a>(
    is_a_to_b: bool,
    owner_account: &'a Address<'a>,
    vault_account: &'a Address<'a>,
    authority: &'a Address<'a>,
    whirlpool: &'a Address<'a>,
) -> Vec<&'a Address<'a>> {
    if is_a_to_b {
        vec![owner_account, vault_account, authority]
    } else {
        vec![vault_account, owner_account, whirlpool]
    }
}
