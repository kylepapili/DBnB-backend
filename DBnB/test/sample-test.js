const { expect, use } = require("chai");
const { Contract, getAccountByName, polarChai } = require("secret-polar");
const { randomBytes } = require("crypto");

use(polarChai);

describe("DBnB", () => {
  async function setup() {
    const contract_owner = getAccountByName("account_1");
    const other = getAccountByName("account_0");
    const contract = new Contract("DBnB");
    await contract.parseSchema();

    return { contract_owner, other, contract };
  }

  it("deploy and init", async () => {
    const { contract_owner, other, contract } = await setup();
    const deploy_response = await contract.deploy(contract_owner);
    const contract_info = await contract.instantiate({ prng_seed: "dGVzdA=="}, "deploy test", contract_owner);
  });

  it("Add listing", async () => {
    const { contract_owner, other, contract } = await setup();
    const deploy_response = await contract.deploy(contract_owner);

    const contract_info = await contract.instantiate({ prng_seed: "dGVzdA=="}, "deploy test", contract_owner);

    const ex_response = await contract.tx.add_listing({account: contract_owner}, {name: "HI Hostel", description: "The best hostel ever", address: "891 Amsterdam Ave, NYC", images: ["http://localhost:3000"], price: "10000000"});
    // await expect(contract.query.get_count()).to.respondWith({ 'count': 103 });

  });

  it("Get listings", async () => {
    const { contract_owner, other, contract} = await setup();
    const deploy_response = await contract.deploy(contract_owner);

    const contract_info = await contract.instantiate({ prng_seed: "dGVzdA=="}, "deploy test", contract_owner);
    const ex_response = await contract.tx.add_listing({account: contract_owner}, {name: "HI Hostel", description: "The best hostel ever", address: "891 Amsterdam Ave, NYC", images: ["http://localhost:3000"], price: "10000000"});
    const query_response = await contract.query.get_listings({page: 0, page_size: 50});
    console.log("test", query_response);
  })

  it("Find listing index", async () => {
    const { contract_owner, other, contract} = await setup();
    const deploy_response = await contract.deploy(contract_owner);

    const contract_info = await contract.instantiate({ prng_seed: "dGVzdA=="}, "deploy test", contract_owner);
    const ex_response = await contract.tx.add_listing({account: contract_owner}, {name: "HI Hostel", description: "The best hostel ever", address: "891 Amsterdam Ave, NYC", images: ["http://localhost:3000"], price: "10000000"});
    const query_response = await contract.query.get_listings({page: 0, page_size: 50});
    const query_index = await contract.query.get_index_of_listing({id: query_response[0][0].id})
    console.log(query_index)
  })

  it("Confirm listing at index", async () => {
    const { contract_owner, other, contract} = await setup();
    const deploy_response = await contract.deploy(contract_owner);

    const contract_info = await contract.instantiate({ prng_seed: "dGVzdA=="}, "deploy test", contract_owner);
    const ex_response = await contract.tx.add_listing({account: contract_owner}, {name: "HI Hostel", description: "The best hostel ever", address: "891 Amsterdam Ave, NYC", images: ["http://localhost:3000"], price: "10000000"});
    const query_response = await contract.query.get_listings({page: 0, page_size: 50});
    const query_index = await contract.query.get_index_of_listing({id: query_response[0][0].id});
    const confirmation = await contract.tx.confirm_listing({account: contract_owner}, {id: query_index.Ok, start: 0, end: 1000});
    const decoder = new TextDecoder();
    console.log(decoder.decode(Buffer.from(confirmation.data)))
  })

  it("Make viewing key", async () => {
    const {contract_owner, other, contract} = await setup();
    const deploy_response = await contract.deploy(contract_owner);
    const contract_info = await contract.instantiate({ prng_seed: "dGVzdA=="}, "deploy test", contract_owner);

    const rand = randomBytes(32)
    const vk_response = await contract.tx.create_viewing_key({account: contract_owner}, {entropy: rand.toString("base64")})
    const decoder = new TextDecoder();
    console.log(decoder.decode(Buffer.from(vk_response.data)));
  })

  it("Pull confirmation without VK", async () => {
    const {contract_owner, other, contract} = await setup();
    const deploy_response = await contract.deploy(contract_owner);
    const contract_info = await contract.instantiate({ prng_seed: "dGVzdA=="}, "deploy test", contract_owner);
    const ex_response = await contract.tx.add_listing({account: contract_owner}, {name: "HI Hostel", description: "The best hostel ever", address: "891 Amsterdam Ave, NYC", images: ["http://localhost:3000"], price: "10000000"});
    const query_response = await contract.query.get_listings({page: 0, page_size: 50});
    const query_index = await contract.query.get_index_of_listing({id: query_response[0][0].id});
    const confirmation = await contract.tx.confirm_listing({account: contract_owner}, {id: query_index.Ok, start: 0, end: 1000});

    // const rand = randomBytes(32)
    // const vk_response = await contract.tx.create_viewing_key({account: contract_owner}, {entropy: rand.toString("base64")})
    // const decoder = new TextDecoder();
    // const vk = JSON.parse(decoder.decode(Buffer.from(vk_response.data)))

    const confirmations = await contract.query.get_confirmations({page: 0, page_size: 50, address: contract_owner.account.address, vk: "penis"})
    console.log(confirmations);
  })

  it("Pull confirmation with VK", async () => {
    const {contract_owner, other, contract} = await setup();
    const deploy_response = await contract.deploy(contract_owner);
    const contract_info = await contract.instantiate({ prng_seed: "dGVzdA=="}, "deploy test", contract_owner);
    const ex_response = await contract.tx.add_listing({account: contract_owner}, {name: "HI Hostel", description: "The best hostel ever", address: "891 Amsterdam Ave, NYC", images: ["http://localhost:3000"], price: "10000000"});
    const query_response = await contract.query.get_listings({page: 0, page_size: 50});
    const query_index = await contract.query.get_index_of_listing({id: query_response[0][0].id});
    const confirmation = await contract.tx.confirm_listing({account: contract_owner}, {id: query_index.Ok, start: 0, end: 1000});

    const rand = randomBytes(32)
    const vk_response = await contract.tx.create_viewing_key({account: contract_owner}, {entropy: rand.toString("base64")})
    const decoder = new TextDecoder();
    const vk = JSON.parse(decoder.decode(Buffer.from(vk_response.data)))

    const confirmations = await contract.query.get_confirmations({page: 0, page_size: 50, address: contract_owner.account.address, vk: vk.create_viewing_key.key})
    console.log(confirmations);
  })

});

