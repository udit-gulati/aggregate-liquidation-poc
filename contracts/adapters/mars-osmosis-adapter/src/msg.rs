use cosmwasm_std::{Uint128, Addr};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub red_bank: Addr,
    pub market_address: Addr,
    pub collateral_denom: String,
    pub borrow_denom: String,
    // pub yield_bearing_denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    // collateral_amount is amount of collateral to deposit (sent with the msg)
    // borrow_amount is amount of asset to borrow against collateral
    DepositAndBorrow {
        borrow_amount: Uint128,
    },
    // repay_amount is amount borrowed to be repaid (sent with the msg)
    // collateral_amount is amount of collateral to withdraw after repay
    WithdrawCollateral {
        collateral_amount: Uint128,
    },

    // UpdateYieldBearingDenom {
    //     yield_bearing_denom: String,
    // },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // // UserDeposit returns the amount deposited by user as a json-encoded number
    // #[returns(UserDepositResponse)]
    // UserDeposit {},
    // // TotalDeposit returns the total amount deposited as a json-encoded number
    // #[returns(TotalDepositResponse)]
    // TotalDeposit {},
    // Config returns the contract config as a json-encoded number
    #[returns(ConfigResponse)]
    Config {},
    // State returns the contract state as a json-encoded number
    #[returns(StateResponse)]
    State {},
}

// // We define a custom struct for each query response
// #[cw_serde]
// pub struct UserDepositResponse {
//     pub underlying_amount: Uint128,
// }

// #[cw_serde]
// pub struct TotalDepositResponse {
//     pub underlying_amount: Uint128,
// }

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub red_bank: Addr,
    pub market_address: Addr,
    pub collateral_denom: String,
    pub borrowed_denom: String,
    // pub yield_bearing_denom: String,
}

#[cw_serde]
pub struct StateResponse {
    pub collateral_deposited: Uint128,
    pub asset_borrowed: Uint128,
}
