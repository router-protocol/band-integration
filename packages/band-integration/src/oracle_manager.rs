use crate::{Deserialize, Serialize};
use cosmwasm_std::{Binary, Uint128};
use ibc_tracking::msg::IBCLifecycleComplete;
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Completed,
    Failed,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WhitelistCosmosChain {
    pub chain_id: String,
    pub incoming_port: String,
    pub incoming_channel: String,
    pub outgoing_port: String,
    pub outgoing_channel: String,
    pub timeout_height: u64,
    pub timeout_timestamp: u64,
    pub remove: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TempIncomingCalls {
    pub src_chain_id: String,
    pub local_fallback_address: String,
    pub token_addr: String,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TempOutgoingCalls {
    pub source_channel: String,
    pub src_chain_id: String,
    pub dest_chain_id: String,
    pub sender: String,
    pub recipient: String,
    pub token_addr: String,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InTransitToIbcCall {
    pub source_channel: String,
    pub src_chain_id: String,
    pub dest_chain_id: String,
    pub sender: String,
    pub recipient: String,
    pub token_addr: String,
    pub amount: Uint128,
    pub timeout_timestamp: u64,
    pub status: Option<Status>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UsdcBurnableInfo {
    pub chain_id: String,
    pub burnable: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IbcChannelInfo {
    pub incoming_port: String,
    pub incoming_channel: String,
    pub outgoing_port: String,
    pub outgoing_channel: String,
    pub timeout_height: u64,
    pub timeout_timestamp: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    #[serde(rename = "ibc_lifecycle_complete")]
    IBCLifecycleComplete(IBCLifecycleComplete),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ReceiveBandData {
        dest_chain_id: String,
        dest_contract_address: String,
        gas_limit: u64,
        gas_price: u64,
        payload: Binary,
        nonce: u64,
        signature: String,
    },
    WhitelistCosmosChain {
        ibc_info: Vec<WhitelistCosmosChain>,
    },
    UpdateAdmin {
        new_admin: String,
    },
    WithdrawFunds {
        denom: String,
        recipient: String,
        amount: Uint128,
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
    FetchWhitelisted {
        chain_id: String,
    },
    FetchAllWhiteListed {
        start_key: Option<String>,
        limit: Option<u64>,
    },
    FetchInTransitCalls {
        start_key: Option<(String, u64)>,
        limit: Option<u64>,
    },
    FetchTempItem {},
    FetchBalances {
        addr: String,
    },
}
