use substreams::scalar::BigInt;
use substreams::skip_empty_output;
use substreams::store::{StoreNew, StoreSetIfNotExists, StoreSetIfNotExistsBigInt};

use crate::key_store::StoreKey;
use crate::pb::messari::orca_whirlpool::v1::{event, Events};

#[substreams::handlers::store]
pub fn store_unique_users(raw_events: Events, store: StoreSetIfNotExistsBigInt) {
    skip_empty_output();

    let bigint1 = BigInt::one();

    for event in raw_events.data {
        match event.r#type.clone().unwrap() {
            event::Type::IncreaseLiquidity(increase_liquidity_event) => {
                let accounts = increase_liquidity_event.accounts.unwrap();

                store.set_if_not_exists(
                    0,
                    StoreKey::User.get_unique_key(&accounts.position_authority),
                    &bigint1,
                );
            }

            event::Type::DecreaseLiquidity(decrease_liquidity_event) => {
                let accounts = decrease_liquidity_event.accounts.unwrap();

                store.set_if_not_exists(
                    0,
                    StoreKey::User.get_unique_key(&accounts.position_authority),
                    &bigint1,
                );
            }

            event::Type::TwoHopSwap(two_hop_swap_event) => {
                let accounts = two_hop_swap_event.accounts.unwrap();

                store.set_if_not_exists(
                    0,
                    StoreKey::User.get_unique_key(&accounts.token_authority),
                    &bigint1,
                );
            }

            event::Type::Swap(orca_swap_event) => {
                let accounts = orca_swap_event.accounts.unwrap();

                store.set_if_not_exists(
                    0,
                    StoreKey::User.get_unique_key(&accounts.token_authority),
                    &bigint1,
                );
            }

            _ => {}
        }
    }
}
