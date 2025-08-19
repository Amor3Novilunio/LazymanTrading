use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;

// ---------------------------------------------------------------
// Create A Json Based on Type(Struct) Via read<Token>("location")
// ---------------------------------------------------------------
pub fn create<T: Serialize>(file: &str, json_config: &T) {
    // ----------------
    // Create the file
    // ----------------
    let file = match File::create(file) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed To Create File | ERR: {err}")),
    };

    // -------------------
    // File Write to File
    // -------------------
    match serde_json::to_writer_pretty(file, json_config) {
        Ok(res) => {
            println!("-------------------------------");
            println!("Config Created");
            println!("-------------------------------");
            res
        }
        Err(err) => std_error_exit!(format!("Failed To Write File | ERR: {err}")),
    };
}
