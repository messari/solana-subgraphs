use crate::traits::account_deserialize::AccountsDeserialize;

use borsh::{BorshDeserialize, BorshSerialize};
use derive_deserialize::AccountsDeserialize;
use substreams_solana::pb::sf::solana::r#type::v1::{CompiledInstruction, ConfirmedTransaction};
use substreams_solana::Address;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SwapInstruction {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
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
