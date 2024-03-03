use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::context::NavbarSearchString;
use crate::stores::states::SearchedCelestialBodyData;
use crate::utils::colors::colored_class;

// the search bar would just route you to the #some-link-width-maybe
// if I wanted to search a tutorial then I would type
// "navigating ...", then it would direct me to /tutorials/page-navigation-tutorials#navigating-the-home-page
//
// There are 2 crucial parts
//
//  1. <a href="#width">see width</a>
//
//  2. <h2 id="width">Width</h2>
//
// (Theory)
//
// The searchbar contains the links that just send you to the matching id on another route
//
// in order to initialize a toast or tooltip, you can use wasmb bindgen to call the code

#[component]
/// This component represents a certain item within the search (its' CelestialItem because the name of the project starts with celestial )
pub fn CelestialItem(data: SearchedCelestialBodyData) -> impl IntoView {
    view! {
        <span id=format!("{}", data.name.clone())>
            <A
                href=format!("/{}", data.link)
                class="bg-transparent text-body border border-3 list-group-item focus-ring-success focus-ring list-group-item-action active"
            >
                <div class="hstack gap-3">
                    <h5 class="me-auto">{data.name.clone()}</h5>
                    <span class="ms-auto badge rounded-pill text-bg-danger">
                        {data.number_of_timezones.clone()} timezones
                    </span>
                </div>
                <p class="lead">{data.description.clone()}</p>
                <div class="hstack gap-3">
                    <small class=colored_class(data.class.clone())>{data.class.clone()}</small>
                    <small class="ms-auto text-end">Discovered {data.calendar_date.clone()}</small>
                </div>
            </A>
        </span>
    }
}

