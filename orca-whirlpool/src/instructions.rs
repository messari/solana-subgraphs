use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct WhirlpoolBumps {
    pub whirlpool_bump: u8,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct InitializePool {
    // The bump value when deriving the PDA of the Whirlpool address.
    pub bumps: WhirlpoolBumps,
    // The desired tick spacing for this pool.
    pub tick_spacing: u16,
    // The desired initial sqrt-price for this pool.
    pub initial_sqrt_price: u128,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct IncreaseLiquidity {
    pub liquidity_amount: u128,
    pub token_max_a: u64,
    pub token_max_b: u64,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct DecreaseLiquidity {
    pub liquidity_amount: u128,
    pub token_min_a: u64,
    pub token_min_b: u64,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct Swap {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct TwoHopSwap {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub amount_specified_is_input: bool,
    pub a_to_b_one: bool,
    pub a_to_b_two: bool,
    pub sqrt_price_limit_one: u128,
    pub sqrt_price_limit_two: u128,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub enum AccountsType {
    TransferHookA,
    TransferHookB,
    TransferHookReward,
    TransferHookInput,
    TransferHookIntermediate,
    TransferHookOutput,
    SupplementalTickArrays,
    SupplementalTickArraysOne,
    SupplementalTickArraysTwo,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct RemainingAccountsSlice {
    pub accounts_type: AccountsType,
    pub length: u8,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct RemainingAccountsInfo {
    pub slices: Vec<RemainingAccountsSlice>,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct DecreaseLiquidityV2 {
    pub liquidity_amount: u128,
    pub token_min_a: u64,
    pub token_min_b: u64,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct IncreaseLiquidityV2 {
    pub liquidity_amount: u128,
    pub token_max_a: u64,
    pub token_max_b: u64,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct InitializePoolV2 {
    pub tick_spacing: u16,
    pub initial_sqrt_price: u128,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct SwapV2 {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct TwoHopSwapV2 {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub amount_specified_is_input: bool,
    pub a_to_b_one: bool,
    pub a_to_b_two: bool,
    pub sqrt_price_limit_one: u128,
    pub sqrt_price_limit_two: u128,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}
