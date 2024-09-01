use crate::constants;
use crate::instructions::{
    decrease_liquidity::{DecreaseLiquidityInstruction, DecreaseLiquidityInstructionAccounts},
    increase_liquidity::{IncreaseLiquidityInstruction, IncreaseLiquidityInstructionAccounts},
    initialize_pool::{InitializePoolInstruction, InitializePoolInstructionAccounts},
    swap::{SwapInstruction, SwapInstructionAccounts},
    two_hop_swap::{TwoHopSwapInstruction, TwoHopSwapInstructionAccounts},
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
    IncreaseLiquidity(
        IncreaseLiquidityInstruction,
        IncreaseLiquidityInstructionAccounts<'a>,
    ),
    DecreaseLiquidity(
        DecreaseLiquidityInstruction,
        DecreaseLiquidityInstructionAccounts<'a>,
    ),
    TwoHopSwap(TwoHopSwapInstruction, TwoHopSwapInstructionAccounts<'a>),
    Swap(SwapInstruction, SwapInstructionAccounts<'a>),
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
            x if x == constants::DiscriminatorConstants::INCREASE_LIQUIDITY => {
                let (data, input_accounts) = Self::deserialize_instruction::<
                    IncreaseLiquidityInstruction,
                    IncreaseLiquidityInstructionAccounts,
                >(&mut rest, instruction_view)?;
                Some(OrcaInstructions::IncreaseLiquidity(data, input_accounts))
            }
            x if x == constants::DiscriminatorConstants::DECREASE_LIQUIDITY => {
                let (data, input_accounts) = Self::deserialize_instruction::<
                    DecreaseLiquidityInstruction,
                    DecreaseLiquidityInstructionAccounts,
                >(&mut rest, instruction_view)?;
                Some(OrcaInstructions::DecreaseLiquidity(data, input_accounts))
            }
            x if x == constants::DiscriminatorConstants::TWO_HOP_SWAP => {
                let (data, input_accounts) = Self::deserialize_instruction::<
                    TwoHopSwapInstruction,
                    TwoHopSwapInstructionAccounts,
                >(&mut rest, instruction_view)?;
                Some(OrcaInstructions::TwoHopSwap(data, input_accounts))
            }
            x if x == constants::DiscriminatorConstants::SWAP => {
                let (data, input_accounts) = Self::deserialize_instruction::<
                    SwapInstruction,
                    SwapInstructionAccounts,
                >(&mut rest, instruction_view)?;
                Some(OrcaInstructions::Swap(data, input_accounts))
            }
            _ => None,
        }
    }
}
