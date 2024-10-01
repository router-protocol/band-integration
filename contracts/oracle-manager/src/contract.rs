use band_integration_package::oracle_manager::{
    ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, SudoMsg,
};
use ibc_tracking::msg::IBCLifecycleComplete;
use router_wasm_bindings::{RouterMsg, RouterQuery};

#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::{to_json_binary, Reply, StdError};
use cw2::{get_contract_version, set_contract_version};

use crate::{
    execution::{
        clear_temp_states, receive_band_data, update_admin, whitelist_chains,
        withdraw_funds,
    },
    handle_reply::handle_reply,
    ibc::{receive_ack, receive_timeout},
    queries::{
        fetch_admin, fetch_balance, fetch_incoming_ibc_state, fetch_intransit_calls,
        fetch_white_listed, fetch_white_listed_cosmos_chains_info,
    },
    state::ADMIN,
};

// version info for migration info
const CONTRACT_NAME: &str = "BandProtocol::OracleManager";
const CONTRACT_VERSION: &str = "0.1.01";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    deps.api.debug("Instantiating the contract🚀");

    let caller: String = info.sender.to_string();
    ADMIN.save(deps.storage, &caller)?;

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_attribute("action", "BandProtocol::OracleManager::INIT"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut<RouterQuery>, env: Env, msg: SudoMsg) -> StdResult<Response<RouterMsg>> {
    let info_string: String = format!("Inside Sudo Invokation {:?}", msg);
    deps.api.debug(&info_string);
    clear_temp_states(deps.storage);
    match msg {
        SudoMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCAck {
            channel,
            sequence,
            success,
            ack: _,
        }) => receive_ack(deps, env, channel, sequence, success).map_err(|e| e.into()),
        SudoMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCTimeout { channel, sequence }) => {
            receive_timeout(deps, env, channel, sequence).map_err(|e| e.into())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<RouterQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<RouterMsg>> {
    // Clear Temp State
    clear_temp_states(deps.storage);
    match msg {
        ExecuteMsg::ReceiveBandData {
            dest_chain_id,
            dest_contract_address,
            gas_limit,
            gas_price,
            payload,
            nonce,
            signature,
        } => receive_band_data(
            deps,
            &env,
            &info,
            dest_chain_id,
            dest_contract_address,
            gas_limit,
            gas_price,
            payload,
            nonce,
            signature,
        ),
        ExecuteMsg::WhitelistCosmosChain { ibc_info } => {
            whitelist_chains(deps, &env, &info, ibc_info)
        }
        ExecuteMsg::WithdrawFunds {
            denom,
            recipient,
            amount,
        } => withdraw_funds(deps, &env, &info, denom, recipient, amount),
        ExecuteMsg::UpdateAdmin { new_admin } => update_admin(deps, &info, new_admin),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut<RouterQuery>, env: Env, msg: Reply) -> StdResult<Response<RouterMsg>> {
    handle_reply(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut<RouterQuery>, env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    let ver = cw2::get_contract_version(deps.storage)?;
    // ensure we are migrating from an allowed contract
    if ver.contract != CONTRACT_NAME.to_string() {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }
    // note: better to do proper semver compare, but string compare *usually* works
    if ver.version >= CONTRACT_VERSION.to_string() {
        return Err(StdError::generic_err("Cannot upgrade from a newer version").into());
    }

    let info_str: String = format!(
        "migrating contract: {}, new_contract_version: {}, contract_name: {}",
        env.contract.address,
        CONTRACT_VERSION.to_string(),
        CONTRACT_NAME.to_string()
    );
    deps.api.debug(&info_str);
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<RouterQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContractVersion {} => to_json_binary(&get_contract_version(deps.storage)?),
        QueryMsg::FetchAdmin {} => to_json_binary(&fetch_admin(deps)?),
        QueryMsg::FetchWhitelisted { chain_id } => {
            to_json_binary(&fetch_white_listed(deps, &chain_id)?)
        }
        QueryMsg::FetchAllWhiteListed { start_key, limit } => to_json_binary(
            &fetch_white_listed_cosmos_chains_info(deps, start_key, limit)?,
        ),
        QueryMsg::FetchInTransitCalls { start_key, limit } => {
            to_json_binary(&fetch_intransit_calls(deps, start_key, limit))
        }
        QueryMsg::FetchTempItem {} => to_json_binary(&fetch_incoming_ibc_state(deps)?),
        QueryMsg::FetchBalances { addr } => to_json_binary(&fetch_balance(deps, addr)?),
    }
}
