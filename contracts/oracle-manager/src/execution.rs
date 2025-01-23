use cosmwasm_std::{
    Binary, DepsMut, Env, Event, MessageInfo, Response, StdResult,
    Uint128, SubMsg, ReplyOn, StdError,
};
use router_wasm_bindings::{
    ethabi::{decode, ParamType, Token}, types::{AckType, RequestMetaData}, RouterMsg, RouterQuery
};
use solabi::encode;

use crate::{
    modifers::is_valid_route_fund_modifier,
    state::{
        ADMIN, NEW_ADMIN, CREATE_OUTBOUND_REQUEST, CURRENT_FEE_PAYER, EVENT, FEE_PAYER, FEE_TANK,
    },
};

pub const MINIMUM_FEE: u128 = 10_000_000_000;
pub const ACK_GAS_PRICE: u64 = 50_000_000;     // it should be the same value w/ the InboundGasPrice param of crosschain module
pub const ACK_GAS_LIMIT: u64 = 300_000;
pub const MINIMUM_RELAYER_FEES: u64 = 25_000_000_000_000_000;

pub fn update_admin(
    deps: DepsMut<RouterQuery>,
    info: &MessageInfo,
    new_admin: String,
) -> StdResult<Response<RouterMsg>> {
    let current_admin: String = ADMIN.load(deps.storage)?;

    assert_eq!(current_admin, info.sender.to_string());

    deps.api.addr_validate(&new_admin)?;
    NEW_ADMIN.save(deps.storage, &new_admin)?;

    Ok(Response::new())
}

pub fn claim_admin(
    deps: DepsMut<RouterQuery>,
    _env: &Env,
    info: &MessageInfo,
) -> StdResult<Response<RouterMsg>> {
    let new_admin = NEW_ADMIN.load(deps.storage)?;

    assert_eq!(new_admin, info.sender.to_string());

    deps.api.addr_validate(&new_admin)?;
    ADMIN.save(deps.storage, &new_admin)?;

    Ok(Response::default())
}

pub fn receive_band_data(
    deps: DepsMut<RouterQuery>,
    _env: &Env,
    info: &MessageInfo,
    dest_chain_id: String,
    dest_contract_address: String,
    gas_limit: u64,
    gas_price: u64,
    payload: Binary,
) -> StdResult<Response<RouterMsg>> {
    // Define the ABI structure for the tuple
    let packet_type = ParamType::Tuple(vec![
        ParamType::Uint(64), // TunnelID
        ParamType::Uint(64), // Sequence
        ParamType::Array(Box::new(ParamType::Tuple(vec![
            // SignalPrices
            ParamType::FixedBytes(32), // SignalID
            ParamType::Uint(64),       // Price
        ]))),
        ParamType::Int(64), // CreatedAt
    ]);

    let fee_payer: String;
    let tokens = decode(&[packet_type], payload.as_slice()).unwrap();

    if let Some(Token::Tuple(values)) = tokens.get(0) {
        // Extract TunnelID
        if let Some(Token::Uint(tunnel_id)) = values.get(0) {
            fee_payer = FEE_PAYER.load(deps.storage, tunnel_id.as_u64())?;
            CURRENT_FEE_PAYER.save(deps.storage, &fee_payer)?;
        } else {
            return Err(StdError::generic_err("Failed to extract TunnelID").into());
        }
    } else {
        return Err(StdError::generic_err("Decoded token is not a tuple").into());
    }

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
        ack_gas_limit: ACK_GAS_LIMIT,
        ack_gas_price: ACK_GAS_PRICE,
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

    let cross_chain_msg = SubMsg {
        id: CREATE_OUTBOUND_REQUEST,
        msg: i_send_request.into(),
        gas_limit: None,
        reply_on: ReplyOn::Success,
    };

    let event: Event = Event::new("ReceiveBandDataEvent")
        .add_attribute("action", "ReceiveBandData")
        .add_attribute("fee_payer", fee_payer);
    EVENT.save(deps.storage, &event)?;

    let res: Response<RouterMsg> = Response::new()
        .add_attribute("action", "ReceiveIbcTokens")
        .add_submessage(cross_chain_msg);
    Ok(res)
}

pub fn register_fee_payer_or_fund(
    deps: DepsMut<RouterQuery>,
    info: &MessageInfo,
    tunnel_id: Option<u64>,
) -> StdResult<Response<RouterMsg>> {
    let sender: String = info.sender.to_string();

    let mut fund = Uint128::default();
    if is_valid_route_fund_modifier(info).is_ok() {
        fund = info.funds.get(0).unwrap().amount;
        let available_fee: Uint128 = FEE_TANK.load(deps.storage, &sender).unwrap_or_default();
        FEE_TANK.save(deps.storage, &sender, &(available_fee + fund))?;
    }

    // The sender can specifies the tunnel for which he would pay
    if tunnel_id.is_some() && fund.gt(&Uint128::new(MINIMUM_FEE))  {
        FEE_PAYER.save(deps.storage, tunnel_id.unwrap(), &sender)?;
    }

    Ok(Response::new().add_attribute("action", "register_fee_payer"))
}