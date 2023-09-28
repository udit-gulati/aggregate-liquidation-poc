use cosmwasm_std::{
    coin, entry_point, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply, Response,
    StdError, StdResult, SubMsg, Uint128,
};
use cw2::set_contract_version;
use neutron_sdk::{
    bindings::{
        msg::{IbcFee, MsgIbcTransferResponse, NeutronMsg},
        query::NeutronQuery,
    },
    sudo::msg::{RequestPacket, RequestPacketTimeoutHeight, TransferSudoMsg},
    NeutronResult,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{state::{Config, CONFIG, State, STATE}, execute::{execute_borrow, execute_liquidate, execute_add_market, execute_remove_market}};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg};

// Default timeout for IbcTransfer is 10000000 blocks
const DEFAULT_TIMEOUT_HEIGHT: u64 = 10000000;
const FEE_DENOM: &str = "untrn";

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config: Config = Config {
        admin: info.sender.clone(),
        contract_addr: env.contract.address.clone(),
        gas_denom: msg.gas_denom,
        usdc_denom: msg.usdc_denom,
        markets: Vec::new(),
        epoch_period: msg.epoch_period,
        dev_fee: msg.dev_fee.unwrap_or(10000), // default: 10% of rewards
        dev_address: msg.dev_address,
        paused: false,
        // kill_switch: KillSwitch::Closed.into(),
    };
    CONFIG.save(deps.storage, &config)?;

    // store state
    let state: State = State {
        positions_count: 0u128,
        total_usdc_borrowed: Uint128::from(0u128),
    };
    STATE.save(deps.storage, &state)?;

    Ok(
        Response::new()
        .add_attribute("action", "init")
        .add_attribute("sender", info.sender.clone())
    )
}

#[entry_point]
pub fn execute(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> NeutronResult<Response<NeutronMsg>> {
    match msg {
        ExecuteMsg::Borrow {
            market_id,
            usdc_amount,
            colletral_denom,
            colletral_amount,
        } => execute_borrow(deps, env, market_id, usdc_amount, colletral_denom, colletral_amount),
        ExecuteMsg::Liquidate {
            market_id,
            usdc_amount,
            colletral_denom,
            colletral_amount,
        } => execute_liquidate(deps, env, market_id, usdc_amount, colletral_denom, colletral_amount),
        ExecuteMsg::AddMarket {
            market_id,
            market_name,
            market_contract,
        } => execute_add_market(deps, env, market_id, market_name, market_contract),
        ExecuteMsg::RemoveMarket {
            market_id,
        } => execute_remove_market(deps, env, market_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps<NeutronQuery>,
    _env: Env,
    _msg: QueryMsg,
) -> StdResult<Binary> {
    match _msg {
        QueryMsg::Info {} => query_info(deps),
        // QueryMsg::SeatomExchangeRate {} => query_seatom_exchange_rate(deps),
        // QueryMsg::BatomExchangeRate {} => query_batom_exchange_rate(deps),
        // QueryMsg::QueryDevFee {} => query_dev_fee(deps),
        // QueryMsg::Window {} => query_current_window(deps),
        // QueryMsg::Undelegations { address } => query_pending_claims(deps, address),
        // QueryMsg::UserClaimable { address } => query_user_claimable(deps, address),
        // QueryMsg::ValidatorList {} => query_top_validators(deps),
        // QueryMsg::ActiveUnbonding { address } => query_active_undelegation(deps, address),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    deps.api.debug("WASMDEBUG: migrate");
    Ok(Response::default())
}
