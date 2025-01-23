use cosmwasm_std::{Reply, StdError, SubMsgResult, DepsMut, Env, Response, StdResult, from_json, Uint128};
use router_wasm_bindings::{RouterMsg, RouterQuery, types::CrosschainRequestResponse};

use crate::{
    execution::{ACK_GAS_LIMIT, ACK_GAS_PRICE, MINIMUM_RELAYER_FEES}, 
    state::{
        CREATE_OUTBOUND_REQUEST, CURRENT_FEE_PAYER, EVENT, FEE_TANK, REQUEST_ID_FEE_PAYER
    }
};

pub fn handle_reply(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    msg: Reply,
) -> StdResult<Response<RouterMsg>> {
    match msg.id {
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
                        let req_dedecuted_fee: Uint128 = cross_chain_req_res.fee_deducted.amount;
                        let ack_deduct_fee = Uint128::new(ACK_GAS_PRICE as u128).checked_mul(Uint128::new(ACK_GAS_LIMIT as u128))?;
                        let relayer_fee_for_ack = Uint128::new(MINIMUM_RELAYER_FEES as u128);
                        let required_fee = req_dedecuted_fee.checked_add(ack_deduct_fee)?.checked_add(relayer_fee_for_ack)?;

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
