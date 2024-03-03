/// Contains dashboard guides
pub mod dashboard;

/// Contains navigation guides
pub mod navigation;

/// Contains searching guides
pub mod searching;

/// Contain Tool guides
pub mod tool;

use leptos::html::Iframe;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
/// This structure represents the guides shown
pub struct Guide {
    name: &'static str,
    tango: View,
    id: &'static str
}

#[component]
/// This component abstracts all of the iframe provided by tango
pub fn TangoFrame(src: &'static str, title: &'static str) -> impl IntoView {
    view! {
        <iframe
            src=src
            sandbox="allow-scripts allow-top-navigation-by-user-activation allow-popups allow-same-origin"
            security="restricted"
            title=title
            width="100%"
            height="500px"
            referrerpolicy="strict-origin-when-cross-origin"
            frameborder="0"
            webkitallowfullscreen="webkitallowfullscreen"
            mozallowfullscreen="mozallowfullscreen"
            allowfullscreen="allowfullscreen"
        ></iframe>
    }
}

#[component]
/// Contains each created card.
pub fn GuideCard(guides: Vec<Guide>) -> impl IntoView {
    let tutorials = guides
        .into_iter()
        .map(|info| {
            view! {
                <GuideTool data=Guide {
                    name: info.name,
                    tango: info.tango,
                    id: info.id,
                }/>
            }
        })
        .collect_view();

    view! {
        <main class="container-fluid py-5 px-3 bd-navbar ">
            <div class="container-xxl bd-gutter flex-wrap flex-lg-nowrap">
                <span>{tutorials}</span>
            </div>
        </main>
    }
}

#[component]
/// This component adds a new tutorial.
pub fn GuideTool(data: Guide) -> impl IntoView {
    view! {
        <div class="accordion py-4">
            <div class="accordion-item">
                <h2 class="accordion-header">
                    <button
                        class="accordion-button"
                        type="button"
                        data-bs-toggle="collapse"
                        data-bs-target=format!("#collapse{}", data.id)
                        aria-expanded="false"
                        aria-controls=format!("collapse{}", data.id)
                    >
                        {data.name}
                    </button>
                </h2>
                <div
                    id=format!("collapse{}", data.id)
                    class="accordion-collapse collapse "
                    data-bs-parent="#accordionTutorialMain"
                >
                    <div class="accordion-body">{data.tango}</div>
                </div>
            </div>
        </div>
    }
}
