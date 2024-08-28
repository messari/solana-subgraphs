use crate::constants;
use crate::instructions::decrease_liquidity::{
    DecreaseLiquidityInstruction, DecreaseLiquidityInstructionAccounts,
};
use crate::instructions::increase_liquidity::{
    IncreaseLiquidityInstruction, IncreaseLiquidityInstructionAccounts,
};
use crate::instructions::initialize_pool::{
    InitializePoolInstruction, InitializePoolInstructionAccounts,
};
use crate::instructions::swap::{SwapInstruction, SwapInstructionAccounts};
use crate::instructions::two_hop_swap::{TwoHopSwapInstruction, TwoHopSwapInstructionAccounts};
use crate::traits::account_deserialize::AccountsDeserialize;

use borsh::BorshDeserialize;
use substreams_solana::block_view::InstructionView;

#[derive(Debug)]
pub enum OrcaInstructions<'a> {
    InitializePool {
        data: InitializePoolInstruction,
        input_accounts: InitializePoolInstructionAccounts<'a>,
    },
    IncreaseLiquidity {
        data: IncreaseLiquidityInstruction,
        input_accounts: IncreaseLiquidityInstructionAccounts<'a>,
    },
    DecreaseLiquidity {
        data: DecreaseLiquidityInstruction,
        input_accounts: DecreaseLiquidityInstructionAccounts<'a>,
    },
    TwoHopSwap {
        data: TwoHopSwapInstruction,
        input_accounts: TwoHopSwapInstructionAccounts<'a>,
    },
    Swap {
        data: SwapInstruction,
        input_accounts: SwapInstructionAccounts<'a>,
    },
}

impl<'a> OrcaInstructions<'a> {
    pub fn from(instruction_view: &'a InstructionView) -> Option<Self> {
        let (tag, mut rest) = instruction_view.data().split_at(8);

        match tag {
            x if x == &constants::DiscriminatorConstants::INITIALIZE_POOL => {
                let data = InitializePoolInstruction::deserialize(&mut rest).ok()?;
                let input_accounts =
                    InitializePoolInstructionAccounts::deserialize(instruction_view)?;

                return Some(OrcaInstructions::InitializePool {
                    data,
                    input_accounts,
                });
            }
            x if x == &constants::DiscriminatorConstants::INCREASE_LIQUIDITY => {
                let data = IncreaseLiquidityInstruction::deserialize(&mut rest).ok()?;
                let input_accounts =
                    IncreaseLiquidityInstructionAccounts::deserialize(instruction_view)?;

                return Some(OrcaInstructions::IncreaseLiquidity {
                    data,
                    input_accounts,
                });
            }
            x if x == &constants::DiscriminatorConstants::DECREASE_LIQUIDITY => {
                let data = DecreaseLiquidityInstruction::deserialize(&mut rest).ok()?;
                let input_accounts =
                    DecreaseLiquidityInstructionAccounts::deserialize(instruction_view)?;

                return Some(OrcaInstructions::DecreaseLiquidity {
                    data,
                    input_accounts,
                });
            }
            x if x == &constants::DiscriminatorConstants::TWO_HOP_SWAP => {
                let data = TwoHopSwapInstruction::deserialize(&mut rest).ok()?;
                let input_accounts = TwoHopSwapInstructionAccounts::deserialize(instruction_view)?;

                return Some(OrcaInstructions::TwoHopSwap {
                    data,
                    input_accounts,
                });
            }
            x if x == &constants::DiscriminatorConstants::SWAP => {
                let data = SwapInstruction::deserialize(&mut rest).ok()?;
                let input_accounts = SwapInstructionAccounts::deserialize(instruction_view)?;

                return Some(OrcaInstructions::Swap {
                    data,
                    input_accounts,
                });
            }
            _ => None,
        }
    }
}
