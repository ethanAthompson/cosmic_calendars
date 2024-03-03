use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::ThemeProvider;

use crate::navbar::Page as MainNavbar;
use crate::stores::states::CosmicTimeColumn;
use crate::theme::image::Background as ImageBackground;
use crate::theme::animated::Background as AnimatedBackground;
use crate::theme::ThemeProvider as BootstrapThemeProvider;
use crate::home::Page as HomePage;
use crate::feedback::Page as FeedbackPage;
use crate::loading::Page as LoadingPage;
use crate::user::Page as VerifiedUserPage;
use crate::error::Page as ErrorPage;
use crate::data::routes::DataRoutes;
use crate::tutorials::routes::TutorialRoutes;
use crate::dashboard::routes::DashboardRoutes;
use crate::stores::get_state;

use crate::context::NavbarSearchString;
use crate::context::NewUser;

/*
Maybe all the different timezones can have the same name?
*/
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let is_new_user = get_state::<bool>("new-user31a85bde66b7d713c8743a12d427f3a5");
    
    let verification = create_rw_signal(NewUser(is_new_user.0.get()));

    let cosmic_search: RwSignal<NavbarSearchString> = create_rw_signal(NavbarSearchString("".to_string()));

    provide_context(verification);
    provide_context(cosmic_search);

    create_effect(move |_| {
        verification.set(NewUser(is_new_user.0.get()));
    });


    view! {
        <BootstrapThemeProvider>
            <ThemeProvider use_data_attribute=true>
                <Router>
                    <MainNavbar/>
                    <ImageBackground/>
                    <AnimatedBackground/>
                    <Routes>
                        <Route path="/" view=VerifiedUserPage>
                            <Route path="/" view=HomePage/>
                            <Route path="/feedback" view=FeedbackPage/>
                            <DataRoutes/>
                            <TutorialRoutes/>
                            <DashboardRoutes/>
                        </Route>
                        <Route path="/:else" view=ErrorPage/>
                    </Routes>
                </Router>
            </ThemeProvider>
        </BootstrapThemeProvider>
    }
}

