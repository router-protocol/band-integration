use crate::{
    handle_revert::revert_inbound_to_src_chain,
    queries::fetch_chain_bytes_from_chain_info,
    state::{CREATE_OUTBOUND_REPLY_ID, TEMP_STATE_CREATE_OUTBOUND_REPLY_ID},
    utils::fetch_and_validate_dest_contract,
};

use cosmwasm_std::{DepsMut, Env, ReplyOn, Response, StdResult, SubMsg, Uint128};
use router_wasm_bindings::{
    types::{AckType, RequestMetaData},
    Bytes, RouterMsg, RouterQuery,
};
use solabi::encode;
use solabi::{encode::Encode, U256};

pub fn create_reserve_outbound_request<T: Encode>(
    deps: DepsMut<RouterQuery>,
    env: &Env,
    src_chain_id: String,
    dest_chain_id: String,
    recipient_token: T,
    dest_token_obj: T,
    dest_token_amount: u128,
    deposit_nonce: u64,
    optinal_arabitrary_token: Option<solabi::Bytes<Vec<u8>>>,
    gas_info: [u64; 2],
) -> StdResult<Response<RouterMsg>> {
    deps.api.debug("Inside the send_reserve_outbound_request");
    let info_str: String = format!("received gas info {:?}", gas_info);
    deps.api.debug(&info_str);
    let destination_contract_address: String =
        match fetch_and_validate_dest_contract(deps.as_ref(), dest_chain_id.clone()) {
            Ok(address) => address,
            Err(_) => {
                return revert_inbound_to_src_chain(deps, env);
            }
        };
    let src_chain_info_bytes: Bytes =
        fetch_chain_bytes_from_chain_info(deps.as_ref(), &src_chain_id)?;

    let mut fixed_bytes: [u8; 32] = [0; 32];
    fixed_bytes.copy_from_slice(&src_chain_info_bytes);

    let src_chain_info_bytes_token: solabi::Bytes<[u8; 32]> = solabi::Bytes(fixed_bytes);
    let contract_call_payload: Vec<u8>;
    let exec_swap_token = (
        recipient_token,
        dest_token_obj,
        U256::new(dest_token_amount),
        U256::new(deposit_nonce as u128),
    );
    match optinal_arabitrary_token {
        Some(instruction_token) => {
            contract_call_payload = encode(&(
                U256::new(1u128),
                src_chain_info_bytes_token,
                exec_swap_token,
                instruction_token,
            ));
        }
        None => {
            contract_call_payload = encode(&(
                U256::new(0u128),
                src_chain_info_bytes_token,
                exec_swap_token,
            ));
        }
    }
    create_outbound(
        deps,
        env,
        destination_contract_address,
        contract_call_payload,
        dest_chain_id,
        gas_info,
    )
}

// fn create_evm_reserve_outbound_request(
//     deps: DepsMut<RouterQuery>,
//     env: &Env,
//     src_chain_id: String,
//     dest_chain_id: String,
//     recipient_token: Address,
//     dest_token_obj: Address,
//     dest_token_amount: u128,
//     deposit_nonce: u64,
//     optinal_arabitrary_token: Option<solabi::Bytes<Vec<u8>>>,
//     gas_info: [u64; 2],
// ) -> StdResult<Response<RouterMsg>> {
//     deps.api.debug("Inside the send_reserve_outbound_request");
//     let info_str: String = format!("received gas info {:?}", gas_info);
//     deps.api.debug(&info_str);
//     let destination_contract_address: String =
//         match fetch_and_validate_dest_contract(deps.as_ref(), dest_chain_id.clone()) {
//             Ok(address) => address,
//             Err(_) => {
//                 return revert_inbound_to_src_chain(deps, env);
//             }
//         };
//     let src_chain_info_bytes: Bytes =
//         fetch_chain_bytes_from_chain_info(deps.as_ref(), &src_chain_id)?;

//     let mut fixed_bytes: [u8; 32] = [0; 32];
//     fixed_bytes.copy_from_slice(&src_chain_info_bytes);

//     let src_chain_info_bytes_token: solabi::Bytes<[u8; 32]> = solabi::Bytes(fixed_bytes);
//     let contract_call_payload: Vec<u8>;
//     let exec_swap_token = (
//         recipient_token,
//         dest_token_obj,
//         U256::new(dest_token_amount),
//         U256::new(deposit_nonce as u128),
//     );
//     match optinal_arabitrary_token {
//         Some(instruction_token) => {
//             contract_call_payload = encode(&(
//                 U256::new(1u128),
//                 src_chain_info_bytes_token,
//                 exec_swap_token,
//                 instruction_token,
//             ));
//         }
//         None => {
//             contract_call_payload = encode(&(
//                 U256::new(0u128),
//                 src_chain_info_bytes_token,
//                 exec_swap_token,
//             ));
//         }
//     }
//     create_outbound(
//         deps,
//         env,
//         destination_contract_address,
//         contract_call_payload,
//         dest_chain_id,
//         gas_info,
//     )
// }

