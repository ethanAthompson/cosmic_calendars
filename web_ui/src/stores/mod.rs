/// This module contains data structures for handling sheet data
pub mod sheets;

/// This module contains data structures for handling state data (Local Storage)
pub mod states;

use leptos_use::storage::StringCodec;
use leptos_use::storage::{use_local_storage, JsonCodec};
use leptos::WriteSignal;
use leptos::Signal;
use leptos::server_fn::serde::{Serialize, Deserialize};
use serde_json::value::Value;

/// This is a wrapper over the leptos_use [use_local_storage] signal
pub fn get_state<T>(item: &'static str) -> (Signal<T>, WriteSignal<T>, impl Fn() + Clone)
where
    T: Clone
        + Default
        + PartialEq
        + for<'de> leptos::server_fn::serde::Deserialize<'de>
        + leptos::server_fn::serde::Serialize,
{
    let (state, set_state, delete_state) = use_local_storage::<T, JsonCodec>(item);

    (state, set_state, delete_state)
}
