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
use substreams_solana::pb::sf::solana::r#type::v1::{CompiledInstruction, ConfirmedTransaction};

#[derive(Debug)]
pub enum OrcaInstructions<'a> {
    InitializePool {
        instruction: InitializePoolInstruction,
        accounts: InitializePoolInstructionAccounts<'a>,
    },
    IncreaseLiquidity {
        instruction: IncreaseLiquidityInstruction,
        accounts: IncreaseLiquidityInstructionAccounts<'a>,
    },
    DecreaseLiquidity {
        instruction: DecreaseLiquidityInstruction,
        accounts: DecreaseLiquidityInstructionAccounts<'a>,
    },
    TwoHopSwap {
        instruction: TwoHopSwapInstruction,
        accounts: TwoHopSwapInstructionAccounts<'a>,
    },
    Swap {
        instruction: SwapInstruction,
        accounts: SwapInstructionAccounts<'a>,
    },
}

impl<'a> OrcaInstructions<'a> {
    pub fn from(
        compiled_instr: &'a CompiledInstruction,
        confirmed_txn: &'a ConfirmedTransaction,
    ) -> Option<Self> {
        let (tag, mut rest) = compiled_instr.data.split_at(8);

        match tag {
            x if x == &constants::DiscriminatorConstants::INITIALIZE_POOL => {
                let instruction = InitializePoolInstruction::deserialize(&mut rest).ok()?;
                let accounts =
                    InitializePoolInstructionAccounts::deserialize(confirmed_txn, compiled_instr)?;

                return Some(OrcaInstructions::InitializePool {
                    instruction,
                    accounts,
                });
            }
            x if x == &constants::DiscriminatorConstants::INCREASE_LIQUIDITY => {
                let instruction = IncreaseLiquidityInstruction::deserialize(&mut rest).ok()?;
                let accounts = IncreaseLiquidityInstructionAccounts::deserialize(
                    confirmed_txn,
                    compiled_instr,
                )?;

                return Some(OrcaInstructions::IncreaseLiquidity {
                    instruction,
                    accounts,
                });
            }
            x if x == &constants::DiscriminatorConstants::DECREASE_LIQUIDITY => {
                let instruction = DecreaseLiquidityInstruction::deserialize(&mut rest).ok()?;
                let accounts = DecreaseLiquidityInstructionAccounts::deserialize(
                    confirmed_txn,
                    compiled_instr,
                )?;

                return Some(OrcaInstructions::DecreaseLiquidity {
                    instruction,
                    accounts,
                });
            }
            x if x == &constants::DiscriminatorConstants::TWO_HOP_SWAP => {
                let instruction = TwoHopSwapInstruction::deserialize(&mut rest).ok()?;
                let accounts =
                    TwoHopSwapInstructionAccounts::deserialize(confirmed_txn, compiled_instr)?;

                return Some(OrcaInstructions::TwoHopSwap {
                    instruction,
                    accounts,
                });
            }
            x if x == &constants::DiscriminatorConstants::SWAP => {
                let instruction = SwapInstruction::deserialize(&mut rest).ok()?;
                let accounts = SwapInstructionAccounts::deserialize(confirmed_txn, compiled_instr)?;

                return Some(OrcaInstructions::Swap {
                    instruction,
                    accounts,
                });
            }
            _ => None,
        }
    }
}
