use regex::Regex;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result, Write};
use std::str;

pub fn parse_safari_output(input: &str) -> serde_json::Value {
    let mut result = HashMap::new();
    let mut current_window = String::new();
    let re_window = Regex::new(r"Safari Window (\d+):").unwrap();
    let re_entry = Regex::new(r"(.*?) - (http.*)").unwrap();

    for line in input.lines() {
        if let Some(caps) = re_window.captures(line) {
            current_window = format!("Window {}", &caps[1]);
            result.insert(current_window.clone(), Vec::new());
        } else if let Some(caps) = re_entry.captures(line) {
            if !current_window.is_empty() {
                let entry = json!({
                    "title": caps[1].trim(),
                    "url": caps[2].trim(),
                });
                result.get_mut(&current_window).unwrap().push(entry);
            }
        }
    }

    json!(result)
}

pub fn save_to_file(content: &str, file_path: &str) -> Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn read_from_file(file_path: &str) -> Result<Value> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let json_data: Value = serde_json::from_str(&content)?;
    Ok(json_data)
}
