use cosmwasm_std::{Addr, Deps, DepsMut, MessageInfo, Response, StdResult, Uint128};
use crate::state::{LISTINGS, Listing};
use crate::error::ContractError;

pub fn validate_owner(sender: &Addr, owner: &Addr) -> Result<(), ContractError> {
    if sender != owner {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

pub fn update_listing_balance(
    deps: &DepsMut, 
    info: MessageInfo,
    token_id: String,
    quantity: Uint128,
) -> StdResult<Listing> {
    let current_listing = LISTINGS.may_load(deps.storage, token_id.clone())?.unwrap_or_default();
    let new_listing = Listing {
        token_id,
        seller: info.sender.clone(),
        price: current_listing.price,
        quantity: current_listing.quantity + quantity,
    };
    Ok(new_listing)
}

pub fn process_purchase(
    deps: DepsMut,
    info: MessageInfo,
    mut listing: Listing,
    quantity: Uint128,
) -> Result<Response, ContractError> {
    if listing.quantity < quantity {
        return Err(ContractError::InsufficientQuantity {});
    }

    listing.quantity -= quantity;
    LISTINGS.save(deps.storage, listing.token_id.clone(), &listing)?;

    Ok(Response::new()
        .add_attribute("action", "purchase")
        .add_attribute("buyer", info.sender)
        .add_attribute("token_id", listing.token_id)
        .add_attribute("quantity", quantity.to_string()))
}
