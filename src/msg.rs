use cosmwasm_std::{Uint128, Addr};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Addr,
    
    pub payment_token: Addr,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ListToken { token_id: String, price: Uint128, quantity: Uint128 },
    
    RemoveListing { token_id: String },
    
    BuyToken { token_id: String, quantity: Uint128 },
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetListing { token_id: String },
    
    GetListingsByOwner { owner: String },
    
    GetAllListings {},
}
