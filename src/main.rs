use safari::*;

fn main() {
    let file_path = "safari.json";
    match get_safari_windows_and_tabs_as_json() {
        Ok(json_output) => {
            utils::save_to_file(&json_output, &file_path).unwrap();
        }
        Err(e) => eprintln!("Error fetching Safari windows: {}", e),
    }

    match utils::read_from_file(file_path) {
        Ok(json_data) => println!("Parsed JSON: {}", json_data),
        Err(e) => println!("Failed to parse JSON: {}", e),
    }
}
