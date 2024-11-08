use clap::Parser;

#[derive(Parser)]
#[command(version, arg_required_else_help = true)]
/// a track generator for XSPF music players
///
/// A track generator designed for `Audacious` player, but can also be used for
/// other players that supports XSPF playlist format.
pub struct Cli {
    /// music files used to generate information
    pub files: Vec<String>,

    #[arg(short, long)]
    /// configuration file location
    pub config: Option<String>,
}
