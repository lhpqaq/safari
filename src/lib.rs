use std::process::Command;
use std::str;
pub mod utils;

pub fn get_safari_windows() -> Result<String, String> {
    let script = r#"
        tell application "Safari"
            set window_list to {}
            repeat with w in windows
                set tab_list to {}
                repeat with t in tabs of w
                    set end of tab_list to URL of t
                end repeat
                set end of window_list to tab_list
            end repeat
        end tell
        return window_list
    "#;

    let output = Command::new("osascript").arg("-e").arg(script).output();
    match output {
        Ok(output) if output.status.success() => {
            let stdout = str::from_utf8(&output.stdout).map_err(|e| e.to_string())?;
            Ok(stdout.to_string())
        }
        Ok(output) => {
            let stderr = str::from_utf8(&output.stderr).unwrap_or("Unknown error");
            Err(stderr.to_string())
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn get_safari_active_tabs() -> Result<String, String> {
    let script = r#"
        tell application "Safari"
            set active_list to {}
            repeat with w in windows
                set end of active_list to URL of current tab of w
            end repeat
        end tell
        return active_list
    "#;

    let output = Command::new("osascript").arg("-e").arg(script).output();

    match output {
        Ok(output) if output.status.success() => {
            let stdout = str::from_utf8(&output.stdout).map_err(|e| e.to_string())?;
            Ok(stdout.to_string())
        }
        Ok(output) => {
            let stderr = str::from_utf8(&output.stderr).unwrap_or("Unknown error");
            Err(stderr.to_string())
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn get_safari_windows_and_tabs_as_json() -> Result<String, String> {
    let script = r#"
        tell application "Safari"
        set window_list to ""
            repeat with w in windows
                set tab_list to ""
                repeat with t in tabs of w
                    set tab_info to (name of t & " - " & URL of t)
                    set tab_list to tab_list & tab_info & linefeed
                end repeat
                set window_list to window_list & "Safari Window " & index of w & ":" & linefeed & tab_list & linefeed
            end repeat
        return window_list
        end tell
    "#;

    let output = Command::new("osascript").arg("-e").arg(script).output();

    match output {
        Ok(output) if output.status.success() => {
            let stdout = str::from_utf8(&output.stdout).map_err(|e| e.to_string())?;
            let parsed = utils::parse_safari_output(stdout);
            let json_result = serde_json::to_string_pretty(&parsed).map_err(|e| e.to_string())?;

            Ok(json_result)
        }
        Ok(output) => {
            let stderr = str::from_utf8(&output.stderr).unwrap_or("Unknown error");
            Err(stderr.to_string())
        }
        Err(err) => Err(err.to_string()),
    }
}
