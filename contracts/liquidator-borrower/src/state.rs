use cw_storage_plus::{Item, Map};
use cosmwasm_std::{Uint128, Addr};
use serde::{Deserialize, Serialize};

pub const STATE: Item<State> = Item::new("state");
pub const CONFIG: Item<Config> = Item::new("config");
pub const USER_POSITIONS: Map<Addr, Position> = Map::new("user_positions");

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct Position {
    pub market_id: u128,
    pub colletral_denom: String,
    pub colletral_amount: Uint128,
    pub borrowed_usdc_amount: Uint128,
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct State {
    pub positions_count: u128,
    pub total_usdc_borrowed: Uint128,
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct Chain {
    pub chain_id: String,
    pub connection_id: String,
    pub src_channel_id: String,
    pub dest_channel_id: String,
    pub dest_gas_denom: String,
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct Colletral {
    pub colletral_id: String,
    pub colletral_denom: String,
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct Market {
    pub market_id: u128,
    pub market_chain: Chain,
    pub market_colletrals: Vec<Colletral>,
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub admin: Addr,
    pub contract_addr: Addr,
    pub gas_denom: String,

    /// lending/borrowing related values
    pub usdc_denom: String,
    pub markets: Vec<Market>,
    pub epoch_period: u64,  // period of position rebalance

    /// platform fee related values
    pub dev_address: Addr,
    pub dev_fee: u64,       // 10^-3 percent. 1 = 0.001%

    /// pause the contract in case of issue
    pub paused: bool,
    // pub kill_switch: u8,
}
