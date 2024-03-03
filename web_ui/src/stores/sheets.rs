use leptos_use::storage::StringCodec;
use leptos_use::storage::{use_local_storage, JsonCodec};
use leptos::WriteSignal;
use leptos::Signal;
use leptos::server_fn::serde::{Serialize, Deserialize};
use serde_json::value::Value;


#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// This structure represents the data from the google sheets.
pub struct SheetData {
    pub table: Table,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Apart of the google sheets data that stores the columns.
pub struct Column {
    pub id: String,
    pub label: String,
    pub r#type: String,
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Apart of the google sheets data that stores the rows
pub struct Row {
    /// The vector in this case will always hold everyone's' feedback
    pub c: Vec<Object>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
/// Represents a Serde Object
pub struct Object {
    pub f: Option<String>,
    pub v: Option<serde_json::Value>,
}



#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Apart of the google sheets data that stores the tables.
pub struct Table {
    pub cols: Vec<Column>,
    pub rows: Vec<Row>,
    pub parsed_num_headers: i64,
}