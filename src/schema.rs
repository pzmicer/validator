diesel::table! {
    tracked_data (id) {
        id -> Int4,
        arrived_at -> Timestamp,
        source_machine -> Text,
        event_type -> Int4,
        validation_ok -> Bool,
        validation_error -> Nullable<Text>,
        found_fields -> Nullable<Text>,
        raw_json -> Nullable<Text>,
        raw_xml -> Text,
    }
}
