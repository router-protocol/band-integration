use cw_storage_plus::{Item, Map};
use cosmwasm_std::{Uint128, Event};

// public constants
pub const CREATE_OUTBOUND_REQUEST: u64 = 1;

pub const ADMIN: Item<String> = Item::new("admin");
pub const NEW_ADMIN: Item<String> = Item::new("new_admin");

pub const ROUTER_FEE_PAYER: Map<&str, String> = Map::new("fee_payer");
pub const CURRENT_FEE_PAYER: Item<String> = Item::new("current_fee_payer");

pub const FEE_TANK: Map<&str, Uint128> = Map::new("fee_tank");

// Temp Event to update it across multiple functions in a single execution
pub const EVENT: Item<Event> = Item::new("temp_event_state");

pub const REQUEST_ID_FEE_PAYER: Map<u64, String> = Map::new("request_id_message_hash");