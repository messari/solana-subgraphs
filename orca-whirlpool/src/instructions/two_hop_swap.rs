use crate::pb::messari::orca_whirlpool::v1::event::Type;
use crate::pb::messari::orca_whirlpool::v1::{two_hop_swap, TwoHopSwap};
use crate::traits::account_deserialize::AccountsDeserialize;
use crate::traits::balance_of::BalanceOf;
use crate::utils;

use borsh::{BorshDeserialize, BorshSerialize};
use derive_deserialize::AccountsDeserialize;
use substreams_solana::block_view::InstructionView;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;
use substreams_solana::Address;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct TwoHopSwapInstruction {
    /// The amount of input or output token to swap from (depending on amount_specified_is_input).
    pub amount: u64,
    /// The maximum/minimum of input/output token to swap into (depending on amount_specified_is_input).
    pub other_amount_threshold: u64,
    /// Specifies the token the parameter `amount`represents. If true, the amount represents the input token of the swap.
    pub amount_specified_is_input: bool,
    /// The direction of the swap of hop one. True if swapping from A to B. False if swapping from B to A.
    pub a_to_b_one: bool,
    /// The direction of the swap of hop two. True if swapping from A to B. False if swapping from B to A.
    pub a_to_b_two: bool,
    /// The maximum/minimum price the swap will swap to in the first hop.
    pub sqrt_price_limit_one: u128,
    /// The maximum/minimum price the swap will swap to in the second hop.
    pub sqrt_price_limit_two: u128,
}

#[derive(AccountsDeserialize, Debug)]
pub struct TwoHopSwapInstructionAccounts<'a> {
    pub token_program: Address<'a>,
    pub token_authority: Address<'a>,
    pub whirlpool_one: Address<'a>,
    pub whirlpool_two: Address<'a>,
    pub token_owner_account_one_a: Address<'a>,
    pub token_vault_one_a: Address<'a>,
    pub token_owner_account_one_b: Address<'a>,
    pub token_vault_one_b: Address<'a>,
    pub token_owner_account_two_a: Address<'a>,
    pub token_vault_two_a: Address<'a>,
    pub token_owner_account_two_b: Address<'a>,
    pub token_vault_two_b: Address<'a>,
    pub tick_array_one0: Address<'a>,
    pub tick_array_one1: Address<'a>,
    pub tick_array_one2: Address<'a>,
    pub tick_array_two0: Address<'a>,
    pub tick_array_two1: Address<'a>,
    pub tick_array_two2: Address<'a>,
    pub oracle_one: Address<'a>,
    pub oracle_two: Address<'a>,
}

pub fn process_two_hop_swap(
    data: TwoHopSwapInstruction,
    input_accounts: TwoHopSwapInstructionAccounts,
    confirmed_txn: &ConfirmedTransaction,
) -> Option<Type> {
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

            amount_a_one: utils::balance_difference(
                token_a_one_pre_bal.clone(),
                token_a_one_post_bal.clone(),
            ),
            amount_b_one: utils::balance_difference(
                token_b_one_pre_bal.clone(),
                token_b_one_post_bal.clone(),
            ),

            amount_a_one_pre: token_a_one_pre_bal.clone(),
            amount_a_one_post: token_a_one_post_bal.clone(),

            amount_b_one_pre: token_b_one_pre_bal.clone(),
            amount_b_one_post: token_b_one_post_bal.clone(),

            amount_a_two: utils::balance_difference(
                token_a_two_pre_bal.clone(),
                token_a_two_post_bal.clone(),
            ),
            amount_b_two: utils::balance_difference(
                token_b_two_pre_bal.clone(),
                token_b_two_post_bal.clone(),
            ),

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
