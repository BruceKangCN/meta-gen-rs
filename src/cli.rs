use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    pub files: Vec<String>,
    #[arg(short, long)]
    pub config: Option<String>,
}
