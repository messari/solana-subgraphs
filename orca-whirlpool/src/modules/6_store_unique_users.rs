use substreams::scalar::BigInt;
use substreams::store::StoreNew;
use substreams::store::StoreSetIfNotExists;
use substreams::store::StoreSetIfNotExistsBigInt;

use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::{event, Events};

#[substreams::handlers::store]
pub fn store_unique_users(raw_events: Events, store: StoreSetIfNotExistsBigInt) {
    for event in raw_events.data {
        match event.r#type.clone().unwrap() {
            event::Type::IncreaseLiquidity(increase_liquidity_event) => {
                let accounts = increase_liquidity_event.accounts.unwrap();

                store.set_if_not_exists(
                    0,
                    StoreKey::User.get_unique_key(&accounts.position_authority),
                    &BigInt::one(),
                );
            }

            event::Type::DecreaseLiquidity(decrease_liquidity_event) => {
                let accounts = decrease_liquidity_event.accounts.unwrap();

                store.set_if_not_exists(
                    0,
                    StoreKey::User.get_unique_key(&accounts.position_authority),
                    &BigInt::one(),
                );
            }

            event::Type::TwoHopSwap(two_hop_swap_event) => {
                let accounts = two_hop_swap_event.accounts.unwrap();

                store.set_if_not_exists(
                    0,
                    StoreKey::User.get_unique_key(&accounts.token_authority),
                    &BigInt::one(),
                );
            }

            event::Type::Swap(orca_swap_event) => {
                let accounts = orca_swap_event.accounts.unwrap();

                store.set_if_not_exists(
                    0,
                    StoreKey::User.get_unique_key(&accounts.token_authority),
                    &BigInt::one(),
                );
            }

            _ => {}
        }
    }
}
