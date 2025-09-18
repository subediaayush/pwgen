mod charset;
mod password;
mod cli;
mod utils;
mod generator;
mod logger;
mod clipboard;

use clap::Parser;
use log::{info, error};
use crate::cli::PwGenCli;
use crate::generator::generate_password;
use crate::password::PwArgs;
use crate::logger::set_verbose;

fn main() {
    let cli = PwGenCli::parse();
    let args = PwArgs::from_cli(&cli);

    set_verbose(cli.verbose.unwrap_or(false));

    match generate_password(args) {
        Ok(password) => {
            if cli.copy.unwrap_or(false) {
                clipboard::copy(password)
            } else {
                info!("{}", password);
            }
        }
        Err(e) => error!("Error generating password: {}", e),
    }
}

