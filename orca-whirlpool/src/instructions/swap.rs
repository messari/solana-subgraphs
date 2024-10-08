use crate::pb::messari::orca_whirlpool::v1::event::Type;
use crate::pb::messari::orca_whirlpool::v1::{orca_swap, OrcaSwap};
use crate::traits::account_deserialize::AccountsDeserialize;
use crate::traits::balance_of::BalanceOf;
use crate::utils;

use borsh::{BorshDeserialize, BorshSerialize};
use derive_deserialize::AccountsDeserialize;
use substreams_solana::block_view::InstructionView;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;
use substreams_solana::Address;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SwapInstruction {
    // The amount of input or output token to swap from (depending on amount_specified_is_input).
    pub amount: u64,
    // The maximum/minimum of input/output token to swap into (depending on amount_specified_is_input).
    pub other_amount_threshold: u64,
    // The maximum/minimum price the swap will swap to.
    pub sqrt_price_limit: u128,
    // Specifies the token the parameter `amount`represents. If true, the amount represents the input token of the swap.
    pub amount_specified_is_input: bool,
    // The direction of the swap. True if swapping from A to B. False if swapping from B to A.
    pub a_to_b: bool,
}

#[derive(AccountsDeserialize, Debug)]
pub struct SwapInstructionAccounts<'a> {
    pub token_program: Address<'a>,
    pub token_authority: Address<'a>,
    pub whirlpool: Address<'a>,
    pub token_owner_account_a: Address<'a>,
    pub token_vault_a: Address<'a>,
    pub token_owner_account_b: Address<'a>,
    pub token_vault_b: Address<'a>,
    pub tick_array_0: Address<'a>,
    pub tick_array_1: Address<'a>,
    pub tick_array_2: Address<'a>,
    pub oracle: Address<'a>,
}

pub fn process_swap(
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
