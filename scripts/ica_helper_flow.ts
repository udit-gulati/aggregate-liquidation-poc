import chalk from "chalk";
import { getAccountByName } from "@arufa/wasmkit";

import { IcaHelperContract } from "../artifacts/typescript_schema/IcaHelperContract";

import networkConfig from "./config/localnet.json";

function sleep(seconds: number) {
  console.log("Sleeping for " + seconds + " seconds");
  return new Promise(resolve => setTimeout(resolve, seconds*1000));
}

async function run () {
  const runTs = String(new Date());
  const nativeDenom = "untrn";  // neutron fee token
  const remoteDenom = "uatom";  // cosmos hub fee token
  const contract_owner = await getAccountByName("account_0");

  const connectionId = networkConfig.relayers.gaia.connection_id;
  const interchainAccountName = "remote_account_1";
  const remoteValidatorOne = "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn";

  console.log("admin account fetched successfully");

  const dao_core = new CwdCoreContract();
  await dao_core.setupClient();

  const ica_helper = new IcaHelperContract();
  await ica_helper.setupClient();

  // Deploy Cwd Core
  const deploy_dao_core = await dao_core.deploy(
    contract_owner,
    {
      amount: [{ amount: "13000", denom: nativeDenom }],
      gas: "5000000",
    }
  );
  console.log(chalk.cyan("Response: "), deploy_dao_core);

  // Deploy ICA helper
  const deploy_ica_helper = await ica_helper.deploy(
    contract_owner,
    {
      amount: [{ amount: "13000", denom: nativeDenom }],
      gas: "5000000",
    }
  );
  console.log(chalk.cyan("Response: "), deploy_ica_helper);

  // Init Cwd Core with CodeId of Proposal, ICQ helper, ICA helper
  const core_contract_info = await dao_core.instantiate(
    {
      name: "AccTest",
      dao_uri: null,
      description: "testing DAO",
      chain_list: [
        {
          chain_id: "test-2",
          stake: 100,
        }
      ],
      contract_registry: chain_registry.contractAddress,
      initial_items: null,
      icq_helper_module_instantiate_info: {
        admin: null,
        code_id: icq_helper.codeId,
        label: `ICQ Helper Contract ${runTs}`,
        msg: {},
      },
      ica_helper_module_instantiate_info: {
        admin: null,
        code_id: icq_helper.codeId,
        label: `ICA Helper Contract ${runTs}`,
        msg: {},
      },
      proposal_modules_instantiate_info: [
        {
          admin: null,
          code_id: icq_helper.codeId,
          label: `Proposal Contract ${runTs}`,
          msg: {},
        }
      ],
      voting_registry_module_instantiate_info: {
        admin: null,
        code_id: icq_helper.codeId,
        label: `Voting Contract ${runTs}`,
        msg: {},
      },
    },
    `DAO Core contract ${runTs}`,
    contract_owner
  );
  console.log(chalk.cyan("Response: "), core_contract_info);

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