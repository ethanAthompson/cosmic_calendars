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
use crate::stores::get_state;

#[component]
/// This component allows a more strict filtering by feedback types
pub fn FeedbackTypeDropDown() -> impl IntoView {
    let state = get_state::<String>("feedback-type-filter");

    let update_state = move |ev: MouseEvent| {
        state.1.set(event_target_value(&ev));
    };

    create_effect(move |_| {
        if state.0.get() == "Comments".to_string() {
            // logging::log!("ok");
        }
    });

    view! {
        <form action="" method="GET" class="bg-transparent dropup text-end">
            <button
                class="btn btn-secondary dropdown-toggle my-4"
                type="button"
                data-bs-toggle="dropdown"
                aria-expanded="false"
            >
                Filter Feedback Types
            </button>
            <ul class="bg-transparent dropdown-menu">
                <li>
                    <button
                        class="bg-transparent dropdown-item"
                        value="Comments"
                        on:click=update_state
                    >
                        Only Comments
                    </button>
                </li>
                <li>
                    <button
                        class="bg-transparent dropdown-item"
                        value="Bug Reports"
                        on:click=update_state
                    >
                        Only Bug Reports
                    </button>
                </li>
                <li>
                    <button
                        class="bg-transparent dropdown-item"
                        value="Feature Request"
                        on:click=update_state
                    >
                        Only Feature Request
                    </button>
                </li>
                <li>
                    <button
                        class="bg-transparent dropdown-item"
                        value="Questions"
                        on:click=update_state
                    >
                        Only Questions
                    </button>
                </li>
            </ul>
        </form>
    }
}

#[component]
/// This component shows when all the checkboxes are checked=false
pub fn YouIgnoredAFilter() -> impl IntoView {
    view! {
        <div class="alert alert-danger py-4 text-body">
            <i class="lead bi bi-emoji-frown mx-2"></i>
            <span class="lead">"Nothing is showing.. filters aren't applied."</span>
        </div>
    }
}

#[component]
/// Only allows a specific box to be open
/// This was the only solution at the moment to achieve featured results..
pub fn FeedbackFilters() -> impl IntoView {
    let state = get_state::<FeedbackSearchFilterStates>("feedback-filter");
    let resovled_signal = create_rw_signal(state.0.get().resolved);
    let unresolved_signal = create_rw_signal(state.0.get().un_resolved);
    let ignored_signal = create_rw_signal(state.0.get().ignored);
    let is_resolved_disabled = create_rw_signal(state.0.get().disabled_resolved);
    let is_un_resolved_disabled = create_rw_signal(state.0.get().disabled_un_resolved);
    let is_ignored_disabled = create_rw_signal(state.0.get().disabled_ignored);
    let feedback_state = get_state::<String>("feedback-type-filter");

    // updates the structure inside local storage (3x)
    let update_state = move || {
        state.1.set(FeedbackSearchFilterStates {
            resolved: resovled_signal.get(),
            un_resolved: unresolved_signal.get(),
            ignored: ignored_signal.get(),
            disabled_ignored: is_ignored_disabled.get(),
            disabled_resolved: is_resolved_disabled.get(),
            disabled_un_resolved: is_un_resolved_disabled.get(),
        });
    };
    // even if you delete the key, it'll always stay
    if feedback_state.0.get_untracked().is_empty() {
        // logging::log!("{:?}", feedback_state.0.get());

        feedback_state.1.set("Comments".to_string());
    }


    // this effect just coordinates the filter boxes (err handl)
    create_effect(move |_| {
        if (state.0.get().ignored == true
            && state.0.get().resolved == true
            && state.0.get().un_resolved == true)
        {
            resovled_signal.set(false);
            unresolved_signal.set(false);
            ignored_signal.set(true);
            update_state();
        };
    });

    let resolved_event = move |ev| {
        resovled_signal.set(event_target_checked(&ev));
        // logging::log!("Resolved: {:?}", resovled_signal.get());
        ignored_signal.set(false);
        unresolved_signal.set(false);

        if (event_target_checked(&ev) == true) {
            is_un_resolved_disabled.set(true);
            is_ignored_disabled.set(true);
        } else {
            is_un_resolved_disabled.set(false);
            is_ignored_disabled.set(false);
        }
        update_state();
    };
    let unresolved_event = move |ev| {
        unresolved_signal.set(event_target_checked(&ev));
        // logging::log!("Un-Resolved: {:?}", unresolved_signal.get());
        ignored_signal.set(false);
        resovled_signal.set(false);

        if (event_target_checked(&ev) == true) {
            is_resolved_disabled.set(true);
            is_ignored_disabled.set(true);
        } else {
            is_resolved_disabled.set(false);
            is_ignored_disabled.set(false);
        }
        update_state();
    };
    let ignored_event = move |ev| {
        ignored_signal.set(event_target_checked(&ev));
        // logging::log!("Ignored: {:?}", ignored_signal.get());
        // cancels the other filters
        unresolved_signal.set(false);
        resovled_signal.set(false);

        if (event_target_checked(&ev) == true) {
            is_un_resolved_disabled.set(true);
            is_resolved_disabled.set(true);
        } else {
            is_un_resolved_disabled.set(false);
            is_resolved_disabled.set(false);
        }

        update_state();
    };

    view! {
        <div>
            <div class="bg-transparent form-check form-check-reverse">
                <input
                    class="bg-transparent form-check-input"
                    type="checkbox"
                    id="FeedbackSearchFilter1"
                    prop:checked=resovled_signal
                    on:change=resolved_event
                    prop:disabled=is_resolved_disabled
                />

                <label class="bg-transparent form-check-label" for="FeedbackSearchFilter1">
                    See Resolved
                    {feedback_state.0.get()}
                    (s)
                </label>
            </div>
            <div class="bg-transparent form-check form-check-reverse">
                <input
                    class="bg-transparent form-check-input"
                    type="checkbox"
                    id="FeedbackSearchFilter2"
                    prop:checked=unresolved_signal
                    on:change=unresolved_event
                    prop:disabled=is_un_resolved_disabled
                />

                <label class="bg-transparent form-check-label" for="FeedbackSearchFilter2">
                    See Un-Resolved
                    {feedback_state.0.get()}
                    (s)
                </label>
            </div>
            <div class="bg-transparent form-check form-check-reverse">
                <input
                    class="bg-transparent form-check-input"
                    type="checkbox"
                    id="FeedbackSearchFilter3"
                    prop:checked=ignored_signal
                    on:change=ignored_event
                    prop:disabled=is_ignored_disabled
                />

                <label class="bg-transparent form-check-label" for="FeedbackSearchFilter3">
                    Ignore Filters
                </label>
            </div>
            <div class="py-2">
                <MultiSearcher/>
            </div>
        </div>
    }
}

