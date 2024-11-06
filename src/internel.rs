use safari::*;
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
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

    // 首先列出目录中的所有以 safari_ 开头且以 .json 结尾的文件
    let mut safari_files = vec![];

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();

        if filename.starts_with("safari_") && filename.ends_with(".json") {
            safari_files.push(filename.to_string());
        }
    }

    if safari_files.is_empty() {
        println!("No Safari session files found.");
        return;
    }

    println!("Found the following Safari session files:");
    for (i, file) in safari_files.iter().enumerate() {
        println!("{}: {}", i + 1, file);
    }

    print!("Select a file to parse (input the number): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let selected = input.trim().parse::<usize>().unwrap();

    if selected == 0 || selected > safari_files.len() {
        println!("Invalid selection.");
        return;
    }

    let selected_file = &safari_files[selected - 1];
    let path = format!("{}/{}", safari_dir, selected_file);
    let content = fs::read_to_string(&path).expect("Unable to read file");

    let json_data: Value = serde_json::from_str(&content).unwrap();
    println!("Parsed JSON: {}", json_data);

    let windows = json_data.as_object().unwrap();
    let mut window_names = vec![];
    for (key, value) in windows {
        if key != "time" {
            window_names.push(key.to_string());
            println!("{}: {:?}", key, value);
        }
    }

    println!("Choose a window to open, or type 'all' to open all windows:");
    for (i, window) in window_names.iter().enumerate() {
        println!("{}: {}", i + 1, window);
    }
    print!("Your choice: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    if choice == "all" {
        for window in windows {
            if window.0 != "time" {
                open_tabs_in_window(window.1.as_array().unwrap());
            }
        }
    } else if let Ok(index) = choice.parse::<usize>() {
        if index > 0 && index <= window_names.len() {
            let selected_window = windows.get(&window_names[index - 1]).unwrap();
            open_tabs_in_window(selected_window.as_array().unwrap());
        } else {
            println!("Invalid window selection.");
        }
    } else {
        println!("Invalid input.");
    }
}

fn open_tabs_in_window(tabs: &[Value]) {
    for tab in tabs {
        let url = &tab["url"].as_str().unwrap();
        println!("Opening URL: {}", url);

        Command::new("open")
            .arg(url)
            .output()
            .expect("Failed to open URL");
    }
}

pub fn list() {
    let safari_dir = get_safari_dir();
    let entries = fs::read_dir(&safari_dir).expect("Unable to read directory");

    let mut safari_files = vec![];

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();

        if filename.starts_with("safari_") && filename.ends_with(".json") {
            safari_files.push(filename.to_string());
        }
    }

    if safari_files.is_empty() {
        println!("No Safari session files found.");
        return;
    }

    safari_files.sort_by(|a, b| {
        let a_time = a.trim_start_matches("safari_").trim_end_matches(".json");
        let b_time = b.trim_start_matches("safari_").trim_end_matches(".json");
        b_time.cmp(a_time)
    });

    println!("Found the following Safari session files:");
    for (i, file) in safari_files.iter().enumerate() {
        println!("{}: {}", i + 1, file);
    }

    print!("Select a file to parse (input the number): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let selected = input.trim().parse::<usize>().unwrap();

    if selected == 0 || selected > safari_files.len() {
        println!("Invalid selection.");
        return;
    }

    let selected_file = &safari_files[selected - 1];
    let path = format!("{}/{}", safari_dir, selected_file);
    let content = fs::read_to_string(&path).expect("Unable to read file");

    let json_data: Value = serde_json::from_str(&content).unwrap();

    let windows = json_data.as_object().unwrap();
    println!("Safari Windows and Tabs:");
    for (window, tabs) in windows {
        if window != "time" {
            println!("\n{}:", window);
            let tabs = tabs.as_array().unwrap();
            for tab in tabs {
                let title = &tab["title"].as_str().unwrap_or("No Title");
                let url = &tab["url"].as_str().unwrap();
                println!("- \x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, title);
            }
        }
    }
}
