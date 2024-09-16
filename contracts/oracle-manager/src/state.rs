use band_integration_package::oracle_manager::{
    IbcChannelInfo, InTransitToIbcCall, TempIncomingCalls, TempOutgoingCalls,
};
use cw_storage_plus::{Item, Map};

// public constants
pub const CREATE_IBC_TANSFER: u64 = 1;
pub const HANDLE_INBOUND_IBC_TOKENS: u64 = 2;

pub const ADMIN: Item<String> = Item::new("admin");

pub const EXECUTORS: Map<&str, bool> = Map::new("executors");

// whitelisted Addresses mapping ChainId => Address
pub const WHITELISTED_IBC_CHANNELS: Map<&str, IbcChannelInfo> =
    Map::new("white_listed_ibc_channels");

pub const TEMP_INCOMING_IBC_CALL: Item<TempIncomingCalls> = Item::new("temp_incoming_ibc_call");

pub const TEMP_OUTGOING_IBC_CALL: Item<TempOutgoingCalls> = Item::new("temp_outgoing_ibc_call");

pub const IN_TRANSIT_IBC_CALLS: Map<(&str, u64), InTransitToIbcCall> =
    Map::new("in_transit_ibc_calls");
