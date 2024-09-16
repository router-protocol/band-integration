use cosmwasm_std::{Deps, StdError, StdResult};
use router_wasm_bindings::RouterQuery;

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
