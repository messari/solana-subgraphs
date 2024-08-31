use substreams::store::StoreNew;
use substreams::store::StoreSetIfNotExists;
use substreams::store::StoreSetIfNotExistsProto;

use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::{event, Events, Withdraw};

#[substreams::handlers::store]
pub fn store_withdraws(raw_events: Events, store: StoreSetIfNotExistsProto<Withdraw>) {
    for event in raw_events.data {
        if let event::Type::DecreaseLiquidity(decrease_liquidity_event) = event.r#type.unwrap() {
            let instruction = decrease_liquidity_event.instruction.unwrap();
            let accounts = decrease_liquidity_event.accounts.unwrap();

            let pool_withdraw = Withdraw {
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
                StoreKey::Withdraw.get_unique_key(&pool_withdraw.id),
                &pool_withdraw,
            );
        }
    }
}
