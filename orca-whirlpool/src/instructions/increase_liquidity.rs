use crate::traits::account_deserialize::AccountsDeserialize;

use borsh::{BorshDeserialize, BorshSerialize};
use derive_deserialize::AccountsDeserialize;
use substreams_solana::pb::sf::solana::r#type::v1::{CompiledInstruction, ConfirmedTransaction};
use substreams_solana::Address;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct IncreaseLiquidityInstruction {
    pub liquidity_amount: u128,
    pub token_max_a: u64,
    pub token_max_b: u64,
}

#[derive(AccountsDeserialize, Debug)]
pub struct IncreaseLiquidityInstructionAccounts<'a> {
    pub token_program: Address<'a>,
    pub whirlpool: Address<'a>,
    pub position_authority: Address<'a>,
    pub position: Address<'a>,
    pub position_token_account: Address<'a>,
    pub token_owner_account_a: Address<'a>,
    pub token_owner_account_b: Address<'a>,
    pub token_vault_a: Address<'a>,
    pub token_vault_b: Address<'a>,
    pub tick_array_lower: Address<'a>,
    pub tick_array_upper: Address<'a>,
}
