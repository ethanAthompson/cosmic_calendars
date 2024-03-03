use chrono::{Datelike, Duration, Timelike};
use chrono::{NaiveDateTime, NaiveTime, TimeDelta};
use leptos::*;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;

pub const UNIX_EPOCH: i64 = 1707870586;

/// This function converts a timestamp to a nicely formatted date.
#[component]
pub fn TimeStamp(stamp: String) -> impl IntoView {
    let d = chrono::NaiveDateTime::parse_from_str(&stamp, "%m/%d/%Y %H:%M:%S").unwrap();
    let stamp_signal = create_rw_signal(d.format("%B %e, %Y").to_string());

    view! { <span>Created {stamp_signal}</span> }
}
