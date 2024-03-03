use leptos::html::Iframe;
use leptos::*;

use super::Guide;
use super::TangoFrame;

pub fn guides() -> Vec<Guide> {
    vec![Guide {
        name: "some title..",
        tango: view! {<TangoFrame src="" title="Soon.." />},
        id: "S1",
    }]
}
