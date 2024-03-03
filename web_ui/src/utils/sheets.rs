use leptos::{create_resource, create_signal, logging, ReadSignal, Resource, WriteSignal};

use crate::stores::sheets::Row;
use crate::stores::sheets::SheetData;
use crate::stores::states::FeedbackSearchFilterStates;
use crate::stores::states::FeedbackSearchStates;
use crate::utils::str::rustify_str;

/// A wrapper to abstract the google sheets api into a [`SheetData`] structure
pub fn get_data(
    data: ReadSignal<&str>,
    set_data: WriteSignal<&str>,
    url: &'static str,
) -> Resource<&'static str, SheetData> {
    // our source signal: some synchronous, local state
    let (data, set_data) = create_signal("");

    // our resource
    create_resource(
        data,
        // every time `data` changes, this will run
        move |value| async move {
            // gets name and feedback type,
            // this can happen when a user clicks on a
            // pill that says filter etch.. or a checkbox..
            // let feedback_url = filter_feedback_type(feedback_url);

            let body = reqwest::get(url)
                .await
                .expect("No URL")
                .text()
                .await
                .expect("Google Visualization Query");

            // removes the unnecessary non-json code and trailing spaces " "
            let body = &body[47..&body.len() - 2];
            // logging::log!("body = {:?}", body);
            let json_body: SheetData = serde_json::from_str(body).expect("json");
            // logging::log!("Users: {:?}", json_body.table.cols);
            // logging::log!("Vbox User 1 = {:?}", json_body.table.rows[0]);
            // logging::log!("Vbox User 2 = {:?}", json_body.table.rows[1]);

            json_body
        },
    )
}

/// This function abstracts the Row structure so its easier to call.
pub fn get_astronomy_row(name: &'static str, data: Row) -> String {
    // logging::log!("{:?}", data.c[4].v.clone().unwrap().to_string());

    match name {
        "Class" => rustify_str(data.c[0].v.clone().unwrap().to_string()),
        "Name" => rustify_str(data.c[1].v.clone().unwrap().to_string()),
        "Description" => rustify_str(data.c[2].v.clone().unwrap().to_string()),
        "Offset" => rustify_str(data.c[3].v.clone().unwrap().to_string()),
        "#Timezones" => rustify_str(data.c[4].v.clone().unwrap().to_string()),
        "MonthsInYear" => rustify_str(data.c[5].v.clone().unwrap().to_string()),
        "DaysInYear" => rustify_str(data.c[6].v.clone().unwrap().to_string()),
        "JulianDate" => rustify_str(data.c[7].v.clone().unwrap().to_string()),
        "CalendarDate" => rustify_str(data.c[8].v.clone().unwrap().to_string()),
        "OBE" => rustify_str(data.c[9].v.clone().unwrap().to_string()),
        "YearsInDays" => rustify_str(data.c[10].v.clone().unwrap().to_string()),
        "DaysInSeconds" => rustify_str(data.c[11].v.clone().unwrap().to_string()),
        _ => "Unknown".to_string(),
    }
}

/// This function allows the correct route to be chosen
pub fn correct_class(name: &'static str, data: Row) -> &'static str {
    match get_astronomy_row("Class", data.clone()).as_str() {
        "Planet" => "planets",
        "Moon" => "moons",
        "Asteroid" => "asteroids",
        "Exo Planet" => "exo-planets",
        "Comet" => "comets",
        _ => "unknown",
    }
}

/// This function hides a card based on the stats in localstorage.
pub fn has_resolved(
    data: Row,
    filter: FeedbackSearchFilterStates,
    search: FeedbackSearchStates,
    typeof_feedback: String,
) -> bool {
    let item = data.c[10].v.clone().unwrap().to_string();
    let name = data.c[1].v.clone().unwrap().to_string();
    let feedback_type = data.c[6].v.clone().unwrap().to_string();

    if rustify_str(name).contains(&search.value) {
        if filter.ignored == true {
            false
        } else {
            if rustify_str(item) == "WIP".to_string() {
                if filter.un_resolved == true {
                    if rustify_str(feedback_type) == typeof_feedback {
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            } else {
                if filter.resolved == true {
                    if rustify_str(feedback_type) == typeof_feedback {
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            }
        }
    } else {
        true
    }
}

//
