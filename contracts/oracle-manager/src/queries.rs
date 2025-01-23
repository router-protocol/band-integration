use cosmwasm_std::{Deps, StdResult, Uint128};
use router_wasm_bindings::RouterQuery;

use crate::state::{
    ADMIN, FEE_PAYER, FEE_TANK,
};

pub fn fetch_admin(deps: Deps<RouterQuery>) -> StdResult<String> {
    ADMIN.load(deps.storage)
}

pub fn fetch_fee_payer_for_tunnel_id(
    deps: Deps<RouterQuery>,
    tunnel_id: u64,
) -> StdResult<String> {
    FEE_PAYER.load(deps.storage, tunnel_id)
}

pub fn fetch_available_funds(
    deps: Deps<RouterQuery>,
    fee_payer: String,
) -> StdResult<Uint128> {
    FEE_TANK.load(deps.storage, &fee_payer)
}