use cosmwasm_std::{Binary, Coin, DepsMut, Env, Response, StdResult, StdError};
use router_wasm_bindings::{
    types::NATIVE_DENOM, RouterMsg, RouterQuery
};

use crate::state::{REQUEST_ID_FEE_PAYER, FEE_TANK};

pub fn handle_sudo_ack(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    request_identifier: u64,
    _exec_flag: bool,
    _exec_data: Binary,
    refund_amount: Coin,
) -> StdResult<Response<RouterMsg>> {
    let fee_payer = REQUEST_ID_FEE_PAYER.load(deps.storage, request_identifier)?;

    let available_funds = FEE_TANK.load(deps.storage, &fee_payer)?;
    if refund_amount.denom != NATIVE_DENOM {
        return Err(StdError::GenericErr { msg: "denom mismathes".to_string()});
    }

    FEE_TANK.save(deps.storage, &fee_payer, &available_funds.checked_add(refund_amount.amount)?)?;
    REQUEST_ID_FEE_PAYER.remove(deps.storage, request_identifier);

    Ok(Response::new().add_attribute("action", "refund"))
}