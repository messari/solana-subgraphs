use crate::pb::messari::orca_whirlpool::v1::event::Type;
use crate::pb::messari::orca_whirlpool::v1::{initialize_pool_v2, InitializePoolV2};
use crate::traits::account_deserialize::AccountsDeserialize;

use borsh::{BorshDeserialize, BorshSerialize};
use derive_deserialize::AccountsDeserialize;
use substreams_solana::block_view::InstructionView;
use substreams_solana::Address;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct WhirlpoolBumps {
    pub whirlpool_bump: u8,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct InitializePoolInstructionV2 {
    // The desired tick spacing for this pool.
    pub tick_spacing: u16,
    // The desired initial sqrt-price for this pool.
    pub initial_sqrt_price: u128,
}

#[derive(AccountsDeserialize, Debug)]
pub struct InitializePoolInstructionAccountsV2<'a> {
    pub whirlpools_config: Address<'a>,
    pub token_mint_a: Address<'a>,
    pub token_mint_b: Address<'a>,
    pub token_badge_a: Address<'a>,
    pub token_badge_b: Address<'a>,
    pub funder: Address<'a>,
    pub whirlpool: Address<'a>,
    pub token_vault_a: Address<'a>,
    pub token_vault_b: Address<'a>,
    pub fee_tier: Address<'a>,
    pub token_program_a: Address<'a>,
    pub token_program_b: Address<'a>,
    pub system_program: Address<'a>,
    pub rent: Address<'a>,
}

pub fn process_initialize_pool_v2(
    data: InitializePoolInstructionV2,
    input_accounts: InitializePoolInstructionAccountsV2,
) -> Option<Type> {
    Some(Type::InitializePoolV2(InitializePoolV2 {
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
