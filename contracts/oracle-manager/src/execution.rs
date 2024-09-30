use band_integration_package::oracle_manager::{
    IbcChannelInfo, InTransitToIbcCall, WhitelistCosmosChain,
};
use cosmwasm_std::{
    BankMsg, Binary, Coin, CosmosMsg, DepsMut, Env, Event, MessageInfo, Response, StdResult,
    Storage, Uint128,
};
use router_wasm_bindings::types::{AckType, RequestMetaData, NATIVE_DENOM};
use router_wasm_bindings::{RouterMsg, RouterQuery};
use solabi::encode;

use crate::modifers::is_admin_modifier;
use crate::state::{
    ADMIN, IN_TRANSIT_IBC_CALLS, TEMP_INCOMING_IBC_CALL, TEMP_OUTGOING_IBC_CALL,
    WHITELISTED_IBC_CHANNELS,
};

pub fn is_native(token: &String) -> bool {
    let native_denom: &str = NATIVE_DENOM;
    if token.as_str() == native_denom {
        return true;
    }

    false
}

pub fn is_ibc(token: &String) -> bool {
    if token.starts_with("ibc/") {
        return true;
    }

    false
}

pub fn validate_funds(info: &MessageInfo) -> StdResult<(Uint128, String, Uint128)> {
    assert_eq!(info.funds.len() < 3, true, "Funds length should be 1 or 2");
    assert_eq!(info.funds.len() != 0, true, "Funds length should be 1 or 2");

    let mut native_amount: Uint128 = Uint128::zero();
    let mut ibc_amount: Uint128 = Uint128::zero();
    let mut ibc_token_address: String = String::default();

    if info.funds.len() == 1 {
        let fund: &cosmwasm_std::Coin = &info.funds[0];
        if fund.denom == NATIVE_DENOM {
            native_amount = fund.amount;
        } else {
            ibc_token_address = fund.denom.clone();
            ibc_amount = fund.amount.clone();
        }
    }
    if info.funds.len() == 2 {
        let fund0: &cosmwasm_std::Coin = &info.funds[0];
        let fund1: &cosmwasm_std::Coin = &info.funds[1];

        assert_eq!(fund0.denom, NATIVE_DENOM, "Native Coins are required");
        assert_eq!(is_ibc(&fund1.denom), true, "IBC is required");
        native_amount = fund0.amount;
        ibc_token_address = fund1.denom.clone();
        ibc_amount = fund1.amount.clone();
    }

    Ok((native_amount, ibc_token_address, ibc_amount))
}

pub fn update_admin(
    deps: DepsMut<RouterQuery>,
    info: &MessageInfo,
    new_admin: String,
) -> StdResult<Response<RouterMsg>> {
    let current_admin: String = ADMIN.load(deps.storage)?;

    assert_eq!(current_admin, info.sender.to_string());

    deps.api.addr_validate(&new_admin)?;
    ADMIN.save(deps.storage, &new_admin)?;

    Ok(Response::new())
}

pub fn withdraw_funds(
    deps: DepsMut<RouterQuery>,
    _env: &Env,
    info: &MessageInfo,
    denom: String,
    recipient: String,
    amount: Uint128,
) -> StdResult<Response<RouterMsg>> {
    is_admin_modifier(deps.as_ref(), &info.sender.to_string())?;

    let bank_msg = BankMsg::Send {
        to_address: recipient.into(),
        amount: vec![Coin { amount, denom }],
    };

    let res = Response::new()
        .add_message(bank_msg)
        .add_attribute("action", "SetGasFactor");
    Ok(res)
}

pub fn receive_band_data(
    _deps: DepsMut<RouterQuery>,
    _env: &Env,
    info: &MessageInfo,
    dest_chain_id: String,
    dest_contract_address: String,
    gas_limit: u64,
    gas_price: u64,
    payload: Binary,
    _nonce: u64,
    _signature: String,
) -> StdResult<Response<RouterMsg>> {
    let caller: String = info.sender.to_string();

    // add a sender to the payload and encode it
    let payload_with_caller_on_router = encode(&(
        caller,
        solabi::Bytes(payload.0),
    ));

    // add a destination contract address
    let request_packet = encode(&(
        dest_contract_address,
        solabi::Bytes(payload_with_caller_on_router),
    ));

    let request_metadata: RequestMetaData = RequestMetaData {
        dest_gas_limit: gas_limit,
        dest_gas_price: gas_price,
        ack_gas_limit: 300_000,
        ack_gas_price: 10_000_000,
        relayer_fee: Uint128::zero(),
        ack_type: AckType::AckOnBoth,
        is_read_call: false,
        asm_address: String::default(),
    };

    let i_send_request: RouterMsg = RouterMsg::CrosschainCall {
        version: 1,
        route_amount: Uint128::zero(),
        route_recipient: String::new(),
        dest_chain_id: dest_chain_id.clone(),
        request_metadata: request_metadata.get_abi_encoded_bytes(),
        request_packet,
    };

    let cross_chain_msg: CosmosMsg<RouterMsg> = i_send_request.into();

    let res: Response<RouterMsg> = Response::new()
        .add_attribute("action", "ReceiveIbcTokens")
        .add_message(cross_chain_msg);
    Ok(res)
}

pub fn whitelist_chains(
    deps: DepsMut<RouterQuery>,
    _env: &Env,
    info: &MessageInfo,
    ibc_info: Vec<WhitelistCosmosChain>,
) -> StdResult<Response<RouterMsg>> {
    is_admin_modifier(deps.as_ref(), &info.sender.to_string())?;

    for i in 0..ibc_info.len() {
        if ibc_info[i].remove {
            WHITELISTED_IBC_CHANNELS.remove(deps.storage, &ibc_info[i].chain_id);
            continue;
        }
        let object: IbcChannelInfo = IbcChannelInfo {
            incoming_port: ibc_info[i].incoming_port.clone(),
            incoming_channel: ibc_info[i].incoming_channel.clone(),
            outgoing_port: ibc_info[i].outgoing_port.clone(),
            outgoing_channel: ibc_info[i].outgoing_channel.clone(),
            timeout_height: ibc_info[i].timeout_height,
            timeout_timestamp: ibc_info[i].timeout_timestamp,
        };
        WHITELISTED_IBC_CHANNELS.save(deps.storage, &ibc_info[i].chain_id, &object)?;
    }
    let event_name: String = String::from("WhitelistCosmosChains");
    let format_str: String = format!("ibc_info {:?}", ibc_info);
    deps.api.debug(&format_str);
    let white_list_event: Event = Event::new(event_name).add_attribute("call_data", format_str);

    let res = Response::new()
        .add_attribute("action", "WhitelistCosmosChains")
        .add_event(white_list_event);
    Ok(res)
}

pub fn store_awaiting_ibc_transfer(
    deps: DepsMut<RouterQuery>,
    sequence: u64,
    data: &InTransitToIbcCall,
) -> StdResult<()> {
    IN_TRANSIT_IBC_CALLS.save(deps.storage, (&data.source_channel.clone(), sequence), data)
}

pub fn clear_temp_states(storage: &mut dyn Storage) {
    TEMP_INCOMING_IBC_CALL.remove(storage);
    TEMP_OUTGOING_IBC_CALL.remove(storage);
}
