use cosmwasm_std::{
    Binary, StdResult, Deps, to_binary,
};
use neutron_sdk::bindings::query::NeutronQuery;

use crate::msg::QueryResponse;
use crate::state::{State, STATE, Config, CONFIG};

pub fn query_info(deps: Deps<NeutronQuery>) -> StdResult<Binary> {
    let config: Config = CONFIG.load(deps.storage)?;

    to_binary(&QueryResponse::Info {
        admin: config.admin,
    })
}

pub fn query_state(deps: Deps<NeutronQuery>) -> StdResult<Binary> {
    let state: State = STATE.load(deps.storage)?;

    to_binary(&QueryResponse::State {
        positions_count: state.positions_count,
        total_usdc_borrowed: state.total_usdc_borrowed,
    })
}

pub fn query_config(deps: Deps<NeutronQuery>) -> StdResult<Binary> {
    let config: Config = CONFIG.load(deps.storage)?;

    to_binary(&QueryResponse::Config {
        admin: config.admin,
        contract_addr: config.contract_addr,
        gas_denom: config.gas_denom,
        usdc_denom: config.usdc_denom,
        epoch_period: config.epoch_period,
        dev_address: config.dev_address,
        dev_fee: config.dev_fee,
        paused: config.paused,
    })
}
