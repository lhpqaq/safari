use safari::*;
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::path::PathBuf;

fn get_time() -> String {
    let now = chrono::Local::now();
    now.format("%Y-%m-%d %H:%M").to_string()
}

fn get_json() -> (Value, String) {
    match get_safari_windows_and_tabs_as_json() {
        Ok(json_output) => {
            let mut parsed_json: Value = serde_json::from_str(&json_output).unwrap();
            let timestamp = get_time();
            parsed_json["time"] = json!(timestamp);

            let filename = format!(
                "safari_{}.json",
                timestamp.replace(":", "-").replace(" ", "_")
            );

            return (parsed_json, filename);
        }
        Err(e) => {
            eprintln!("Error fetching Safari windows: {}", e);
            std::process::exit(1);
        }
    }
}

fn get_safari_dir() -> String {
    let home_dir = env::var("HOME").expect("Unable to get HOME directory");

    let mut safari_dir = PathBuf::from(&home_dir);
    safari_dir.push(".safari");

    if !safari_dir.exists() {
        fs::create_dir_all(&safari_dir).expect("Failed to create Safari directory");
    }

    safari_dir.to_str().unwrap().to_string()
}

pub fn dump() {
    let (parsed_json, filename) = get_json();
    let safari_dir = get_safari_dir();
    let json_path = format!("{}/{}", safari_dir, filename);
    fs::write(
        json_path,
        serde_json::to_string_pretty(&parsed_json).unwrap(),
    )
    .expect("Unable to write file");
}

pub fn reopen() {
    let safari_dir = get_safari_dir();
    let entries = fs::read_dir(&safari_dir).expect("Unable to read directory");

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        if filename.starts_with("safari_") && filename.ends_with(".json") {
            let content = fs::read_to_string(&path).expect("Unable to read file");
            let json_data: Value = serde_json::from_str(&content).unwrap();
            println!("Parsed JSON: {}", json_data);
        }
    }
}
