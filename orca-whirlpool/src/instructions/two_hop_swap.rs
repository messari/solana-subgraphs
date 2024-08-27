use crate::traits::account_deserialize::AccountsDeserialize;

use borsh::{BorshDeserialize, BorshSerialize};
use derive_deserialize::AccountsDeserialize;
use substreams_solana::pb::sf::solana::r#type::v1::{CompiledInstruction, ConfirmedTransaction};
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
