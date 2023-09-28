use cosmwasm_std::{
    coin, entry_point, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply, Response,
    StdError, StdResult, SubMsg, Uint128, Addr,
};
use neutron_sdk::{
    bindings::{
        msg::{IbcFee, MsgIbcTransferResponse, NeutronMsg},
        query::NeutronQuery,
    },
    query::min_ibc_fee::query_min_ibc_fee,
    sudo::msg::{RequestPacket, RequestPacketTimeoutHeight, TransferSudoMsg},
    NeutronResult,
};

pub fn execute_borrow(
    mut deps: DepsMut<NeutronQuery>,
    env: Env,
    market_id: u128,
    usdc_amount: Uint128,
    colletral_denom: String,
    colletral_amount: Uint128,
) -> NeutronResult<Response<NeutronMsg>> {
    Ok(Response::default())
}

pub fn execute_liquidate(
    mut deps: DepsMut<NeutronQuery>,
    env: Env,
    market_id: u128,
    usdc_amount: Uint128,
    colletral_denom: String,
    colletral_amount: Uint128,
) -> NeutronResult<Response<NeutronMsg>> {
    Ok(Response::default())
}

pub fn execute_add_market(
    mut deps: DepsMut<NeutronQuery>,
    env: Env,
    market_id: u128,
    market_name: String,
    market_contract: Addr,
) -> NeutronResult<Response<NeutronMsg>> {
    Ok(Response::default())
}

pub fn execute_remove_market(
    mut deps: DepsMut<NeutronQuery>,
    env: Env,
    market_id: u128,
) -> NeutronResult<Response<NeutronMsg>> {
    Ok(Response::default())
}
