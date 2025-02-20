use crate::{Deserialize, Serialize};
use cosmwasm_std::{Binary, Coin};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    HandleIAck {
        request_identifier: u64,
        exec_flag: bool,
        exec_data: Binary,
        refund_amount: Coin,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ReceiveBandData {
        dest_chain_id: String,
        dest_contract_address: String,
        gas_limit: u64,
        payload: Binary,
    },
    UpdateAdmin {
        new_admin: String,
    },
    ClaimAdmin { },
    RegisterFeePayerOrFund {
        band_fee_payer: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // fetch contract version
    GetContractVersion {},
    FetchAdmin {},
    FetchFeePayer {
        band_fee_payer: String,
    },
    FetchAvailableFunds {
        fee_payer: String,
    },
}
