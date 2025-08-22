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

pub struct QueryStringProps {
    pub query: Vec<(String, String)>,
}

pub fn query_string(props: QueryStringProps) -> String {
    let mut result: Vec<String> = Vec::new();
    for query in props.query {
        let (key, value) = query;
        result.push(format!("{}={}", key, value));
    }

    result.join("&")
}
