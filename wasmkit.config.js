const localnet_accounts = [
  {
    name: 'account_0',
    address: 'juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y',
    mnemonic: 'clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose'
  }
];

const umee_accounts = [
  {
    name: 'account_0',
    address: 'umee14zjhj7y6n9np0q8wfctasnfzgrzajansllh0du',
    mnemonic: 'only song turkey pelican settle claw loud glass raccoon school alert powder clap suit harvest pizza street split point swallow filter element carry memory'
  }
];

const nibiru_accounts = [
  {
    name: 'account_0',
    address: 'nibi1zcmtvr8gue0xj3h8vyf969c05lpzuupnfm0d8s',
    mnemonic: 'piece fluid casual gadget elephant smoke actor angle success feel window auction settle eager rifle object hen index dish cat swallow fluid game elder'
  }
];

const networks = {
  localnet: {
    endpoint: 'http://localhost:26657/',
    chainId: 'testing-1',
    accounts: localnet_accounts,
  },
  umee: {
    endpoint: 'http://35.246.147.222:36657',
    chainId: 'umee-hack',
    accounts: umee_accounts,
  },
  nibiru: {
    endpoint: 'http://35.246.147.222:26657',
    chainId: 'nibiru-hack',
    accounts: nibiru_accounts,
  },
};

module.exports = {
  networks: {
    umee: networks.umee,
    nibiru: networks.nibiru,
    default: networks.localnet,
    localnet: networks.localnet,
  },
  mocha: {
    timeout: 60000
  },
  rust: {
    version: "1.68.2",
  }
};