// fn create_non_evm_reserve_outbound_request(
//     deps: DepsMut<RouterQuery>,
//     env: &Env,
//     src_chain_id: String,
//     dest_chain_id: String,
//     recipient_token: String,
//     dest_token_obj: String,
//     dest_token_amount: u128,
//     deposit_nonce: u64,
//     optinal_arabitrary_token: Option<solabi::Bytes<Vec<u8>>>,
//     gas_info: [u64; 2],
// ) -> StdResult<Response<RouterMsg>> {
//     deps.api.debug("Inside the send_reserve_outbound_request");
//     let info_str: String = format!("received gas info {:?}", gas_info);
//     deps.api.debug(&info_str);
//     let destination_contract_address: String =
//         match fetch_and_validate_dest_contract(deps.as_ref(), dest_chain_id.clone()) {
//             Ok(address) => address,
//             Err(_) => {
//                 return revert_inbound_to_src_chain(deps, env);
//             }
//         };
//     let src_chain_info_bytes: Bytes =
//         fetch_chain_bytes_from_chain_info(deps.as_ref(), &src_chain_id)?;

//     let mut fixed_bytes: [u8; 32] = [0; 32];
//     fixed_bytes.copy_from_slice(&src_chain_info_bytes);

//     let src_chain_info_bytes_token: solabi::Bytes<[u8; 32]> = solabi::Bytes(fixed_bytes);
//     let contract_call_payload: Vec<u8>;
//     let exec_swap_token = (
//         recipient_token,
//         dest_token_obj,
//         U256::new(dest_token_amount),
//         U256::new(deposit_nonce as u128),
//     );
//     match optinal_arabitrary_token {
//         Some(instruction_token) => {
//             contract_call_payload = encode(&(
//                 U256::new(1u128),
//                 src_chain_info_bytes_token,
//                 exec_swap_token,
//                 instruction_token,
//             ));
//         }
//         None => {
//             contract_call_payload = encode(&(
//                 U256::new(0u128),
//                 src_chain_info_bytes_token,
//                 exec_swap_token,
//             ));
//         }
//     }
//     create_outbound(
//         deps,
//         env,
//         destination_contract_address,
//         contract_call_payload,
//         dest_chain_id,
//         gas_info,
//     )
// }

fn create_outbound(
    deps: DepsMut<RouterQuery>,
    _env: &Env,
    destination_contract_address: String,
    contract_payload: Vec<u8>,
    dest_chain_id: String,
    gas_info: [u64; 2],
) -> StdResult<Response<RouterMsg>> {
    let request_packet: Bytes = encode(&(
        destination_contract_address,
        solabi::Bytes(contract_payload),
    ));
    // TODO: revert zero gas limit
    let request_metadata: RequestMetaData = RequestMetaData {
        dest_gas_limit: 0,
        dest_gas_price: gas_info[1],
        ack_gas_limit: 300_000,
        ack_gas_price: 10_000_000,
        relayer_fee: Uint128::zero(),
        ack_type: AckType::AckOnBoth,
        is_read_call: false,
        asm_address: String::default(),
    };
    let i_send_request: RouterMsg = RouterMsg::CrosschainCall {
        version: 1,
        route_amount: Uint128::new(0u128),
        route_recipient: String::default(),
        dest_chain_id,
        request_metadata: request_metadata.get_abi_encoded_bytes(),
        request_packet,
    };
    TEMP_STATE_CREATE_OUTBOUND_REPLY_ID.save(deps.storage, &i_send_request)?;
    let outbound_submessage: SubMsg<RouterMsg> = SubMsg {
        gas_limit: None,
        id: CREATE_OUTBOUND_REPLY_ID,
        reply_on: ReplyOn::Success,
        msg: i_send_request.into(),
    };

    return Ok(Response::new().add_submessage(outbound_submessage));
}

pub fn send_failed_request(failed_request: RouterMsg) -> StdResult<Response<RouterMsg>> {
    let outbound_submessage: SubMsg<RouterMsg> = SubMsg {
        gas_limit: None,
        id: CREATE_OUTBOUND_REPLY_ID,
        reply_on: ReplyOn::Success,
        msg: failed_request.into(),
    };

    return Ok(Response::new().add_submessage(outbound_submessage));
}
