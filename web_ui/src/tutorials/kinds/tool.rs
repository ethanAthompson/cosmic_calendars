use leptos::html::Iframe;
use leptos::*;

use super::Guide;
use super::TangoFrame;

/// Example
/// tango: view! {<TangoFrame src="https://app.tango.us/app/embed/79fe60bf-f9b8-4809-a65b-ac1b6704e571?skipCover=false&defaultListView=false&skipBranding=true" title="Navigating the Mars West Zone Vertical radio toggle button group to set home timezone." />},
///
pub fn guides() -> Vec<Guide> {
    vec![Guide {
        name: "some title..",
        tango: view! {<TangoFrame src="" title="" />},
        id: "NVT1",
    }]
}
