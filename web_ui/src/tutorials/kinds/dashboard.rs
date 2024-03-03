use leptos::html::Iframe;
use leptos::*;

use super::Guide;
use super::TangoFrame;

// could I just return a constant Vec<Guide>? instead of a function

pub fn guides() -> Vec<Guide> {
    vec![Guide {
        name: "some title..",
        tango: view! {<TangoFrame src="" title="Soon.." />},
        id: "DD1",
    },
    Guide {
        name: "some title 2..",
        tango: view! {<TangoFrame src="" title="Soon.." />},
        id: "DD2",
    }]
}
