use substreams::store::StoreNew;
use substreams::store::StoreSetIfNotExists;
use substreams::store::StoreSetIfNotExistsProto;

use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::{event, Deposit, Events};

#[substreams::handlers::store]
pub fn store_depsoits(raw_events: Events, store: StoreSetIfNotExistsProto<Deposit>) {
    for event in raw_events.data {
        match event.r#type.unwrap() {
            event::Type::IncreaseLiquidity(increase_liquidity_event) => {
                let instruction = increase_liquidity_event.instruction.unwrap();
                let accounts = increase_liquidity_event.accounts.unwrap();

                let pool_deposit = Deposit {
                    id: [event.txn_id.clone(), event.slot.to_string()].join("-"),

                    amount_a: instruction.amount_a.unwrap_or_default(),
                    amount_b: instruction.amount_b.unwrap_or_default(),

                    output_amount: instruction.liquidity_amount,

                    from: accounts.position_authority,
                    to: accounts.whirlpool,

                    slot: event.slot,
                    txn_id: event.txn_id,
                    block_height: event.block_height,
                    block_timestamp: event.block_timestamp,
                    block_hash: event.block_hash,
                };

                store.set_if_not_exists(
                    0,
                    StoreKey::Deposit.get_unique_key(&pool_deposit.id),
                    &pool_deposit,
                );
            }
            _ => {}
        }
    }
}
