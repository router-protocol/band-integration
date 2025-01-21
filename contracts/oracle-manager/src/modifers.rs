use cosmwasm_std::{Deps, StdError, StdResult, MessageInfo};
use router_wasm_bindings::{RouterQuery, types::NATIVE_DENOM};

use crate::{
    queries::fetch_admin,
    state::WHITELISTED_IBC_CHANNELS,
};

pub fn is_admin_modifier(deps: Deps<RouterQuery>, sender: &str) -> StdResult<()> {
    let admin: String = fetch_admin(deps)?; 
    if sender == &admin {
        return Ok(());
    }
    return StdResult::Err(StdError::GenericErr {
        msg: String::from("Auth: The caller is not Admin"),
    });
}

pub fn is_white_listed_modifier(
    deps: Deps<RouterQuery>,
    chain_id: String,
    contract: String,
) -> StdResult<()> {
    let is_white_listed_contract = WHITELISTED_IBC_CHANNELS.has(deps.storage, &chain_id);
    let info_str: String = format!("--chain_id: {:?}, contract: {:?}", chain_id, contract);
    deps.api.debug(&info_str);
    if !is_white_listed_contract {
        let info_str: String = format!(
            "Auth: The Sender/Receiver contract is not whitelisted, chain_id: {:?}, contract: {:?}",
            chain_id, contract
        );
        deps.api.debug(&info_str);
        return StdResult::Err(StdError::GenericErr { msg: info_str });
    }
    Ok(())
}

pub fn is_valid_route_fund_modifier(info: &MessageInfo) -> StdResult<()> {
    if info.funds.len() != 1 {
        return Err(StdError::GenericErr {
            msg: "funds length should be one".into(),
        });
    }
    if info.funds[0].denom != NATIVE_DENOM {
        return Err(StdError::GenericErr {
            msg: "only Native Denom  accepted".into(),
        });
    }
    Ok(())
}
