use cosmwasm_std::{
    DepsMut, MessageInfo, Response, Uint128, CosmosMsg,
    WasmMsg, to_binary, Coin, Env,
};

use crate::error::ContractError;
use crate::red_bank::RedBankExecuteMsg;
use crate::state::{CONFIG, Config, STATE, State};
// use crate::red_bank::{RedBankQueryMsg, MarketResponse, RedBankExecuteMsg};

pub fn try_deposit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    borrow_amount: Uint128,
) -> Result<Response, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];
    // let contract_addr: String = env.contract.address.to_string();

    let config: Config = CONFIG.load(deps.storage)?;
    let mut state: State = STATE.load(deps.storage)?;

    // read amount of collateral sent by user on deposit
    let mut amount_collateral: Uint128 = Uint128::default();
    for coin in &info.funds {
        if coin.denom == config.collateral_denom {
            amount_collateral = coin.amount
        }
    }

    // let market_msg = RedBankQueryMsg::Market {
    //     denom: config.underlying_denom.clone(),
    // };
    // let market_query = WasmQuery::Smart {
    //     contract_addr: config.red_bank.to_string(),
    //     msg: to_binary(&market_msg)?,
    // };
    // let market_data: MarketResponse = deps.querier.query(&QueryRequest::Wasm(
    //     market_query
    // ))?;

    state.collateral_deposited += amount_collateral;
    state.asset_borrowed += borrow_amount;
    STATE.save(deps.storage, &state)?;

    // Deposit user's collateral into red bank
    let collateral_deposit_msg: RedBankExecuteMsg = RedBankExecuteMsg::Deposit {
        on_behalf_of: None, // TODO: use this for multi users (applicable for other markets?)
    };
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.red_bank.to_string(),
        msg: to_binary(&collateral_deposit_msg)?,
        funds: vec![Coin {
            denom: config.collateral_denom.clone(),
            amount: amount_collateral,
        }],
    }));

    // Borrow amount of asset specified by the user
    let borrow_msg: RedBankExecuteMsg = RedBankExecuteMsg::Borrow {
        denom: config.borrowed_denom,
        amount: borrow_amount,
        recipient: None,
    };
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.red_bank.to_string(),
        msg: to_binary(&borrow_msg)?,
        funds: vec![],
    }));

    deps.api.debug("collateral deposited successfully");
    Ok(
        Response::new()
        .add_messages(messages)
        .add_attribute("collateral_deposited", amount_collateral.to_string())
        .add_attribute("total_collateral_deposited", state.collateral_deposited.to_string())
        // .add_attribute("exchange_rate", state.exchange_rate.to_string())
    )
}

pub fn try_withdraw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    collateral_amount: Uint128,
) -> Result<Response, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];
    // let contract_addr: String = env.contract.address.to_string();

    // Calc amount of collateral to withdraw
    let config: Config = CONFIG.load(deps.storage)?;
    let mut state: State = STATE.load(deps.storage)?;

    // read amount of repay asset sent by user on deposit
    let mut amount_repay: Uint128 = Uint128::default();
    for coin in &info.funds {
        if coin.denom == config.borrowed_denom {
            amount_repay = coin.amount
        }
    }

    // let market_msg = RedBankQueryMsg::Market {
    //     denom: config.underlying_denom.clone(),
    // };
    // let market_query = WasmQuery::Smart {
    //     contract_addr: config.red_bank.to_string(),
    //     msg: to_binary(&market_msg)?,
    // };
    // let market_data: MarketResponse = deps.querier.query(&QueryRequest::Wasm(
    //     market_query
    // ))?;

    state.collateral_deposited = state.collateral_deposited.checked_sub(collateral_amount).unwrap();
    state.asset_borrowed = state.asset_borrowed.checked_sub(amount_repay).unwrap();
    STATE.save(deps.storage, &state)?;

    // Repay borrowed amount
    let repay_msg: RedBankExecuteMsg = RedBankExecuteMsg::Repay {
        on_behalf_of: None,
    };
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.red_bank.to_string(),
        msg: to_binary(&repay_msg)?,
        funds: vec![Coin {
            denom: config.borrowed_denom.clone(),
            amount: amount_repay,
        }],
    }));

    // Withdraw the collateral
    let collateral_withdraw_msg: RedBankExecuteMsg = RedBankExecuteMsg::Withdraw {
        denom: config.collateral_denom.to_string(),
        amount: Some(collateral_amount),
        recipient: None,
    };
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.red_bank.to_string(),
        msg: to_binary(&collateral_withdraw_msg)?,
        funds: vec![],
    }));

    deps.api.debug("collateral withdrawn successfully");
    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("collateral_withdrawn", collateral_amount.to_string())
        .add_attribute("total_collateral_deposited", state.collateral_deposited.to_string())
        // .add_attribute("exchange_rate", state.exchange_rate.to_string())
    )
}
