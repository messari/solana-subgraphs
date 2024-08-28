use crate::traits::account_deserialize::AccountsDeserialize;

use borsh::{BorshDeserialize, BorshSerialize};
use derive_deserialize::AccountsDeserialize;
use substreams_solana::block_view::InstructionView;
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
