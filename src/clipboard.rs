use log::{error, info};

pub fn copy(string: String) {
    match cli_clipboard::set_contents(string) {
        Ok(_) => info!("Password copied to clipboard."),
        Err(e) => error!("Failed to copy password to clipboard: {}", e),
    };
}