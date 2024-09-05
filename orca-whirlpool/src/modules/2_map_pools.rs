use crate::pb::messari::orca_whirlpool::v1::{event, Events, Pool, Pools};
use substreams::skip_empty_output;

#[substreams::handlers::map]
pub fn map_pools(raw_events: Events) -> Result<Pools, substreams::errors::Error> {
    skip_empty_output();

    let data: Vec<Pool> = raw_events
        .data
        .into_iter()
        .filter_map(|event| match event.r#type {
            Some(event::Type::InitializePool(initialize_pool_event)) => {
                let accounts = initialize_pool_event.accounts.unwrap();
                Some(Pool {
                    address: accounts.whirlpool,
                    token_mint_a: accounts.token_mint_a,
                    token_mint_b: accounts.token_mint_b,
                    token_vault_a: accounts.token_vault_a,
                    token_vault_b: accounts.token_vault_b,
                    created_timestamp: event.block_timestamp,
                    created_block_number: event.block_height,
                })
            }
            Some(event::Type::InitializePoolV2(initialize_pool_v2_event)) => {
                let accounts = initialize_pool_v2_event.accounts.unwrap();
                Some(Pool {
                    address: accounts.whirlpool,
                    token_mint_a: accounts.token_mint_a,
                    token_mint_b: accounts.token_mint_b,
                    token_vault_a: accounts.token_vault_a,
                    token_vault_b: accounts.token_vault_b,
                    created_timestamp: event.block_timestamp,
                    created_block_number: event.block_height,
                })
            }
            _ => None,
        })
        .collect();

    Ok(Pools { data })
}
