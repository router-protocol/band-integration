use std::collections::BTreeMap;

use cosmwasm_std::{Env, StdError, StdResult};
use serde_cw_value::Value;

const IBC_CALLBACK: &str = "ibc_callback";

pub fn insert_callback_key(memo: Value, env: &Env) -> Value {
    let serde_cw_value::Value::Map(mut m) = memo else {
        unreachable!()
    };
    m.insert(
        serde_cw_value::Value::String(IBC_CALLBACK.to_owned()),
        serde_cw_value::Value::String(env.contract.address.to_string()),
    );
    serde_cw_value::Value::Map(m)
}

pub fn include_callback_inside_memo(env: &Env) -> StdResult<String> {
    let temp = Value::Map(BTreeMap::new());

    let memo_val = insert_callback_key(temp, env);
    let memo = serde_json_wasm::to_string(&memo_val)
        .map_err(|_e| StdError::generic_err("InvalidMemo {}"))?;

    return Ok(memo);
}
