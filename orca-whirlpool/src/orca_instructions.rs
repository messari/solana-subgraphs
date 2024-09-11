use crate::constants;
use crate::instructions::{
    decrease_liquidity::{DecreaseLiquidityInstruction, DecreaseLiquidityInstructionAccounts},
    decrease_liquidity_v2::{
        DecreaseLiquidityInstructionAccountsV2, DecreaseLiquidityInstructionV2,
    },
    increase_liquidity::{IncreaseLiquidityInstruction, IncreaseLiquidityInstructionAccounts},
    increase_liquidity_v2::{
        IncreaseLiquidityInstructionAccountsV2, IncreaseLiquidityInstructionV2,
    },
    initialize_pool::{InitializePoolInstruction, InitializePoolInstructionAccounts},
    initialize_pool_v2::{InitializePoolInstructionAccountsV2, InitializePoolInstructionV2},
    swap::{SwapInstruction, SwapInstructionAccounts},
    swap_v2::{SwapInstructionAccountsV2, SwapInstructionV2},
    two_hop_swap::{TwoHopSwapInstruction, TwoHopSwapInstructionAccounts},
    two_hop_swap_v2::{TwoHopSwapInstructionAccountsV2, TwoHopSwapInstructionV2},
};

use crate::traits::account_deserialize::AccountsDeserialize;

use borsh::BorshDeserialize;
use substreams_solana::block_view::InstructionView;

#[derive(Debug)]
pub enum OrcaInstructions<'a> {
    InitializePool(
        InitializePoolInstruction,
        InitializePoolInstructionAccounts<'a>,
    ),
    InitializePoolV2(
        InitializePoolInstructionV2,
        InitializePoolInstructionAccountsV2<'a>,
    ),
    IncreaseLiquidity(
        IncreaseLiquidityInstruction,
        IncreaseLiquidityInstructionAccounts<'a>,
    ),
    IncreaseLiquidityV2(
        IncreaseLiquidityInstructionV2,
        IncreaseLiquidityInstructionAccountsV2<'a>,
    ),
    DecreaseLiquidity(
        DecreaseLiquidityInstruction,
        DecreaseLiquidityInstructionAccounts<'a>,
    ),
    DecreaseLiquidityV2(
        DecreaseLiquidityInstructionV2,
        DecreaseLiquidityInstructionAccountsV2<'a>,
    ),
    TwoHopSwap(TwoHopSwapInstruction, TwoHopSwapInstructionAccounts<'a>),
    TwoHopSwapV2(TwoHopSwapInstructionV2, TwoHopSwapInstructionAccountsV2<'a>),
    Swap(SwapInstruction, SwapInstructionAccounts<'a>),
    SwapV2(SwapInstructionV2, SwapInstructionAccountsV2<'a>),
}

impl<'a> OrcaInstructions<'a> {
    fn deserialize_instruction<T: BorshDeserialize, U: AccountsDeserialize<'a>>(
        rest: &mut &[u8],
        instruction_view: &'a InstructionView,
    ) -> Option<(T, U)> {
        let data = T::deserialize(rest).ok()?;
        let input_accounts = U::deserialize(instruction_view)?;
        Some((data, input_accounts))
    }

    pub fn from(instruction_view: &'a InstructionView) -> Option<Self> {
        let (tag, mut rest) = instruction_view.data().split_at(8);

        match tag {
            x if x == constants::DiscriminatorConstants::INITIALIZE_POOL => {
                let (data, input_accounts) = Self::deserialize_instruction::<
                    InitializePoolInstruction,
                    InitializePoolInstructionAccounts,
                >(&mut rest, instruction_view)?;
                Some(OrcaInstructions::InitializePool(data, input_accounts))
            }
            x if x == constants::DiscriminatorConstants::INITIALIZE_POOL_V2 => {
                let (data, input_accounts) = Self::deserialize_instruction::<
                    InitializePoolInstructionV2,
                    InitializePoolInstructionAccountsV2,
                >(&mut rest, instruction_view)?;
                Some(OrcaInstructions::InitializePoolV2(data, input_accounts))
            }
            x if x == constants::DiscriminatorConstants::INCREASE_LIQUIDITY => {
                let (data, input_accounts) = Self::deserialize_instruction::<
                    IncreaseLiquidityInstruction,
                    IncreaseLiquidityInstructionAccounts,
                >(&mut rest, instruction_view)?;
                Some(OrcaInstructions::IncreaseLiquidity(data, input_accounts))
            }
            x if x == constants::DiscriminatorConstants::INCREASE_LIQUIDITY_V2 => {
                let (data, input_accounts) = Self::deserialize_instruction::<
                    IncreaseLiquidityInstructionV2,
                    IncreaseLiquidityInstructionAccountsV2,
                >(&mut rest, instruction_view)?;
                Some(OrcaInstructions::IncreaseLiquidityV2(data, input_accounts))
            }
            x if x == constants::DiscriminatorConstants::DECREASE_LIQUIDITY => {
                let (data, input_accounts) = Self::deserialize_instruction::<
                    DecreaseLiquidityInstruction,
                    DecreaseLiquidityInstructionAccounts,
                >(&mut rest, instruction_view)?;
                Some(OrcaInstructions::DecreaseLiquidity(data, input_accounts))
            }
            x if x == constants::DiscriminatorConstants::DECREASE_LIQUIDITY_V2 => {
                let (data, input_accounts) = Self::deserialize_instruction::<
                    DecreaseLiquidityInstructionV2,
                    DecreaseLiquidityInstructionAccountsV2,
                >(&mut rest, instruction_view)?;
                Some(OrcaInstructions::DecreaseLiquidityV2(data, input_accounts))
            }
            x if x == constants::DiscriminatorConstants::TWO_HOP_SWAP => {
                let (data, input_accounts) = Self::deserialize_instruction::<
                    TwoHopSwapInstruction,
                    TwoHopSwapInstructionAccounts,
                >(&mut rest, instruction_view)?;
                Some(OrcaInstructions::TwoHopSwap(data, input_accounts))
            }
            x if x == constants::DiscriminatorConstants::TWO_HOP_SWAP_V2 => {
                let (data, input_accounts) = Self::deserialize_instruction::<
                    TwoHopSwapInstructionV2,
                    TwoHopSwapInstructionAccountsV2,
                >(&mut rest, instruction_view)?;
                Some(OrcaInstructions::TwoHopSwapV2(data, input_accounts))
            }
            x if x == constants::DiscriminatorConstants::SWAP => {
                let (data, input_accounts) = Self::deserialize_instruction::<
                    SwapInstruction,
                    SwapInstructionAccounts,
                >(&mut rest, instruction_view)?;
                Some(OrcaInstructions::Swap(data, input_accounts))
            }
            x if x == constants::DiscriminatorConstants::SWAP_V2 => {
                let (data, input_accounts) = Self::deserialize_instruction::<
                    SwapInstructionV2,
                    SwapInstructionAccountsV2,
                >(&mut rest, instruction_view)?;
                Some(OrcaInstructions::SwapV2(data, input_accounts))
            }
            _ => None,
        }
    }
}
