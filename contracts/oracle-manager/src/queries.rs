use std::marker::PhantomData;

use band_integration_package::oracle_manager::{
    IbcChannelInfo, InTransitToIbcCall, TempIncomingCalls, TempOutgoingCalls,
};
use cosmwasm_std::{AllBalanceResponse, BankQuery, Deps, Order, StdResult};
use cw_storage_plus::Bound;
use router_wasm_bindings::RouterQuery;

use crate::state::{
    ADMIN, IN_TRANSIT_IBC_CALLS, TEMP_INCOMING_IBC_CALL, TEMP_OUTGOING_IBC_CALL,
    WHITELISTED_IBC_CHANNELS,
};

pub fn fetch_admin(deps: Deps<RouterQuery>) -> StdResult<String> {
    ADMIN.load(deps.storage)
}
pub fn fetch_incoming_ibc_state(deps: Deps<RouterQuery>) -> StdResult<TempIncomingCalls> {
    TEMP_INCOMING_IBC_CALL.load(deps.storage)
}

pub fn load_awaiting_ibc_transfer(
    deps: Deps<RouterQuery>,
    source_channel: &str,
    sequence: u64,
) -> StdResult<InTransitToIbcCall> {
    IN_TRANSIT_IBC_CALLS.load(deps.storage, (source_channel, sequence))
}

pub fn load_temp_outgoing_ibc_transfer(deps: Deps<RouterQuery>) -> StdResult<TempOutgoingCalls> {
    TEMP_OUTGOING_IBC_CALL.load(deps.storage)
}

pub fn load_temp_incoming_ibc_transfer(deps: Deps<RouterQuery>) -> StdResult<TempIncomingCalls> {
    TEMP_INCOMING_IBC_CALL.load(deps.storage)
}

pub fn fetch_balance(deps: Deps<RouterQuery>, addr: String) -> StdResult<AllBalanceResponse> {
    deps.querier
        .query(&BankQuery::AllBalances { address: addr }.into())
}

pub fn fetch_intransit_calls(
    deps: Deps<RouterQuery>,
    start_key: Option<(String, u64)>,
    limit: Option<u64>,
) -> Vec<((String, u64), InTransitToIbcCall)> {
    let limit: u64 = limit.unwrap_or(100);
    let mut start: Option<Bound<(&str, u64)>> = None;
    let mut _key: (String, u64) = (String::default(), 0);
    if let Some(value) = start_key {
        _key = value;
        start = Some(Bound::Exclusive(((&_key.0, _key.1), PhantomData)));
    }
    match IN_TRANSIT_IBC_CALLS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit as usize)
        .collect()
    {
        Ok(data) => data,
        Err(_) => vec![],
    }
}

/**
 * @notice Used to fetch the whitelisted contract
 * @param   chain_id
 * @param   chain_type
*/
pub fn fetch_white_listed(deps: Deps<RouterQuery>, chain_id: &str) -> StdResult<IbcChannelInfo> {
    WHITELISTED_IBC_CHANNELS.load(deps.storage, chain_id)
}

/**
    @notice Used to fetch if all white listed contracts details
*/
pub fn fetch_white_listed_cosmos_chains_info(
    deps: Deps<RouterQuery>,
    start_key: Option<String>,
    limit: Option<u64>,
) -> StdResult<Vec<(String, IbcChannelInfo)>> {
    let limit: u64 = limit.unwrap_or(100);
    let mut start: Option<Bound<&str>> = None;
    let mut _key = String::default();
    if let Some(value) = start_key {
        _key = value;
        start = Some(Bound::Exclusive((&_key, PhantomData)));
    }
    match WHITELISTED_IBC_CHANNELS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit as usize)
        .collect()
    {
        Ok(data) => return Ok(data),
        Err(err) => return Err(err),
    };
}
