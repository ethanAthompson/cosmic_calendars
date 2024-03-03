use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use super::Page;
use super::overview::Page as Overview;
use super::kinds::GuideCard;

/* guides for each kind of tutorial */
use super::kinds::dashboard::guides as dashboard_guides;
use super::kinds::navigation::guides as navigation_guides;
use super::kinds::searching::guides as searching_guides;
use super::kinds::tool::guides as tool_guides;

#[component(transparent)]
/// This transparent component represented the aggregated tutorial cards for each route.
pub fn TutorialRoutes() -> impl IntoView {
    view! {
        <Route path="/tutorials" view=move || Page>
            <Route
                path="dashboard-tutorials"
                view=move || view! { <GuideCard guides=dashboard_guides()/> }
            />
            <Route
                path="page-navigation-tutorials"
                view=move || view! { <GuideCard guides=navigation_guides()/> }
            />
            <Route
                path="searching-tutorials"
                view=move || view! { <GuideCard guides=searching_guides()/> }
            />
            <Route path="tool-tutorials" view=move || view! { <GuideCard guides=tool_guides()/> }/>
            <Route path="" view=Overview/>
        </Route>
    }
}


