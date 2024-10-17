use clap::Parser;

#[derive(Parser)]
#[command(version)]
#[command(about = "a track generator for XSPF music playlists")]
pub struct Cli {
    /// music files used to generate information
    pub files: Vec<String>,

    /// configuration file location
    #[arg(short, long)]
    pub config: Option<String>,
}
