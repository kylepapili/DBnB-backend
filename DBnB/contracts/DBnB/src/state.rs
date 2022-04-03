use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Binary, CanonicalAddr, Storage, Uint128};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";
pub static LISTINGS_KEY: &[u8] = b"listings";
pub static LISTING_IDS: &[u8] = b"listingsid";
pub static CONFIRMATIONS: &[u8] = b"confirms";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: CanonicalAddr,
    pub prng_seed: Binary
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Listing {
    pub id: String,
    pub owner: CanonicalAddr,
    pub name: String,
    pub description: String,
    pub address: String,
    pub images: Vec<String>,
    pub price: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Confirmation {
    pub id: String,
    pub addr: CanonicalAddr,
    pub start: u64,
    pub end: u64,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}
