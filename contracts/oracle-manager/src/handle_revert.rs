use band_integration_package::oracle_manager::{
    IbcChannelInfo, TempIncomingCalls, TempOutgoingCalls,
};
use cosmwasm_std::{Coin, DepsMut, Env, ReplyOn, Response, StdResult, SubMsg};
use ibc_tracking::msg::MsgTransfer;
use router_wasm_bindings::{RouterMsg, RouterQuery};

use crate::{
    queries::load_temp_incoming_ibc_transfer,
    state::{
        CREATE_IBC_TANSFER, TEMP_INCOMING_IBC_CALL, TEMP_OUTGOING_IBC_CALL,
        WHITELISTED_IBC_CHANNELS,
    },
    utils::include_callback_inside_memo,
};

pub fn revert_funds_to_ibc_chain(
    deps: DepsMut<RouterQuery>,
    env: &Env,
    _err: &str,
) -> StdResult<Response<RouterMsg>> {
    let temp_inbound_req_data: TempIncomingCalls = load_temp_incoming_ibc_transfer(deps.as_ref())?;
    let ibc_channel_info: IbcChannelInfo =
        WHITELISTED_IBC_CHANNELS.load(deps.storage, &temp_inbound_req_data.src_chain_id)?;
    let mut timeout_timestamp: Option<u64> = None;
    if ibc_channel_info.timeout_timestamp != 0 {
        timeout_timestamp = Some(
            env.block
                .time
                .plus_seconds(ibc_channel_info.timeout_timestamp)
                .nanos(),
        );
    }
    let memo = include_callback_inside_memo(&env)?;
    let ibc_transfer = MsgTransfer {
        source_port: ibc_channel_info.outgoing_port.clone(),
        source_channel: ibc_channel_info.outgoing_channel.clone(),
        token: Some(
            Coin {
                denom: temp_inbound_req_data.token_addr.clone(),
                amount: temp_inbound_req_data.amount,
            }
            .into(),
        ),
        sender: env.contract.address.to_string(),
        receiver: temp_inbound_req_data.local_fallback_address.clone(),
        timeout_height: None,
        timeout_timestamp,
        memo,
    };

    let ibc_fallback_state = TempOutgoingCalls {
        source_channel: ibc_channel_info.outgoing_channel.clone(),
        src_chain_id: env.block.chain_id.clone(),
        dest_chain_id: temp_inbound_req_data.src_chain_id.clone(),
        amount: temp_inbound_req_data.amount,
        sender: env.contract.address.to_string(),
        recipient: temp_inbound_req_data.local_fallback_address.clone(),
        token_addr: temp_inbound_req_data.token_addr.clone(),
        // recipient,
    };

    TEMP_INCOMING_IBC_CALL.remove(deps.storage);
    TEMP_OUTGOING_IBC_CALL.save(deps.storage, &ibc_fallback_state)?;

    let info_str: String = format!("{:?}", ibc_transfer);
    deps.api.debug(&info_str);

    let res = Response::new()
        .add_attribute("action", "send_ibc_tokens")
        .add_submessage(SubMsg {
            id: CREATE_IBC_TANSFER,
            msg: ibc_transfer.into(),
            gas_limit: None,
            reply_on: ReplyOn::Always,
        });
    Ok(res)
}
