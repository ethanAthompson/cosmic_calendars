use leptos_use::storage::StringCodec;
use leptos_use::storage::{use_local_storage, JsonCodec};
use leptos::WriteSignal;
use leptos::Signal;
use leptos::server_fn::serde::{Serialize, Deserialize};
use serde_json::value::Value;


#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// This structure represents the timezone you choose within the dashboard
pub struct CosmicTimeColumn {
    pub host: String,
    pub tz: String,
    pub time: String
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// This structure represents the date you choose within the dashboard
pub struct CosmicDateRow {
   pub cal: String,
   pub host: String,
   pub date: String,
   pub id: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// This structure represents the timezone you choose
pub struct CosmicTimeZoneState {
    pub name: String,
    pub timezone: String
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// This structure represents the timezone you choose
pub struct EarthCalendarState {
    pub preferred: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// This structure represents the feedback search customization
pub struct FeedbackSearchFilterStates {
    pub resolved: bool,
    pub un_resolved: bool,
    pub ignored: bool,
    pub disabled_resolved: bool,
    pub disabled_un_resolved: bool,
    pub disabled_ignored: bool

}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// This structure represents the search features
pub struct FeedbackSearchStates {
    pub value: String
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// This structure contains the format for all searched  (celestial bodies).
pub struct SearchedCelestialBodyData {
    pub name: String,
    pub link: String,
    pub description: String,
    pub offset: String,
    pub class: String,
    pub number_of_timezones: String,
    pub months_in_year: String,
    pub days_in_year: String,
    pub julian_date_discovered: String,
    pub calendar_date: String,
    pub eccentricity: String,
    pub years_in_days: String,
    pub days_in_seconds: String
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// This structure represents the state of the username
pub struct UserNameState {
    pub value: String
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// This structure represents the state of either chosen celestial time conversions
pub struct CelestialChosenNameState {
    pub left: String,
    pub right: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// This structure represents a log of the state of the datetime conversion process
pub struct DateTimeStateAccounting {
    pub accounting_date: Vec<CelestialDateState>,
    pub accounting_time: Vec<CelestialTimeState>
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// This structure represents the state of the time conversion process
pub struct CelestialTimeState {
    pub left_time: CelestialTime,
    pub right_time: CelestialTime,
    pub converted_time: CelestialTime,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// This structure represents the state of the date conversion process
pub struct CelestialDateState {
    pub left_date: CelestialDate,
    pub right_date: CelestialDate,
    pub converted_date: CelestialDate,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// This structure represents the state of a calendar selected
pub struct CelestialCalendar {
    pub left: String,
    pub right: String
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// This structure represents the state of a locale selected (any other planet not earth)
pub struct CelestialLocaleNonEarths {
    pub left: String,
    pub right: String
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// This structure represents the state of a timezone selected
pub struct CelestialLocale {
    pub left: String,
    pub right: String
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// This structure represents the state a celestial date
pub struct CelestialTime {
    pub hour: String,
    pub minute: String,
    pub second: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// This structure represents the state a celestial date
pub struct CelestialDate {
    pub year: String,
    pub month: String,
    pub day: String,
    pub ls: String
}
