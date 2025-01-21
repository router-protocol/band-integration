use band_integration_package::oracle_manager::{
    IbcChannelInfo, InTransitToIbcCall, TempIncomingCalls, TempOutgoingCalls,
};
use cw_storage_plus::{Item, Map};
use cosmwasm_std::{Uint128, Event};

// public constants
pub const CREATE_IBC_TANSFER: u64 = 1;
pub const HANDLE_INBOUND_IBC_TOKENS: u64 = 2;
pub const CREATE_OUTBOUND_REQUEST: u64 = 3;

pub const ADMIN: Item<String> = Item::new("admin");

// whitelisted Addresses mapping ChainId => Address
pub const WHITELISTED_IBC_CHANNELS: Map<&str, IbcChannelInfo> =
    Map::new("white_listed_ibc_channels");

pub const TEMP_INCOMING_IBC_CALL: Item<TempIncomingCalls> = Item::new("temp_incoming_ibc_call");

pub const TEMP_OUTGOING_IBC_CALL: Item<TempOutgoingCalls> = Item::new("temp_outgoing_ibc_call");

pub const IN_TRANSIT_IBC_CALLS: Map<(&str, u64), InTransitToIbcCall> =
    Map::new("in_transit_ibc_calls");

pub const FEE_PAYER: Map<u64, String> = Map::new("fee_payer");
pub const CURRENT_FEE_PAYER: Item<String> = Item::new("current_fee_payer");

pub const FEE_TANK: Map<&str, Uint128> = Map::new("fee_tank");

// Temp Event to update it across multiple functions in a single execution
pub const EVENT: Item<Event> = Item::new("temp_event_state");

pub const REQUEST_ID_FEE_PAYER: Map<u64, String> = Map::new("request_id_message_hash");