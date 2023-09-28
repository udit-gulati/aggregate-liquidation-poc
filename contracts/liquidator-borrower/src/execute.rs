use cosmwasm_std::{
    StdError, StdResult, SubMsg, Uint128, Addr, Reply, Response,
    coin, entry_point, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
};
use neutron_sdk::{
    bindings::{
        query::NeutronQuery,
        msg::{IbcFee, MsgIbcTransferResponse, NeutronMsg},
    },
    NeutronResult,
};

use crate::state::{MARKETS, Market};

pub fn execute_borrow(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    market_id: u128,
    usdc_amount: Uint128,
    colletral_denom: String,
    colletral_amount: Uint128,
) -> NeutronResult<Response<NeutronMsg>> {
    // use the adapter pattern and just call a common
    // interface here for given market_id
    // to deposit stATOM and borrow axlUSDC
    Ok(Response::default())
}

pub fn execute_liquidate(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    market_id: u128,
    usdc_amount: Uint128,
    colletral_denom: String,
    colletral_amount: Uint128,
) -> NeutronResult<Response<NeutronMsg>> {
    // use the adapter pattern and just call a common
    // interface here for given market_id
    // to repay axlUSDC and withdraw stATOM
    Ok(Response::default())
}

pub fn execute_add_market(
    deps: DepsMut<NeutronQuery>,
    _env: Env,
    market_id: u128,
    market_name: String,
    market_contract: Addr,
) -> NeutronResult<Response<NeutronMsg>> {
    // map the market_id to the market adapter
    // contract here, make sure the market contract
    // has the required interface as this contract
    // has no way to checking before adding to it's list

    if let Some(_) = MARKETS.may_load(deps.storage, market_contract.clone())? {
        return Err(StdError::generic_err(
           format!("Market with address {} already exists", market_contract.clone())
        ).into())
    }
    MARKETS.save(deps.storage, market_contract, &Market {
        market_id,
        market_name,
        market_colletrals: Vec::new(),
    })?;
    Ok(Response::default())
}

pub fn execute_remove_market(
    deps: DepsMut<NeutronQuery>,
    _env: Env,
    market_contract: Addr,
) -> NeutronResult<Response<NeutronMsg>> {
    // simple clear the market contract
    // details from this contract's storage

    if let Some(_) = MARKETS.may_load(deps.storage, market_contract.clone())? {
        MARKETS.remove(deps.storage, market_contract);
    }
    Ok(Response::default())
}
