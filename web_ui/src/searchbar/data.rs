use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use super::item::CelestialItem;
use crate::context::NavbarSearchString;
use crate::stores::sheets::Row;
use crate::stores::sheets::SheetData;
use crate::stores::states::SearchedCelestialBodyData;
use crate::utils::sheets::get_data;
use crate::utils::str::rustify_str;
use crate::utils::sheets::correct_class;
use crate::utils::sheets::get_astronomy_row;

#[component]
/// This component abstracts the astronomical google sheets data in a specific format
pub fn DisplayList(url: &'static str) -> impl IntoView {
    let (data, set_data) = create_signal("");

    let async_data = get_data(data, set_data, url);

    let app = {
        move || match async_data.get() {
            None => view! {
                <div class="spinner-border" role="status">
                    <span class="visually-hidden">Loading...</span>
                </div>
            }
            .into_view(),
            Some(data) => view! { <GoogleSheetsData data=data/> }.into_view(),
        }
    };
    view! {
        <span class="row gap-4" id="cosmic-search-main">
            {app}
        </span>
    }
}

#[component]
/// Displays the astronomical google sheets data
pub fn GoogleSheetsData(#[prop(into)] data: SheetData) -> impl IntoView {
    // if data is == 0 or 1 or unimplemented then its unsupported (N/A)
    view! {
        <For
            each=move || data.table.rows.clone()
            key=|username| 0
            children=move |data| {
                view! {
                    <CelestialItem data=SearchedCelestialBodyData {
                        name: get_astronomy_row("Name", data.clone()),
                        link: format!(
                            "data/{}/{}",
                            correct_class("Class", data.clone()),
                            get_astronomy_row("Name", data.clone()).to_lowercase(),
                        ),
                        description: get_astronomy_row("Description", data.clone()),
                        offset: get_astronomy_row("Offset", data.clone()),
                        class: get_astronomy_row("Class", data.clone()),
                        number_of_timezones: get_astronomy_row("#Timezones", data.clone()),
                        months_in_year: get_astronomy_row("MonthsInYear", data.clone()),
                        days_in_year: get_astronomy_row("DaysInYear", data.clone()),
                        julian_date_discovered: get_astronomy_row("JulianDate", data.clone()),
                        calendar_date: get_astronomy_row("CalendarDate", data.clone()),
                        eccentricity: get_astronomy_row("OBE", data.clone()),
                        years_in_days: get_astronomy_row("YearsInDays", data.clone()),
                        days_in_seconds: get_astronomy_row("DaysInSeconds", data.clone()),
                    }/>
                }
            }
        />
    }
}
