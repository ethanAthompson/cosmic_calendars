use chrono::Local;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::core::Position;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;
use rust_solar::kepler::TimeZone;
use rust_solar::planets::mars::Mars;
use rust_solar::planets::mars::Martian;
use strum::EnumProperty;
use web_sys::Event;
use web_sys::MouseEvent;

use crate::stores::get_state;
use crate::stores::states::CosmicTimeZoneState;
use crate::env::{LEFTCARD, RIGHTCARD};
