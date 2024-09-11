use crate::pb::messari::orca_whirlpool::v1::event::Type;
use crate::pb::messari::orca_whirlpool::v1::{decrease_liquidity, DecreaseLiquidity};
use crate::traits::account_deserialize::AccountsDeserialize;
use crate::traits::balance_of::BalanceOf;
use crate::utils;

use borsh::{BorshDeserialize, BorshSerialize};
use derive_deserialize::AccountsDeserialize;
use substreams_solana::block_view::InstructionView;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;
use substreams_solana::Address;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct DecreaseLiquidityInstruction {
    // The total amount of Liquidity the user desires to withdraw.
    pub liquidity_amount: u128,
    // The minimum amount of tokenA the user is willing to withdraw.
    pub token_min_a: u64,
    // The minimum amount of tokenB the user is willing to withdraw.
    pub token_min_b: u64,
}

#[derive(AccountsDeserialize, Debug)]
pub struct DecreaseLiquidityInstructionAccounts<'a> {
    pub whirlpool: Address<'a>,
    pub token_program: Address<'a>,
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

pub fn process_decrease_liquidity(
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
