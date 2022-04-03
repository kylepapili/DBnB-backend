const accounts = [
  {
    name: 'account_0',
    address: 'secret1v47hf32ky24yhcw7tuwarw0rgfu7afc3j64487',
    mnemonic: 'wrist century army elbow cram draw absurd hello roof cage dog public middle perfect code release notice rich stomach bullet agree match home dice'
  },
  {
    name: 'account_1',
    address: 'secret1ftulcz0g5yrjplyxy58kawj2snl92nd6gfgrdz',
    mnemonic: 'tiger syrup sauce better save defense stool frost weapon secret mom panic skirt spoil light skull cancel list material latin story sadness evidence able'
  }
];

const networks = {
  localnet: {
    endpoint: "http://localhost:1317/",
    accounts: accounts,
    fees: {
      upload: {
        amount: [{ amount: "1500000", denom: "uscrt" }],
        gas: "6000000",
      },
      init: {
        amount: [{ amount: "12500", denom: "uscrt" }],
        gas: "50000",
      }
    }
  }
}

module.exports = {
  networks: {
    default: networks.localnet
  },
  mocha: {
    timeout: 60000
  }
};