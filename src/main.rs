use safari::*;

fn main() {
    // match get_safari_active_tabs() {
    //     Ok(active_tabs) => println!("Active tabs in Safari windows: \n{}", active_tabs),
    //     Err(e) => eprintln!("Error fetching active Safari tabs: {}", e),
    // }
    // match get_safari_windows() {
    //     Ok(windows) => println!("Open Safari windows and tabs: \n{}", windows),
    //     Err(e) => eprintln!("Error fetching Safari windows: {}", e),
    // }
    // match get_safari_windows_and_tabs() {
    //     Ok(windows) => println!("Open Safari windows and tabs: \n{}", windows),
    //     Err(e) => eprintln!("Error fetching Safari windows and tabs: {}", e),
    // }
    match get_safari_windows_and_tabs_as_json() {
        Ok(json_output) => println!("{}", json_output),
        Err(e) => eprintln!("Error fetching Safari windows: {}", e),
    }
}
