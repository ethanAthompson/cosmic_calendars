/// Contains kinds of tutorials
pub mod kinds;

/// Contains tutorial routes
pub mod routes;

/// Contains invalid page
pub mod overview;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
/// This component is the tutorial page that you see
pub fn Page() -> impl IntoView {
    view! {
        <main class="container-fluid py-5 bd-navbar">
            <div class="container-xxl bd-gutter flex-wrap flex-lg-nowrap">
                <div class="container-fluid py-4">
                    <p class="display-6 text-center">Welcome to Cosmic Calendar Tutorials</p>
                </div>
                <Outlet/>
                <div
                    class="vstack row btn-group gap-3 px-4"
                    role="group"
                    aria-label="Dashboard Tutorials"
                >
                    <A href="dashboard-tutorials" class="py-5 fs-2 col btn btn-primary rounded">
                        Dashboard Tutorials
                    </A>
                    <A
                        href="page-navigation-tutorials"
                        class="py-5 fs-2 col btn btn-primary rounded"
                    >
                        Navigation Tutorials
                    </A>
                    <A href="searching-tutorials" class="py-5 fs-2 col btn btn-primary rounded">
                        Searching Tutorials
                    </A>
                    <A href="tool-tutorials" class="py-5 fs-2 col btn btn-primary rounded">
                        Tool Tutorials
                    </A>
                </div>
            </div>
        </main>
    }
}
