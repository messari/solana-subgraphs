use substreams::store::StoreNew;
use substreams::store::StoreSetIfNotExists;
use substreams::store::StoreSetIfNotExistsProto;

use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::{event, Events, Pool};

#[substreams::handlers::store]
pub fn store_pools(raw_events: Events, store: StoreSetIfNotExistsProto<Pool>) {
    for event in raw_events.data {
        if let event::Type::InitalizePool(initialize_pool_event) = event.r#type.unwrap() {
            let accounts = initialize_pool_event.accounts.unwrap();

            let pool = Pool {
                address: accounts.whirlpool,

                token_mint_a: accounts.token_mint_a,
                token_mint_b: accounts.token_mint_b,

                token_vault_a: accounts.token_vault_a,
                token_vault_b: accounts.token_vault_b,

                created_timestamp: event.block_timestamp,
                created_block_number: event.block_height,
            };

            store.set_if_not_exists(0, StoreKey::Pool.get_unique_key(&pool.address), &pool);
        }
    }
}
