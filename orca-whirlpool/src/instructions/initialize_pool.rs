use crate::traits::account_deserialize::AccountsDeserialize;

use borsh::{BorshDeserialize, BorshSerialize};
use derive_deserialize::AccountsDeserialize;
use substreams_solana::pb::sf::solana::r#type::v1::{CompiledInstruction, ConfirmedTransaction};
use substreams_solana::Address;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct WhirlpoolBumps {
    pub whirlpool_bump: u8,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct InitializePoolInstruction {
    // The bump value when deriving the PDA of the Whirlpool address.
    pub bumps: WhirlpoolBumps,
    // The desired tick spacing for this pool.
    pub tick_spacing: u16,
    // The desired initial sqrt-price for this pool.
    pub initial_sqrt_price: u128,
}

#[derive(AccountsDeserialize, Debug)]
pub struct InitializePoolInstructionAccounts<'a> {
    pub whirlpools_config: Address<'a>,
    pub token_mint_a: Address<'a>,
    pub token_mint_b: Address<'a>,
    pub funder: Address<'a>,
    pub whirlpool: Address<'a>,
    pub token_vault_a: Address<'a>,
    pub token_vault_b: Address<'a>,
    pub fee_tier: Address<'a>,
    pub token_program: Address<'a>,
    pub system_program: Address<'a>,
    pub rent: Address<'a>,
}
