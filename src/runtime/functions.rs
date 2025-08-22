use std::time::{SystemTime, UNIX_EPOCH};

pub struct JoinStringProps {
    pub split_prefix: String,
    pub string_collection: String,
}

pub fn join_string(props: JoinStringProps) -> Vec<String> {
    let mut new_array: Vec<String> = Vec::new();

    for collection in props.string_collection.split(&props.split_prefix) {
        new_array.push(collection.trim().to_string());
    }

    new_array
}

pub fn get_unix_clock() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        .to_string()
}
