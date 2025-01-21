use band_integration_package::oracle_manager::{InTransitToIbcCall, TempOutgoingCalls};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{Reply, StdError, SubMsgResult, DepsMut, Env, Response, StdResult, from_json, Uint128};
use ibc_tracking::msg::MsgTransferResponse;
use prost::Message;
use router_wasm_bindings::{RouterMsg, RouterQuery, types::CrosschainRequestResponse};

use crate::handle_revert::revert_funds_to_ibc_chain;
use crate::queries::load_temp_outgoing_ibc_transfer;
use crate::state::{
    CREATE_IBC_TANSFER, CREATE_OUTBOUND_REQUEST, HANDLE_INBOUND_IBC_TOKENS, IN_TRANSIT_IBC_CALLS, TEMP_INCOMING_IBC_CALL,
    CURRENT_FEE_PAYER, FEE_TANK, EVENT, REQUEST_ID_FEE_PAYER,
};

pub fn handle_reply(
    deps: DepsMut<RouterQuery>,
    env: Env,
    msg: Reply,
) -> StdResult<Response<RouterMsg>> {
    match msg.id {
        CREATE_IBC_TANSFER => {
            let info_str: String = format!("msg_id {:?}, msg_result: {:?}", msg.id, msg.result);
            deps.api.debug(&info_str);
            // TODO: need to handle nonce data here, logic depends on the msg binary data structure.
            match msg.result {
                SubMsgResult::Ok(msg_result) => match msg_result.data {
                    Some(binary_data) => {
                        deps.api.debug("Binary Data Found");
                        deps.api.debug(&binary_data.to_string());
                        let ibc_transfer_response = MsgTransferResponse::decode(&binary_data[..])
                            .map_err(|_e| {
                            StdError::generic_err(format!(
                                "Failed to decode ibc transfer response: {binary_data}"
                            ))
                        })?;
                        let temp_call: TempOutgoingCalls =
                            load_temp_outgoing_ibc_transfer(deps.as_ref())?;

                        let data: InTransitToIbcCall = InTransitToIbcCall {
                            source_channel: temp_call.source_channel.clone(),
                            src_chain_id: temp_call.src_chain_id.clone(),
                            dest_chain_id: temp_call.dest_chain_id.clone(),
                            sender: temp_call.sender.clone(),
                            recipient: temp_call.recipient.clone(),
                            token_addr: temp_call.token_addr.clone(),
                            amount: temp_call.amount,
                            timeout_timestamp: env.block.time.seconds() + 1800,
                            status: None,
                        };
                        IN_TRANSIT_IBC_CALLS.save(
                            deps.storage,
                            (&temp_call.source_channel, ibc_transfer_response.sequence),
                            &data,
                        )?;
                    }
                    None => deps.api.debug("No Binary Data Found"),
                },
                SubMsgResult::Err(err) => deps.api.debug(&err.to_string()),
            }
        }
        HANDLE_INBOUND_IBC_TOKENS => {
            let info_str: String = format!("msg_id {:?}, msg_result: {:?}", msg.id, msg.result);
            deps.api.debug(&info_str);
            // TODO: need to handle nonce data here, logic depends on the msg binary data structure.
            match msg.result {
                SubMsgResult::Ok(_) => {
                    deps.api.debug("Removing TEMP_INCOMING_IBC_CALL state");
                    TEMP_INCOMING_IBC_CALL.remove(deps.storage);
                }
                SubMsgResult::Err(err) => {
                    let debug_str: String = format!("Inside HANDLE_INBOUND_IBC_TOKENS Reply Error Case, Need to Revert Tokens Err:-{:?}", err.to_string());
                    deps.api.debug(&debug_str);
                    return revert_funds_to_ibc_chain(deps, &env, &err.to_string());
                }
            }
        }
        CREATE_OUTBOUND_REQUEST => {
            deps.api.debug(&msg.id.to_string());
            let response: Response<RouterMsg> = Response::new();
            match msg.result {
                SubMsgResult::Ok(msg_result) => match msg_result.data {
                    Some(binary_data) => {
                        deps.api.debug("Binary Data Found");
                        let cross_chain_req_res: CrosschainRequestResponse =
                            from_json(&binary_data).unwrap();

                        let info_str: String = format!(
                            "Binary data {:?}, response {:?}",
                            &binary_data.to_string(),
                            cross_chain_req_res
                        );
                        deps.api.debug(&info_str);

                        let fee_payer: String = CURRENT_FEE_PAYER.load(deps.storage)?;
                        let available_fee: Uint128 =
                            FEE_TANK.load(deps.storage, &fee_payer).unwrap_or_default();
                        let required_fee: Uint128 = cross_chain_req_res.fee_deducted.amount;
                        if required_fee > available_fee {
                            let error: String = format!("Please provide sufficient fee, AvailableFee {:?}, RequiredFee {:?}", available_fee, required_fee);
                            return StdResult::Err(StdError::GenericErr { msg: error });
                        }
                        FEE_TANK.save(deps.storage, &fee_payer, &(available_fee - required_fee))?;
                        CURRENT_FEE_PAYER.remove(deps.storage);

                        REQUEST_ID_FEE_PAYER.save(
                            deps.storage,
                            cross_chain_req_res.request_identifier,
                            &fee_payer,
                        )?;
                        let event = EVENT.load(deps.storage)?;
                        EVENT.remove(deps.storage);

                        return Ok(response.add_event(event));
                    }
                    None => deps.api.debug("No Binary Data Found"),
                },
                SubMsgResult::Err(err) => deps.api.debug(&err.to_string()),
            }
        }
        id => return Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
    Ok(Response::new())
}

// pub fn handle_ibc_transfer_reply(deps: DepsMut<RouterQuery>, reply: Reply) -> StdResult<Response> {
//     let SubMsgResult::Ok(SubMsgResponse { data: Some(b), .. }) = reply.result else {
//         return Err(StdError::generic_err("ibc transfer reply failed".to_string()));
//     };

//     let ibc_transfer_response: MsgTransferResponse = from_json(&b).unwrap();
//     let ibc_transfer_info = load_temp_ibc_transfer(deps.as_ref())?;
//     store_awaiting_ibc_transfer(
//         deps,
//         ibc_transfer_response.sequence,
//         &ibc_transfer_info,
//     )?;

//     Ok(Response::new().add_event(
//         Event::new("IBC transferred")
//             .add_attribute("channel", ibc_transfer_info.channel)
//             .add_attribute("sequence", ibc_transfer_response.sequence.to_string()),
//     ))
// }
