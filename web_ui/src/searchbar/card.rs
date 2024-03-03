use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use super::data::DisplayList;
use crate::env::ASTRONOMICAL_DATA_URL;
use super::filter::Filter;

#[component]
/// This component represents the searchbar modal (as a page)
pub fn Page() -> impl IntoView {
    view! {
        <div
            class="modal fade"
            id="SearchModal"
            tabindex="-1"
            aria-labelledby="SearchModalLabel"
            aria-hidden="true"
        >
            <div class="modal-dialog modal-lg modal-dialog-centered modal-dialog-scrollable">
                <div class="modal-content">
                    <div class="modal-header hstack gap-3">
                        <Filter/>
                        <button
                            type="button"
                            class="btn-close"
                            data-bs-dismiss="modal"
                            aria-label="Close"
                        ></button>
                    </div>
                    <div
                        class="modal-body list-group overflow-auto py-1"
                        style="max-height: 500px;"
                    >
                        <DisplayList url=ASTRONOMICAL_DATA_URL/>
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">
                            Close
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}


