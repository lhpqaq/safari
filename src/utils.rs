use regex::Regex;
use serde_json::json;
use std::collections::HashMap;
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
