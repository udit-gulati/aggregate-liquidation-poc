use cosmwasm_std::{Uint128, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    pub gas_denom: String,
    pub usdc_denom: String,
    pub epoch_period: u64,

    /// platform fee related values
    pub dev_address: Addr,
    pub dev_fee: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum ExecuteMsg {
    Borrow {
        market_id: u128,
        usdc_amount: Uint128,
        colletral_denom: String,
        colletral_amount: Uint128,
    },
    Liquidate {
        market_id: u128,
        usdc_amount: Uint128,
        colletral_denom: String,
        colletral_amount: Uint128,
    },
    AddMarket {
        market_id: u128,
        market_name: String,
        market_contract: Addr,
    },
    RemoveMarket {
        market_contract: Addr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    Info {},
    State {},
    Config {},
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryResponse {
    Info {
        admin: Addr,
    },
    State {
        positions_count: u128,
        total_usdc_borrowed: Uint128,
    },
    Config {
        admin: Addr,
        contract_addr: Addr,
        gas_denom: String,
        usdc_denom: String,
        epoch_period: u64,
        dev_address: Addr,
        dev_fee: u64,
        paused: bool,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MigrateMsg {}
