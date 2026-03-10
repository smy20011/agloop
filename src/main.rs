use std::fs;

use crate::config::Config;

mod command;
mod config;
mod task;

fn main() {
    let config_file_content = fs::read_to_string("agloop.toml").expect("Cannot read agloop.toml");
    let _config: Config = toml::from_str(&config_file_content).unwrap();
}
