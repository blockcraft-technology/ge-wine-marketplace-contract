use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

pub const OWNER: Item<Addr> = Item::new("owner");
pub const LISTINGS: Map<String, Listing> = Map::new("listings");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Listing {
    pub token_id: String,
    pub seller: Addr,
    pub price: Uint128,
    pub quantity: Uint128,
}

impl Default for Listing {
    fn default() -> Self {
        Listing {
            token_id: "".to_string(),
            seller: Addr::unchecked(""),
            price: Uint128::zero(),
            quantity: Uint128::zero(),
        }
    }
}
