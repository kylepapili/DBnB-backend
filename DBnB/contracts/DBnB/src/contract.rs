use hex;

use cosmwasm_std::{debug_print, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError, StdResult, Storage, Uint128, HandleResult, HumanAddr};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use secret_toolkit::storage::{AppendStore, AppendStoreMut};
use secret_toolkit::viewing_key::{ViewingKey, ViewingKeyStore};

use crate::rand::{Prng};
use crate::msg::{HandleAnswer, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, Confirmation, CONFIRMATIONS, Listing, LISTING_IDS, LISTINGS_KEY, State};


pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        owner: deps.api.canonical_address(&env.message.sender)?,
        prng_seed: msg.prng_seed
    };

    config(&mut deps.storage).save(&state)?;

    debug_print!("Contract was initialized by {}", env.message.sender);

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::AddListing { name, description, address, images, price } => add_listing(deps, env, name, description, address, images, price),
        HandleMsg::ConfirmListing {id, start, end} => confirm_listing(deps, env, id, start, end),
        HandleMsg::CreateViewingKey {entropy} => create_viewing_key(deps, env, entropy)
    }
}

pub fn create_viewing_key<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    entropy: String
) -> HandleResult {
    let key = ViewingKey::create(&mut deps.storage, &env, &env.message.sender, entropy.as_bytes());

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::CreateViewingKey { key })?)
    })
}

pub fn confirm_listing<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    index: u32,
    start: u64,
    end: u64
) -> HandleResult {
    let mut store = PrefixedStorage::new(LISTINGS_KEY, &mut deps.storage);
    let store = AppendStoreMut::<Listing, _>::attach_or_create(&mut store)?;




    return match store.get_at(index) {
        Ok(listing) => {
            let canon_addr = deps.api.canonical_address(&env.message.sender)?;
            let mut confirmation_store = PrefixedStorage::multilevel(&[CONFIRMATIONS, canon_addr.clone().as_slice()], &mut deps.storage);
            let mut confirmation_store = AppendStoreMut::<Confirmation, _>::attach_or_create(&mut confirmation_store)?;

            let confirmation = confirmation_store.iter().map(|x| x.unwrap()).find(|x| x.id == listing.id);

            return match confirmation {
                None => {
                    let confirmation = Confirmation {
                        id: listing.id,
                        addr: canon_addr,
                        start,
                        end
                    };
                    confirmation_store.push(&confirmation);
                    Ok(HandleResponse {
                        messages: vec![],
                        log: vec![],
                        data: Some(to_binary(&HandleAnswer::ListingConfirmation { booked: true})?)
                    })
                }
                Some(confirm) => {
                    Err(StdError::generic_err("Listing already booked."))
                }
            };
        }
        Err(x) => {
            Err(x)
        }
    };
}

pub fn add_listing<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    name: String,
    description: String,
    address: String,
    images: Vec<String>,
    price: Uint128
) -> HandleResult {
    let config = config_read(&deps.storage).load()?;

    let entropy_len = 48 + env.message.sender.len();
    let mut rng_entropy = Vec::with_capacity(entropy_len);
    rng_entropy.extend_from_slice(&env.block.height.to_be_bytes());
    rng_entropy.extend_from_slice(&env.block.time.to_be_bytes());
    rng_entropy.extend_from_slice(&env.message.sender.0.as_bytes());
    rng_entropy.extend_from_slice(config.prng_seed.as_slice());

    let mut rng = Prng::new(config.prng_seed.as_slice(), &rng_entropy);

    let rand_data = rng.rand_bytes();
    let hexed_rand = hex::encode(rand_data);

    let owner_canon = deps.api.canonical_address(&env.message.sender)?;
    let listing = Listing {
        id: hexed_rand.clone(),
        owner: owner_canon,
        name,
        description,
        address,
        images,
        price,
    };

    let mut store = PrefixedStorage::new(LISTINGS_KEY, &mut deps.storage);
    let mut store = AppendStoreMut::attach_or_create(&mut store)?;

    // let mut store = listings(&mut deps.storage);
    store.push(&listing)?;

    let mut store = PrefixedStorage::new(LISTING_IDS, &mut deps.storage);
    let mut store = AppendStoreMut::attach_or_create(&mut store)?;

    // let mut listing_ids = listing_ids(&mut deps.storage);
    store.push(&hexed_rand)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: None
    })
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetListings { page, page_size } => to_binary(&query_listings(deps, page, page_size)?),
        QueryMsg::GetIndexOfListing { id} => to_binary(&find_listing_id(&deps, id)),
        QueryMsg::GetConfirmations { page, page_size, address, vk } => to_binary(&query_confirmations(deps, page, page_size, address, vk))
    }
}

fn query_confirmations<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    page: u32,
    page_size: u32,
    address: HumanAddr,
    vk: String
) -> StdResult<Vec<Confirmation>> {

    return match ViewingKey::check(&deps.storage, &address, &vk) {
        Ok(_) => {
            let canon_addr = deps.api.canonical_address(&address)?;
            let store = ReadonlyPrefixedStorage::multilevel(&[CONFIRMATIONS, canon_addr.as_slice()], &deps.storage);
            let store = AppendStore::attach(&store);

            let store = if let Some(result) = store {
                result?
            } else {
                return Ok(vec![])
            };

            let listing: Vec<Confirmation> = store
                .iter()
                .rev()
                .skip((page * page_size) as _)
                .take(page_size as _).map(|x| x.unwrap()).collect();

            Ok(listing)

        }
        Err(x) => {
            Err(x)
        }
    }
}

fn find_listing_id<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    id: String
) -> StdResult<u32> {
    let store = ReadonlyPrefixedStorage::new(LISTING_IDS, &deps.storage);
    let store = AppendStore::<String, _>::attach(&store);

    let store = if let Some(result) = store {
        result?
    } else {
        return Err(StdError::generic_err("ID NOT IN LISTINGS"))
    };

    for i in 0..store.len() {
        let thing = store.get_at(i).unwrap();
        if thing == id {
            return Ok(i)
        }
    }
    Err(StdError::generic_err("Unable to find specific listing."))
}

fn query_listings<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    page: u32,
    page_size: u32,
) -> StdResult<(Vec<Listing>, u64)> {
    // let store = listings_read(&deps.storage);
    let store = ReadonlyPrefixedStorage::new(LISTINGS_KEY, &deps.storage);
    let store = AppendStore::attach(&store);

    let store = if let Some(result) = store {
        result?
    } else {
        return Ok((vec![], 0))
    };

    let listing: Vec<Listing> = store
        .iter()
        .rev()
        .skip((page * page_size) as _)
        .take(page_size as _).map(|x| x.unwrap()).collect();

    Ok((listing, store.len() as u64))
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(20, &[]);

        let msg = InitMsg { prng_seed: Binary("test==".into()) };
        let env = mock_env("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg { prng_seed: Binary("test==".into()) };
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        // anyone can increment
        let env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::Increment {};
        let _res = handle(&mut deps, env, msg).unwrap();

        // should increase counter by 1
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg { prng_seed: Binary("test==".into()) };
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        // not anyone can reset
        let unauth_env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::Reset { count: 5 };
        let res = handle(&mut deps, unauth_env, msg);
        match res {
            Err(StdError::Unauthorized { .. }) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_env = mock_env("creator", &coins(2, "token"));
        let msg = HandleMsg::Reset { count: 5 };
        let _res = handle(&mut deps, auth_env, msg).unwrap();

        // should now be 5
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
