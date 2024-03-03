pub mod ratings;
pub mod filters;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use crate::stores::states::FeedbackSearchFilterStates;
use self::ratings::FeedbackList;
use self::filters::{FeedbackFilters, YouIgnoredAFilter, FeedbackTypeDropDown};
use crate::stores::get_state;

#[component]
/// This component represents the feedback page
pub fn Page() -> impl IntoView {
    let state = get_state::<FeedbackSearchFilterStates>("feedback-filter");


    view! {
        <div class="container py-5">
            <h1>Please complete a quick feedback form</h1>
            <p class="px-4">Your feedback is collected to help improve the website.</p>
            <div class="container-fluid">
                <iframe
                    class="rounded-3 bg-transparent"
                    src="https://docs.google.com/forms/d/e/1FAIpQLScH2EGNVDeXAf8VIF05VjxrwImy0uv47aYKD0g221t9A5smsQ/viewform?embedded=true"
                    sandbox="allow-scripts allow-top-navigation-by-user-activation allow-popups allow-same-origin allow-forms "
                    security="restricted"
                    title="A form that allows users to help improve the website by their feedback"
                    width="100%"
                    height="650px"
                    referrerpolicy="strict-origin-when-cross-origin"
                    frameborder="0"
                    marginheight="0"
                    marginwidth="0"
                    allowtransparency="true"
                    webkitallowfullscreen="webkitallowfullscreen"
                    mozallowfullscreen="mozallowfullscreen"
                    allowfullscreen="allowfullscreen"
                ></iframe>
            </div>
        </div>
        <div class="bg-transparent container-xl py-5 justify-content-center">
            <div class="bg-transparent accordion accordian-flush" id="main-users-feedback-accord">
                <div class="bg-transparent accordion-item">
                    <h2 class="bg-transparent accordion-header">
                        <button
                            class=" accordion-button"
                            type="button"
                            data-bs-toggle="collapse"
                            data-bs-target="#main-users-feedback-accord-collapseOne"
                            aria-expanded="false"
                            aria-controls="main-users-feedback-accord-collapseOne"
                        >
                            <span class=" fw-bold display-5">
                                <span class=" mx-2">"Users' Feedback"</span>
                                <i class=" mx-2 bi bi-emoji-smile"></i>
                            </span>
                        </button>
                    </h2>
                    <div
                        id="main-users-feedback-accord-collapseOne"
                        class="bg-transparent accordion-collapse collapse"
                    >

                        <div class="bg-transparent accordion-body">
                            <span class=" ">
                                <strong>
                                    Feedback helps fix the application, please fill out the
                                </strong>
                                <A href="feedback" class="text-decoration-none">
                                    <span>Feedback Form</span>
                                    <i class="mx-2 bi bi-box-arrow-in-left"></i>
                                </A>
                            </span>
                            <div class="bg-transparent py-0">
                                <FeedbackTypeDropDown/>
                                <FeedbackFilters/>

                                <Show
                                    when=move || {
                                        (state.0.get().ignored == false
                                            && state.0.get().resolved == false
                                            && state.0.get().un_resolved == false)
                                    }

                                    fallback=move || {
                                        view! { <span></span> }
                                    }
                                >

                                    <YouIgnoredAFilter/>

                                </Show>
                            </div>
                            <div class="bg-transparent px-2">
                                <FeedbackList/>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