// describe("Get listing", () => {
//   async function setup() {
//     const contract_owner = getAccountByName("account_1");
//     const other = getAccountByName("account_0");
//     const contract = new Contract("DBnB");
//     await contract.parseSchema();
//
//     return { contract_owner, other, contract };
//   }
//
//   it("deploy and init", async () => {
//     const { contract_owner, other, contract } = await setup();
//     const deploy_response = await contract.deploy(contract_owner);
//
//     const contract_info = await contract.instantiate({"factorial": 0}, "deploy test", contract_owner);
//
//     await expect(contract.query.get_factorial()).to.respondWith({ 'factorial': 0 });
//   });
//
//   it("calculate factorial", async () => {
//     const { contract_owner, other, contract } = await setup();
//     const deploy_response = await contract.deploy(contract_owner);
//
//     const contract_info = await contract.instantiate({"factorial": 0}, "deploy test", contract_owner);
//
//     const ex_response = await contract.tx.factorial({account: contract_owner}, 4);
//     await expect(contract.query.get_factorial()).to.respondWith({ 'factorial': 24 });
//   });
//
//   it("recalculate factorial", async () => {
//     const { contract_owner, other, contract } = await setup();
//     const deploy_response = await contract.deploy(contract_owner);
//
//     const contract_info = await contract.instantiate({"factorial": 0}, "deploy test", contract_owner);
//
//     const ex_response = await contract.tx.factorial({account: contract_owner}, 4);
//     await expect(contract.query.get_factorial()).to.respondWith({ 'factorial': 24 });
//
//     const ex_response_1 = await contract.tx.factorial({account: contract_owner}, 3);
//     await expect(contract.query.get_factorial()).to.respondWith({ 'factorial': 6 });
//   });
// });
