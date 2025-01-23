use cosmwasm_std::{Deps, StdError, StdResult, MessageInfo, Uint128};
use router_wasm_bindings::{RouterQuery, types::NATIVE_DENOM};

use crate::queries::fetch_admin;

pub fn is_admin_modifier(deps: Deps<RouterQuery>, sender: &str) -> StdResult<()> {
    let admin: String = fetch_admin(deps)?; 
    if sender == &admin {
        return Ok(());
    }
    return StdResult::Err(StdError::GenericErr {
        msg: String::from("Auth: The caller is not Admin"),
    });
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

pub fn is_ibc(token: &String) -> bool {
    if token.starts_with("ibc/") {
        return true;
    }

    false
}

pub fn validate_funds(info: &MessageInfo) -> StdResult<(Uint128, String, Uint128)> {
    assert_eq!(info.funds.len() < 3, true, "Funds length should be 1 or 2");
    assert_eq!(info.funds.len() != 0, true, "Funds length should be 1 or 2");

    let mut native_amount: Uint128 = Uint128::zero();
    let mut ibc_amount: Uint128 = Uint128::zero();
    let mut ibc_token_address: String = String::default();

    if info.funds.len() == 1 {
        let fund: &cosmwasm_std::Coin = &info.funds[0];
        if fund.denom == NATIVE_DENOM {
            native_amount = fund.amount;
        } else {
            ibc_token_address = fund.denom.clone();
            ibc_amount = fund.amount.clone();
        }
    }
    if info.funds.len() == 2 {
        let fund0: &cosmwasm_std::Coin = &info.funds[0];
        let fund1: &cosmwasm_std::Coin = &info.funds[1];

        assert_eq!(fund0.denom, NATIVE_DENOM, "Native Coins are required");
        assert_eq!(is_ibc(&fund1.denom), true, "IBC is required");
        native_amount = fund0.amount;
        ibc_token_address = fund1.denom.clone();
        ibc_amount = fund1.amount.clone();
    }

    Ok((native_amount, ibc_token_address, ibc_amount))
}