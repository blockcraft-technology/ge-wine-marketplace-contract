use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use cosmwasm_std::entry_point;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, QueryMsg, InstantiateMsg};
use crate::state::{OWNER, LISTINGS, Listing};
use crate::helpers::{validate_owner, update_listing_balance, process_purchase};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    OWNER.save(deps.storage, &info.sender)?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ListToken { token_id, price, quantity } => list_token(deps, info, token_id, price, quantity),
        ExecuteMsg::RemoveListing { token_id } => remove_listing(deps, info, token_id),
        ExecuteMsg::BuyToken { token_id, quantity } => buy_token(deps, info, token_id, quantity),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetListing { token_id } => to_json_binary(&get_listing(deps, token_id)?),
        QueryMsg::GetListingsByOwner { owner } => to_json_binary(&get_listings_by_owner(deps, owner)?),
        QueryMsg::GetAllListings {} => to_json_binary(&get_all_listings(deps)?),
    }
}

pub fn list_token(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    price: Uint128,
    quantity: Uint128,
) -> Result<Response, ContractError> {
    let listing = update_listing_balance(&deps, info, token_id.clone(), quantity)?;  
    LISTINGS.save(deps.storage, token_id.clone(), &listing)?; 
    Ok(Response::new().add_attribute("action", "list_token").add_attribute("token_id", token_id))
}

pub fn remove_listing(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
) -> Result<Response, ContractError> {
    let listing = LISTINGS.load(deps.storage, token_id.clone())?;  
    validate_owner(&info.sender, &listing.seller)?;
    LISTINGS.remove(deps.storage, token_id.clone()); 
    Ok(Response::new().add_attribute("action", "remove_listing").add_attribute("token_id", token_id))
}

pub fn buy_token(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    quantity: Uint128,
) -> Result<Response, ContractError> {
    let listing = LISTINGS.load(deps.storage, token_id)?;
    process_purchase(deps, info, listing, quantity)
}

fn get_listing(deps: Deps, token_id: String) -> StdResult<Listing> {
    LISTINGS.load(deps.storage, token_id)
}

fn get_listings_by_owner(deps: Deps, owner: String) -> StdResult<Vec<Listing>> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let listings: Vec<Listing> = LISTINGS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .filter_map(|item| match item {
            Ok((_, listing)) if listing.seller == owner_addr => Some(listing),
            _ => None,
        })
        .collect();
    Ok(listings)
}

fn get_all_listings(deps: Deps) -> StdResult<Vec<Listing>> {
    let listings: Vec<Listing> = LISTINGS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| item.map(|(_, listing)| listing))
        .collect::<StdResult<_>>()?;
    Ok(listings)
}
