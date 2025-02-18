use band_integration_package::oracle_manager::{
    ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, SudoMsg,
};
use router_wasm_bindings::{RouterMsg, RouterQuery};

#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::{to_json_binary, Reply, StdError};
use cw2::{get_contract_version, set_contract_version};

use crate::{
    execution::{
        receive_band_data, update_admin, register_fee_payer_or_fund, claim_admin,
    },
    handle_reply::handle_reply,
    queries::{
        fetch_admin, fetch_fee_payer_for_tunnel_id, fetch_available_funds,
    },
    state::ADMIN,
    sudo::handle_sudo_ack,
};

// version info for migration info
const CONTRACT_NAME: &str = "BandProtocol::OracleManager";
const CONTRACT_VERSION: &str = "0.1.2";

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
    match msg {
        SudoMsg::HandleIAck {
            request_identifier,
            exec_flag,
            exec_data,
            refund_amount,
        } => handle_sudo_ack(deps, env, request_identifier, exec_flag, exec_data, refund_amount),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<RouterQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<RouterMsg>> {
    match msg {
        ExecuteMsg::ReceiveBandData {
            dest_chain_id,
            dest_contract_address,
            max_gas_limit,
            payload,
        } => receive_band_data(
            deps,
            &env,
            &info,
            dest_chain_id,
            dest_contract_address,
            max_gas_limit,
            payload,
        ),
        ExecuteMsg::UpdateAdmin { new_admin } => update_admin(deps, &info, new_admin),
        ExecuteMsg::ClaimAdmin{ } => claim_admin(deps, &env, &info),
        ExecuteMsg::RegisterFeePayerOrFund { band_fee_payer } => register_fee_payer_or_fund(deps, &info, band_fee_payer),
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
        QueryMsg::FetchFeePayer { band_fee_payer } => to_json_binary(&fetch_fee_payer_for_tunnel_id(deps, band_fee_payer)?),
        QueryMsg::FetchAvailableFunds { fee_payer } => to_json_binary(&fetch_available_funds(deps, fee_payer)?),
    }
}
