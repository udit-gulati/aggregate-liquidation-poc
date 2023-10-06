use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;
// use mars_utils::{
//     error::ValidationError,
//     helpers::{decimal_param_le_one, decimal_param_lt_one},
// };

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub red_bank: Addr,
    pub market_address: Addr,
    pub collateral_denom: String,
    pub borrowed_denom: String,
    // pub yield_bearing_denom: String,
}

#[cw_serde]
pub struct State {
    pub collateral_deposited: Uint128,
    pub asset_borrowed: Uint128,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const STATE: Item<State> = Item::new("state");
// pub const OSMO_BALANCES: Map<&Addr, Uint128> = Map::new("osmo_balance");
