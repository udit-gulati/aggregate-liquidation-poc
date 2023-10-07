import { Contract, wasmKitTypes, Coin } from "@arufa/wasmkit";
export type ExecuteMsg = {
  deposit_and_borrow: {
    borrow_amount: Uint128;
  };
} | {
  withdraw_collateral: {
    collateral_amount: Uint128;
  };
} | {
  liquidate_position: {
    position_id: Uint128;
  };
};
export type Uint128 = string;
export type Addr = string;
export interface InstantiateMsg {
  borrow_denom: string;
  collateral_denom: string;
  market_address: Addr;
  red_bank: Addr;
}
export type QueryMsg = {
  config: {};
} | {
  state: {};
};
export interface ConfigResponse {
  borrowed_denom: string;
  collateral_denom: string;
  market_address: Addr;
  owner: Addr;
  red_bank: Addr;
}
export interface StateResponse {
  asset_borrowed: Uint128;
  collateral_deposited: Uint128;
}
export interface MarsAdapterReadOnlyInterface {
  config: () => Promise<any>;
  state: () => Promise<any>;
}
export class MarsAdapterQueryContract extends Contract implements MarsAdapterReadOnlyInterface {
  constructor(contractName: string, instantiateTag?: string) {
    super(contractName, instantiateTag);
    this.config = this.config.bind(this);
    this.state = this.state.bind(this);
  }
  config = async (): Promise<any> => {
    return this.queryMsg({
      config: {}
    });
  };
  state = async (): Promise<any> => {
    return this.queryMsg({
      state: {}
    });
  };
}
export interface MarsAdapterInterface extends MarsAdapterReadOnlyInterface {
  depositAndBorrow: ({
    account,
    customFees,
    memo,
    transferAmount
  }: {
    account: wasmKitTypes.UserAccount;
    customFees?: wasmKitTypes.TxnStdFee;
    memo?: string;
    transferAmount?: readonly Coin[];
  }, {
    borrowAmount
  }: {
    borrowAmount: Uint128;
  }) => Promise<any>;
  withdrawCollateral: ({
    account,
    customFees,
    memo,
    transferAmount
  }: {
    account: wasmKitTypes.UserAccount;
    customFees?: wasmKitTypes.TxnStdFee;
    memo?: string;
    transferAmount?: readonly Coin[];
  }, {
    collateralAmount
  }: {
    collateralAmount: Uint128;
  }) => Promise<any>;
  liquidatePosition: ({
    account,
    customFees,
    memo,
    transferAmount
  }: {
    account: wasmKitTypes.UserAccount;
    customFees?: wasmKitTypes.TxnStdFee;
    memo?: string;
    transferAmount?: readonly Coin[];
  }, {
    positionId
  }: {
    positionId: Uint128;
  }) => Promise<any>;
}
export class MarsAdapterContract extends MarsAdapterQueryContract implements MarsAdapterInterface {
  constructor(instantiateTag?: string) {
    super("mars_adapter", instantiateTag);
    this.depositAndBorrow = this.depositAndBorrow.bind(this);
    this.withdrawCollateral = this.withdrawCollateral.bind(this);
    this.liquidatePosition = this.liquidatePosition.bind(this);
  }
  depositAndBorrow = async ({
    account,
    customFees,
    memo,
    transferAmount
  }: {
    account: wasmKitTypes.UserAccount;
    customFees?: wasmKitTypes.TxnStdFee;
    memo?: string;
    transferAmount?: readonly Coin[];
  }, {
    borrowAmount
  }: {
    borrowAmount: Uint128;
  }): Promise<any> => {
    return await this.executeMsg({
      deposit_and_borrow: {
        borrow_amount: borrowAmount
      }
    }, account, customFees, memo, transferAmount);
  };
  withdrawCollateral = async ({
    account,
    customFees,
    memo,
    transferAmount
  }: {
    account: wasmKitTypes.UserAccount;
    customFees?: wasmKitTypes.TxnStdFee;
    memo?: string;
    transferAmount?: readonly Coin[];
  }, {
    collateralAmount
  }: {
    collateralAmount: Uint128;
  }): Promise<any> => {
    return await this.executeMsg({
      withdraw_collateral: {
        collateral_amount: collateralAmount
      }
    }, account, customFees, memo, transferAmount);
  };
  liquidatePosition = async ({
    account,
    customFees,
    memo,
    transferAmount
  }: {
    account: wasmKitTypes.UserAccount;
    customFees?: wasmKitTypes.TxnStdFee;
    memo?: string;
    transferAmount?: readonly Coin[];
  }, {
    positionId
  }: {
    positionId: Uint128;
  }): Promise<any> => {
    return await this.executeMsg({
      liquidate_position: {
        position_id: positionId
      }
    }, account, customFees, memo, transferAmount);
  };
}