/// Contains the code for `Get started today!`
pub mod launch;

/// Contains the code to allow the user to signup..
pub mod user;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::theme::animated::Background;
use self::launch::SetupConfig;

/*

Make a Way to have the user enter their name..
*/

#[component]
/// This page represents the loading screen when [new_user#... = false] in the local_storage.
///  
/// I feel that the background image should more like a space hub or something you'll see on a loading screen
/// that has a nice infinite loop. that makes you feel like you just walked into something new.
pub fn Page() -> impl IntoView {

    view! {
        <div class="position-absolute top-50 start-50 translate-middle text-light">
            // Title
            <div class="bg-transparent p-4 rounded-5">
                <h1 class="display-5 text-center">Cosmic Calendars</h1>
                <figure class="text-center py-2 ">
                    <blockquote class="blockquote">
                        <p class="fst-italic lead w-100">
                            "Navigating Time Zones and Communication Delays Across the Solar System."
                        </p>
                    </blockquote>
                </figure>
            </div>
            // Launcher
            <div class="d-flex justify-content-center">
                <button
                    class="btn btn-primary  focus-ring focus-ring-info text-body floater d-flex align-items-center justify-content-center"
                    data-bs-target="#ModalToggleQ0"
                    data-bs-toggle="modal"
                >
                    // <img
                    // class="spinner img-fluid w-25"
                    // src="/public/favicon_io/apple-touch-icon.png"
                    // />
                    <span class="display-5 mx-2">Launch!</span>
                </button>

            </div>
        </div>
        <SetupConfig/>
    }
}

   // <div
        //     class="card bg-opacity-25 border-0 rounded-0 start-0 "
        //     style="max-width: 25rem; min-height:100vh;"
        // >
        //     <div class="card-body text-body position-relative">
        //         <div class="container-fluid d-flex align-items-center justify-content-center">
        //             <img
        //                 class="py-2 img-fluid w-25 h-25"
        //                 src="public/images/interactive/rustacean-flat-happy.png"
        //             />
        //         </div>
        //         <h5 class="card-title text-center">Greetings New User</h5>
        //         <p class="card-text lead">
        //             "This website is about Navigating Time Zones and Communication Delays Across the Solar System."
        //         </p>
        //         <p class="py-2 text-center display-6">"Get started today!"</p>
        //         <SetupConfig/>
        //     </div>
        //     <div class="card-footer border-0  text-body bg-transparent border-secondary lead">
        //         Cosmic Calendars by <span class="fst-italic">Ethan Thompson</span>
        //     </div>
        // </div>