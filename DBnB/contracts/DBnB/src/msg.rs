use cosmwasm_std::{Binary, HumanAddr, Uint128};
use schemars::JsonSchema;
use secret_toolkit::viewing_key::ViewingKey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub prng_seed: Binary
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    AddListing {
        name: String,
        description: String,
        address: String,
        images: Vec<String>,
        price: Uint128,
    },
    ConfirmListing {
        id: u32,
        start: u64,
        end: u64
    },
    CreateViewingKey {
        entropy: String,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    ListingConfirmation {
        booked: bool
    },
    CreateViewingKey {
        key: String
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetListings {
        page: u32,
        page_size: u32,
    },
    GetConfirmations {
        page: u32,
        page_size: u32,
        address: HumanAddr,
        vk: String
    },
    GetIndexOfListing {
        id: String
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    Listings {

    }
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AddListing {
}
