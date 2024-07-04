use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProstCoin {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestCallback {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub job_id: u64,
    #[prost(int64, tag = "4")]
    pub callback_height: i64,
    #[prost(message, required, tag = "5")]
    pub fees: ProstCoin,
}

impl From<cosmwasm_std::Coin> for ProstCoin {
    fn from(coin: cosmwasm_std::Coin) -> Self {
        ProstCoin {
            denom: coin.denom,
            amount: coin.amount.to_string(),
        }
    }
}

impl From<ProstCoin> for cosmwasm_std::Coin {
    fn from(prost_coin: ProstCoin) -> Self {
        cosmwasm_std::Coin {
            denom: prost_coin.denom,
            amount: prost_coin.amount.parse().unwrap_or_default(),
        }
    }
}

pub const STATE: Item<State> = Item::new("state");
