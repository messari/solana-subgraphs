use crate::constants;
use crate::instructions::{
    decrease_liquidity::process_decrease_liquidity,
    decrease_liquidity_v2::process_decrease_liquidity_v2,
    increase_liquidity::process_increase_liquidity,
    increase_liquidity_v2::process_increase_liquidity_v2, initialize_pool::process_initialize_pool,
    initialize_pool_v2::process_initialize_pool_v2, swap::process_swap, swap_v2::process_swap_v2,
    two_hop_swap::process_two_hop_swap, two_hop_swap_v2::process_two_hop_swap_v2,
};
use crate::orca_instructions::OrcaInstructions;
use crate::pb::messari::orca_whirlpool::v1::{Event, Events};

use substreams::skip_empty_output;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Events, substreams::errors::Error> {
    skip_empty_output();

    let data: Vec<Event> = block
        .transactions()
        .flat_map(|confirmed_txn| process_txn(confirmed_txn, &block))
        .collect();

    Ok(Events { data })
}

fn process_txn(confirmed_txn: &ConfirmedTransaction, block: &Block) -> Vec<Event> {
    confirmed_txn
        .walk_instructions()
        .filter(|instr| instr.program_id() == constants::ORCA_WHIRLPOOL)
        .filter_map(|instr| {
            OrcaInstructions::from(&instr)
                .and_then(|decoded_instr| process_instruction(decoded_instr, confirmed_txn, block))
        })
        .collect()
}

fn process_instruction(
    decoded_instr: OrcaInstructions,
    confirmed_txn: &ConfirmedTransaction,
    block: &Block,
) -> Option<Event> {
    let instr_type = match decoded_instr {
        OrcaInstructions::InitializePool(data, input_accounts) => {
            process_initialize_pool(data, input_accounts)
        }
        OrcaInstructions::InitializePoolV2(data, input_accounts) => {
            process_initialize_pool_v2(data, input_accounts)
        }
        OrcaInstructions::IncreaseLiquidity(data, input_accounts) => {
            process_increase_liquidity(data, input_accounts, confirmed_txn)
        }
        OrcaInstructions::IncreaseLiquidityV2(data, input_accounts) => {
            process_increase_liquidity_v2(data, input_accounts, confirmed_txn)
        }
        OrcaInstructions::DecreaseLiquidity(data, input_accounts) => {
            process_decrease_liquidity(data, input_accounts, confirmed_txn)
        }
        OrcaInstructions::DecreaseLiquidityV2(data, input_accounts) => {
            process_decrease_liquidity_v2(data, input_accounts, confirmed_txn)
        }
        OrcaInstructions::TwoHopSwap(data, input_accounts) => {
            process_two_hop_swap(data, input_accounts, confirmed_txn)
        }
        OrcaInstructions::TwoHopSwapV2(data, input_accounts) => {
            process_two_hop_swap_v2(data, input_accounts, confirmed_txn)
        }
        OrcaInstructions::Swap(data, input_accounts) => {
            process_swap(data, input_accounts, confirmed_txn)
        }
        OrcaInstructions::SwapV2(data, input_accounts) => {
            process_swap_v2(data, input_accounts, confirmed_txn)
        }
    };

    instr_type.map(|r#type| Event {
        slot: block.slot,
        txn_id: confirmed_txn.id().clone(),
        block_height: block.block_height.clone().unwrap_or_default().block_height,
        block_timestamp: block.block_time.clone().unwrap_or_default().timestamp,
        block_hash: block.blockhash.clone(),
        r#type: Some(r#type),
    })
}