#[component]
/// This component contains multiple searches (if possible)
pub fn MultiSearcher() -> impl IntoView {
    let state = get_state::<FeedbackSearchFilterStates>("feedback-filter");
    let state_ref = create_rw_signal(state.0.get().clone());
    let username = create_rw_signal("".to_string());
    let search_state = get_state::<FeedbackSearchStates>("feedback-search-filter");
    let search_state_ref = create_rw_signal(search_state.0.get().clone());

    create_effect(move |_| {
        username.set(search_state.0.get().value);
    });

    view! {
        <Show
            when=move || {
                (state.0.get().ignored == false && state.0.get().resolved == false
                    && state.0.get().un_resolved == false)
            }

            fallback=move || {
                view! {
                    <form
                        action=""
                        class="bg-transparent d-flex gap-3"
                        id="InputFeedbackSearchFilter"
                        name="InputFeedbackSearchFilter"
                    >
                        <div class="bg-transparent container-fluid">
                            <label
                                for="InputFeedbackSearchFilter"
                                class="bg-transparent form-label"
                            >
                                Search by User
                            </label>
                            <div class="bg-transparent input-group mb-3">
                                <span type="submit" class="input-group-text btn btn-success">
                                    Confirm ?
                                </span>
                                <input
                                    type="text"
                                    id="FeedbackSearchFilter"
                                    class="bg-transparent form-control border-secondary me-auto"
                                    aria-describedby="FeedbackSearchInputBlock"
                                    prop:value=username
                                    on:input=move |ev| {
                                        username.set(event_target_value(&ev));
                                        search_state
                                            .1
                                            .set(FeedbackSearchStates {
                                                value: username.get(),
                                            });
                                    }
                                />

                            </div>
                            <div id="FeedbackSearchInputBlock" class="bg-transparent form-text">
                                <Show
                                    when=move || !username.get().is_empty()
                                    fallback=move || {
                                        view! { <span>Users may be duplicated..</span> }
                                    }
                                >

                                    <span>{username} may be duplicated..</span>
                                </Show>
                            </div>
                        </div>
                    </form>
                }
            }
        >

            <span></span>
        </Show>
    }
}