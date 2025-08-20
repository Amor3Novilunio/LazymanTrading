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
