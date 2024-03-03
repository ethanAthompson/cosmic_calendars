/// This module contains the final question code.
pub mod last;
/// This module contains the first question code.
pub mod zero;
/// This module contains the second question code.
pub mod one;
/// This module contains the third question code.
pub mod two;
// This module contains the fourth question code.
pub mod third;


use leptos::*;
use leptos_meta::*;

use self::zero::Question as QuestionZero;
use self::one::Question as QuestionOne;
use self::two::Question as QuestionTwo;
use self::third::Question as QuestionThree;
use self::last::Question as QuestionFinal;


#[component]
/// This component is the collection of Question Modals.
pub fn SetupConfig() -> impl IntoView {
    view! {
        <div class="mx-2 container-lg h-100 text-center align-items-center justify-content-end">
            <QuestionZero/>
            <QuestionOne/>
            <QuestionTwo/>
            <QuestionThree/>
            <QuestionFinal/>
        </div>
    }
}


#[component]
/// This component takes in a child (a way to replicate modal code).
pub fn Abstract() -> impl IntoView {
    view! { <div></div> }
}