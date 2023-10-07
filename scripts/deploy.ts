import chalk from "chalk";
import { getAccountByName } from "@arufa/wasmkit";

import { MarsAdapterContract } from "../artifacts/typescript_schema/MarsAdapterContract";

// import relayerCfg from "./relayer_cfg.json";

function sleep(seconds: number) {
  console.log("Sleeping for " + seconds + " seconds");
  return new Promise(resolve => setTimeout(resolve, seconds*1000));
}

async function run () {
  const runTs = String(new Date());
  const contract_owner = await getAccountByName("account_0");
  const BLOCK_MAX_GAS = new Number(100_000_000).toString();

  const nativeDenom = "unibi";
  // const portId = relayerCfg.nibiru.port_id;
  // const channelId = relayerCfg.nibiru.channel_id;
  // const connectionId = relayerCfg.nibiru.connection_id;

  console.log("admin account fetched successfully");

  const liq_contract = new MarsAdapterContract();
  await liq_contract.setupClient();

  // deploy liquidation
  const deploy_liq_contract = await liq_contract.deploy(
    contract_owner,
    {
      amount: [{ amount: "16000000", denom: nativeDenom }],
      gas: "50000000",
    }
  );
  console.log(chalk.cyan("Response: "), deploy_liq_contract);

  // init liquidation
  const init_liq_contract = await liq_contract.instantiate(
    {
      borrow_denom: "uatom",
      collateral_denom: "uumee",
      market_address: "nibi1qg5ega6dykkxc307y25pecuufrjkxkaggkkxh7nad0vhyhtuhw3shmlsys",
      red_bank: "nibi1xr3rq8yvd7qplsw5yx90ftsr2zdhg4e9z60h5duusgxpv72hud3scz87vz",
    },
    `Liquidation contract ${runTs}`,
    contract_owner,
    undefined,
    { // custom fees
      amount: [{ amount: "16000000", denom: nativeDenom }],
      gas: "50000000",
    },
    contract_owner.account.address,
  );
  console.log(chalk.cyan("Response: "), init_liq_contract);

  console.log("All contract instance created successfully");

  // // Register account on remote chain
  // const register_res = await staking_contract.register(
  //   {
  //     account: contract_owner,
  //     customFees: {
  //       amount: [{ amount: "75000", denom: nativeDenom }],
  //       gas: "300000",
  //     },
  //   },
  //   {
  //     connectionId: connectionId,
  //     interchainAccountId: interchainAccountName,
  //   }
  // );
  // console.log(chalk.cyan("Response: "), register_res);

  // await sleep(10);  // wait for addr to be created
}

module.exports = { default: run };
