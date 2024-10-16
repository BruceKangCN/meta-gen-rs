pub mod cli;

use std::{fs::File, io::Read};

use clap::Parser;
use cli::Cli;
use dirs::config_dir;
use rusty_ffmpeg_meta::{generate_info, util::Serializer};

fn main() {
    let cli = Cli::parse();

    let default_path = config_dir()
        .unwrap_or_default()
        .join("rfm.toml")
        .to_str()
        .unwrap()
        .to_owned();
    let config_path = cli.config.unwrap_or(default_path);
    let mut config_file = File::open(config_path).unwrap();
    let mut content = String::new();
    config_file.read_to_string(&mut content).unwrap();
    let config = toml::from_str(&content).unwrap();

    let serializer = Serializer { config };

    for location in cli.files {
        print!("{}", generate_info(&location, &serializer).unwrap());
    }
}
