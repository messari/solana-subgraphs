#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod constants;
mod instructions;
mod pb;
mod utils;

use borsh::BorshDeserialize;
use hex;
use instructions::{
    DecreaseLiquidity, DecreaseLiquidityV2, IncreaseLiquidity, IncreaseLiquidityV2, InitializePool,
    InitializePoolV2, Swap, SwapV2, TwoHopSwap, TwoHopSwapV2,
};
use pb::messari::orca_whirlpool::v1::{OrcaSwaps, Swap as OrcaSwap};
use std::collections::HashSet;
use substreams::{log, skip_empty_output};
use substreams_entity_change::pb::entity::{entity_change, EntityChange, EntityChanges};
use substreams_solana::pb::sf::solana::r#type::v1::{
    Block, CompiledInstruction, InnerInstruction, InnerInstructions, Message, MessageHeader,
    TokenBalance, Transaction, TransactionStatusMeta,
};
use utils::{idl_discriminator, string_to_bigint, txn_pre_checks};

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<OrcaSwaps, substreams::errors::Error> {
    skip_empty_output();

    let mut data: Vec<OrcaSwap> = Vec::new();

    for confirmed_txn in block.transactions() {
        let (txn, txn_meta, txn_messages) = match txn_pre_checks(&confirmed_txn) {
            Some(details) => details,
            None => continue,
        };

        for (idx, inst) in txn_messages.instructions.iter().enumerate() {
            let accounts = confirmed_txn.resolved_accounts_as_strings();
            let program = &accounts[inst.program_id_index as usize];

            if program != constants::ORCA_WHIRLPOOL {
                continue;
            }

            let inner_instructions = txn_meta
                .inner_instructions
                .iter()
                .filter(|inner_inst| inner_inst.index == idx as u32)
                .cloned()
                .collect::<Vec<InnerInstructions>>();

            let signatures: Vec<String> = txn
                .signatures
                .clone()
                .into_iter()
                .map(|bytes| bs58::encode(bytes).into_string())
                .collect();

            let inst_data = inst.data.clone();
            let (tag, rest) = inst_data.split_at(8);

            match tag {
                x if x == &constants::DiscriminatorConstants::INITIALIZE_POOL => {
                    let decoded = InitializePool::try_from_slice(&rest);
                    // TODO
                }
                x if x == &constants::DiscriminatorConstants::INCREASE_LIQUIDITY => {
                    let decoded = IncreaseLiquidity::try_from_slice(&rest);
                    // TODO
                }
                x if x == &constants::DiscriminatorConstants::DECREASE_LIQUIDITY => {
                    let decoded = DecreaseLiquidity::try_from_slice(&rest);
                    // TODO
                }
                x if x == &constants::DiscriminatorConstants::TWO_HOP_SWAP => {
                    let decoded = TwoHopSwap::try_from_slice(&rest);
                    // TODO
                }
                x if x == &constants::DiscriminatorConstants::SWAP => {
                    let decoded = Swap::try_from_slice(&rest).unwrap();

                    data.push(OrcaSwap {
                        amount: decoded.amount,
                        other_amount_threshold: decoded.other_amount_threshold,
                        sqrt_price_limit: decoded.sqrt_price_limit.to_string(),
                        amount_specified_is_input: decoded.amount_specified_is_input,
                        a_to_b: decoded.a_to_b,
                        signature: signatures.get(0).unwrap().to_string(),
                    });

                    log::println("--------------------------------");
                    log::info!("txn: {:?}", signatures);
                    log::info!("1. Tag: {:?}", tag);
                    log::info!("2. Decoded: {:?}", decoded);
                    log::println("--------------------------------");
                }
                x if x == &constants::DiscriminatorConstants::INITIALIZE_POOL_V2 => {
                    let decoded = InitializePoolV2::try_from_slice(&rest);
                    // TODO
                }
                x if x == &constants::DiscriminatorConstants::INCREASE_LIQUIDITY_V2 => {
                    let decoded = IncreaseLiquidityV2::try_from_slice(&rest);
                    // TODO
                }
                x if x == &constants::DiscriminatorConstants::DECREASE_LIQUIDITY_V2 => {
                    let decoded = DecreaseLiquidityV2::try_from_slice(&rest);
                    // TODO
                }
                x if x == &constants::DiscriminatorConstants::TWO_HOP_SWAP_V2 => {
                    let decoded = TwoHopSwapV2::try_from_slice(&rest);
                    // TODO
                }
                x if x == &constants::DiscriminatorConstants::SWAP_V2 => {
                    let decoded = SwapV2::try_from_slice(&rest);
                    // TODO
                }
                _ => {}
            }
        }
    }

    Ok(OrcaSwaps { data })
}

#[substreams::handlers::map]
fn graph_out(events: OrcaSwaps) -> Result<EntityChanges, ()> {
    skip_empty_output();
    let mut entity_changes: Vec<EntityChange> = vec![];

    for swap in events.data.iter() {
        let id = [swap.signature.clone(), swap.amount.to_string()].join("-");

        let mut events_entity_change =
            EntityChange::new("OrcaSwap", id.as_str(), 0, entity_change::Operation::Create);

        events_entity_change
            .change("amount", swap.amount)
            .change("other_amount_threshold", swap.other_amount_threshold)
            .change(
                "sqrt_price_limit",
                string_to_bigint(swap.sqrt_price_limit.clone()),
            )
            .change("amount_specified_is_input", swap.amount_specified_is_input)
            .change("a_to_b", swap.a_to_b)
            .change("signature", swap.signature.clone());

        entity_changes.push(events_entity_change);
    }

    Ok(EntityChanges { entity_changes })
}
