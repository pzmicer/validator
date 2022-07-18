use std::time::SystemTime;

use diesel::Insertable;
use serde::Deserialize;
use crate::schema::tracked_data;

#[derive(Insertable)]
#[diesel(table_name = tracked_data)]
pub struct NewTrackedData {
    pub arrived_at: SystemTime,
    pub source_machine: String,
    pub event_type: i32,
    pub validation_ok: bool,
    pub validation_error: Option<String>,
    pub found_fields: Option<String>,
    pub raw_json: Option<String>,
    pub raw_xml: String,
}

#[derive(Default, Debug, Deserialize, PartialEq)]
pub struct RequiredData {
    #[serde(rename = "type")]
    pub event_type: i32,
    pub str_field2: String,
}

#[derive(Default, Debug, Deserialize, PartialEq)]
pub struct JsonData {
    pub str_field6: String,
}

#[derive(Default, Debug, Deserialize, PartialEq)]
pub struct XmlData {
    pub organizational_identifier: String,
    pub unique_client: String,
    pub hmac: String,
    pub protocol_version: f64,
    pub client_version: f64,
    pub ck: String,
    pub list_of_files: String,
    pub meta_data1: String,
    pub meta_data2: String,
    pub int_field1: i32,
    pub int_field2: i32,
    pub int_field3: i32,
    pub int_field4: i32,
    pub int_field5: i32,
    pub int_field6: i32,
    pub int_field7: i32,
    pub int_field8: i32,
    pub int_field9: i32,
    pub int_field10: i32,
    pub str_field1: String,
    pub str_field3: String,
    pub str_field4: String,
    pub str_field5: String,
    pub str_field6: String,
    pub str_field7: String,
    pub str_field8: String,
    pub str_field9: String,
    pub str_field10: String,
}