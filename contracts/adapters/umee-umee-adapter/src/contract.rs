use cosmwasm_std::{
    entry_point, to_binary, Binary, Env, Deps, DepsMut,
    MessageInfo, Response, StdError, StdResult, Uint128,
};

// use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;

use crate::error::ContractError;
use crate::query::{query_config, query_state};
use crate::execute::{try_deposit, try_withdraw};
use crate::state::{Config, State, CONFIG, STATE};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
// use crate::red_bank::{RedBankQueryMsg, MarketResponse};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let config = Config {
        owner: info.sender.clone(),
        red_bank: msg.red_bank.clone(),
        market_address: msg.market_address,
        collateral_denom: msg.collateral_denom,
        borrowed_denom: msg.borrow_denom,
    };

    // let market_msg = RedBankQueryMsg::Market {
    //     denom: config.collateral_denom.clone(),
    // };
    // let market_query = WasmQuery::Smart {
    //     contract_addr: config.red_bank.to_string(),
    //     msg: to_binary(&market_msg)?,
    // };
    // let market_data: MarketResponse = deps.querier.query(&QueryRequest::Wasm(
    //     market_query
    // ))?;

    let state: State = State {
        collateral_deposited: Uint128::from(0u128),
        asset_borrowed: Uint128::from(0u128),
    };

    CONFIG.save(deps.storage, &config)?;
    STATE.save(deps.storage, &state)?;

    deps.api.debug(&format!("Contract was initialized by {}", info.sender));

    Ok(
        Response::default()
        // .add_messages(messages)
        .add_attribute("action", "init")
        .add_attribute("sender", info.sender.clone())
        .add_attribute("red_bank", config.red_bank.clone())
        .add_attribute("market_address", config.market_address.clone())
        .add_attribute("collateral_denom", config.collateral_denom.clone())
        .add_attribute("borrowed_denom", config.borrowed_denom.clone())
    )
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::DepositAndBorrow {
            borrow_amount,
        } => try_deposit(deps, env, info, borrow_amount),
        ExecuteMsg::WithdrawCollateral {
            collateral_amount,
        } => try_withdraw(deps, env, info, collateral_amount),

        // ExecuteMsg::UpdateYieldBearingDenom { yield_bearing_denom } =>
        //     try_update_yield_bearing_denom(deps, info, yield_bearing_denom),
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::State {} => to_binary(&query_state(deps)?),
        // QueryMsg::UserDeposit {} => to_binary(&query_user_deposit(deps)?),
        // QueryMsg::TotalDeposit {} => to_binary(&query_total_deposit(deps)?),
    }
}
