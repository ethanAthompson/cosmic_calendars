use leptos::*;

#[component]
/// This component represents a tutorial that is not found in the url
pub fn Page() -> impl IntoView {
    view! {
        <main class="container-fluid py-4 bd-navbar ">
            <div class="container-xxl bd-gutter flex-wrap flex-lg-nowrap">
                <span class="fs-4 lead">Please choose one of the tutorials to start with..</span>
            </div>
        </main>
    }
}
