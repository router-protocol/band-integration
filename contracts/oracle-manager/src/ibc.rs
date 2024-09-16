use band_integration_package::oracle_manager::Status;
use cosmwasm_std::{DepsMut, Env, Event, Response, StdResult};

use router_wasm_bindings::{RouterMsg, RouterQuery};

use crate::{execution::store_awaiting_ibc_transfer, queries::load_awaiting_ibc_transfer};

pub fn receive_ack(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    source_channel: String,
    sequence: u64,
    success: bool,
) -> StdResult<Response<RouterMsg>> {
    let info_string: String = format!("Inside ReceiveAck");
    deps.api.debug(&info_string);
    let mut ibc_transfer_state =
        load_awaiting_ibc_transfer(deps.as_ref(), &source_channel, sequence)?;

    let info_string: String = format!("Inside ReceiveAck {:?} {:?}", sequence, ibc_transfer_state);
    deps.api.debug(&info_string);
    if !success {
        ibc_transfer_state.status = Some(Status::Failed);
    } else {
        ibc_transfer_state.status = Some(Status::Completed);
    }
    store_awaiting_ibc_transfer(deps, sequence, &ibc_transfer_state)?;

    Ok(Response::new().add_event(Event::new("IBCReceiveAck success")))
}

pub fn receive_timeout(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    source_channel: String,
    sequence: u64,
) -> StdResult<Response<RouterMsg>> {
    let mut ibc_transfer_state =
        load_awaiting_ibc_transfer(deps.as_ref(), &source_channel, sequence)?;

    ibc_transfer_state.status = Some(Status::Failed);
    store_awaiting_ibc_transfer(deps, sequence, &ibc_transfer_state)?;

    Ok(Response::new().add_event(Event::new("IBCReceiveTimeOut success")))
}

// fn apply_fallback_logic(
//     storage: &mut dyn Storage,
//     event_nonce: u64,
//     source_channel: &String,
//     sequence: u64,
//     local_fallback_address: String,
//     denom: String,
//     amount: Uint128,
//     is_expired: bool,
//     is_event_packet: bool,
// ) -> Result<Response, IbcTrackingError> {
//     let msg = BankMsg::Send {
//         to_address: local_fallback_address,
//         amount: vec![Coin {
//             denom,
//             amount,
//         }],
//     };

//     if !is_event_packet {
//         remove_awaiting_ibc_transfer(storage, &source_channel, sequence)?;
//     } else {
//         let mut status = PacketStatus::AckFailed;
//         if is_expired{
//             status = PacketStatus::Expired;
//         }

//         PACKET_STATUS.save(storage, event_nonce, &PacketInfo{
//             status,
//             channel: Some(source_channel.clone()),
//             sequence: Some(sequence),
//         })?;
//     }

//     Ok(Response::new()
//         .add_submessage(SubMsg::new(msg))
//         .add_event(
//             Event::new("IBCTransferFailed")
//                 .add_attribute("channel", source_channel)
//                 .add_attribute("sequence", sequence.to_string())
//                 .add_attribute("nonce", event_nonce.to_string())
//         ))
// }
