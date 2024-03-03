use leptos::*;

#[component]
/// This component is a wrapper over the bootstrap progress bar
pub fn PageProgess(#[prop(into)] percent: String, #[prop(into)] purpose: String) -> impl IntoView {
    view! {
        <div
            class="progress border-0 rounded-bottom-1 rounded-top-0"
            role="progressbar"
            aria-label=purpose.clone()
            aria-valuenow=percent.clone()
            aria-valuemin="0"
            aria-valuemax="100"
        >
            <div class=format!("progress-bar w-{}", percent.clone())></div>
        </div>
    }
}