use leptos::html::{time, Input};
use leptos::svg::filter;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use serde_json::to_string;
use serde_json::value::Value;
use serde_with::rust;
use web_sys::{Event, MouseEvent, Node, SubmitEvent};

use crate::stores::states::{FeedbackSearchFilterStates, FeedbackSearchStates};
use crate::utils::sheets::get_data;
use crate::utils::str::rustify_str;
use crate::utils::time::TimeStamp;
use crate::env::FEEDBACK_URL;
use crate::stores::{get_state, sheets::*};
use crate::utils::colors::direct_feedback_types;
use crate::utils::sheets::has_resolved;
use crate::utils::images::direct_profile_picture;

#[component]
/// This component is the list of feedback that you see
pub fn FeedbackList() -> impl IntoView {
    view! {
        <div class="bg-transparent container-xxl ">
            <div class="bg-transparent container-fluid py-5">
                <DisplayList url=FEEDBACK_URL/>
            </div>
        </div>
    }
}

#[component]
/// Displays the feedback to page
fn DisplayList(url: &'static str) -> impl IntoView {
    // // our source signal: some synchronous, local state
    let (data, set_data) = create_signal("");
    let async_data = get_data(data, set_data, url);

    let app = {
        move || match async_data.get() {
            None => view! {
                <div class="bg-transparent spinner-border" role="status">
                    <span class="bg-transparent visually-hidden ">Loading...</span>
                </div>
            }
            .into_view(),
            Some(data) => view! { <GoogleSheetsData data=data/> }.into_view(),
        }
    };

    view! {
        <div class="bg-transparent container-fluid">
            <div class="bg-transparent row row-cols-auto gap-3">{app}</div>
        </div>
    }
}

#[component]
/// Displays the google sheets data
pub fn GoogleSheetsData(#[prop(into)] data: SheetData) -> impl IntoView {
    let search_state = get_state::<FeedbackSearchStates>("feedback-search-filter");
    let feedback_state = get_state::<String>("feedback-type-filter");
    let state = get_state::<FeedbackSearchFilterStates>("feedback-filter");
    let reactive_data: RwSignal<_> = create_rw_signal(data.clone());

    view! {
        <Show
            // if you see any weird bugs, then this may be the issue.
            when=move || {
                (feedback_state.0.get() == "Comments" || feedback_state.0.get() == "Questions"
                    || feedback_state.0.get() == "Bug Reports"
                    || feedback_state.0.get() == "Feature Request")
                    && (state.0.get().ignored == true || state.0.get().resolved == true
                        || state.0.get().un_resolved == true)
            }

            fallback=move || view! { <span></span> }
        >
            <For
                // a function that returns the items we're iterating over; a signal is fine
                each=move || reactive_data.get().table.rows.clone()
                // a unique key for each item
                key=|username| username.c[1].v.clone().unwrap().to_string().clone()
                // renders each item to a view
                children=move |data: Row| {
                    view! {
                        <div hidden=has_resolved(
                            data.clone(),
                            state.0.get(),
                            search_state.0.get(),
                            feedback_state.0.get(),
                        )>
                            <div
                                class="bg-transparent card mb-3"
                                style="max-width: 540px;"
                                id=format!(
                                    "feedback-status={}",
                                    rustify_str(data.c[10].v.clone().unwrap().to_string()),
                                )
                            >

                                <div class="bg-transparent row g-0">
                                    <div class="bg-transparent col-md-4">
                                        <img
                                            src=direct_profile_picture(
                                                rustify_str(
                                                    data.c[9].v.clone().unwrap().to_string().clone(),
                                                ),
                                            )

                                            class="bg-transparent rounded-circle img-fluid p-2"
                                            alt=rustify_str(
                                                data.c[9].v.clone().unwrap().to_string().clone(),
                                            )
                                        />

                                    </div>
                                    <div class="bg-transparent col-md-8">
                                        <div class="bg-transparent card-body">
                                            <div class="bg-transparent py-2 d-flex justify-content-between">
                                                <h5 class="bg-transparent card-title ">
                                                    {rustify_str(
                                                        data.c[1].v.clone().unwrap().to_string().clone(),
                                                    )}

                                                </h5>
                                                <span
                                                    class="bg-transparent user-select-none"
                                                    data-toggle="tooltip"
                                                    title="Feedback Type"
                                                >
                                                    <h5 class=direct_feedback_types(
                                                        rustify_str(
                                                            data.c[6].v.clone().unwrap().to_string().clone(),
                                                        ),
                                                    )>
                                                        {rustify_str(
                                                            data.c[6].v.clone().unwrap().to_string().clone(),
                                                        )}

                                                    </h5>
                                                </span>

                                            </div>
                                            <p class="bg-transparent card-text">
                                                <ul class="bg-transparent list-group">
                                                    <li class="bg-transparent list-group-item">
                                                        <span class="bg-transparent border-bottom">Feedback</span>
                                                        <p class="bg-transparent fst-italic py-2">
                                                            {rustify_str(
                                                                data.c[7].v.clone().unwrap().to_string().clone(),
                                                            )}

                                                        </p>
                                                    </li>
                                                    <li class="bg-transparent list-group-item">
                                                        <span class="bg-transparent border-bottom">
                                                            Suggestions
                                                        </span>
                                                        <p class="bg-transparent fst-italic py-2">
                                                            {rustify_str(
                                                                data.c[8].v.clone().unwrap().to_string().clone(),
                                                            )}

                                                        </p>
                                                    </li>
                                                </ul>
                                            </p>
                                            <p class="bg-transparent card-text ">
                                                <small class="bg-transparent text-body-secondary">
                                                    <TimeStamp stamp=data.c[0].f.clone().unwrap()/>
                                                </small>
                                            </p>

                                            <p class="bg-transparent card-text">
                                                <ul class="bg-transparent list-group list-group-horizontal user-select-none">
                                                    <li class="bg-transparent list-group-item list-group-item-info">
                                                        Status
                                                    </li>
                                                    <Resolve comment=rustify_str(
                                                        data.c[10].v.clone().unwrap().to_string(),
                                                    )/>

                                                </ul>
                                            </p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                }
            />

        </Show>
    }
}

#[component]
/// This component takes in the comment that I set,
/// and marks the feedback card with the checkmark,
/// which notes that the suggestion has been added/implemented/resolved.
pub fn Resolve(#[prop(into)] comment: String) -> impl IntoView {
    // a singal to capture the environmnet of the comment to use with leptos <Show> component
    let react_comment = create_rw_signal(comment.clone());

    view! {
        <Show
            when=move || { react_comment.get() == "Resolved".to_string() }
            fallback=move || {
                view! {
                    <li class="bg-transparent list-group-item d-flex gap-2 align-items-center">
                        <span>{react_comment.get()}</span>
                    </li>
                }
            }
        >

            <li class="bg-transparent list-group-item d-flex gap-2 align-items-center">
                <span>{react_comment.get()}</span>
                <i class="bg-transparent bi bi-check-circle text-success"></i>
            </li>
        </Show>
    }
}
