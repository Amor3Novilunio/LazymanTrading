use crate::std_error_exit;
use serde::de::DeserializeOwned;
use std::fs::File;

// ---------------------------------------------
// Read A Json Via read<Token>("location")
// ---------------------------------------------
pub fn read<T: DeserializeOwned>(path: &str) -> T {
    // --------------------------------
    // Store the File in Memory Buffer
    // --------------------------------
    let file = match File::open(path) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed To Open Path | ERR: {err}")),
    };

    // -----------------------------------------
    // Extract the json file from memory buffer
    // -----------------------------------------
    match serde_json::from_reader(file) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed To Deserialize | ERR: {err}")),
    }
}
