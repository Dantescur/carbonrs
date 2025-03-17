use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Parser)]
#[command(name = "carbonrs")]
#[command(version = "0.1.0")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to the input code file
    #[arg(required_unless_present_any = [ "list_fonts", "list_themes" ])]
    pub input: Option<String>,

    /// Path to the output image file (default: output.png)
    #[arg(short, long)]
    pub output: Option<String>,

    /// Use specific font by name
    #[arg(short, long)]
    pub font: Option<String>,

    /// List available monospace fonts
    #[arg(long, conflicts_with_all = ["input", "output","font"], action = clap::ArgAction::SetTrue)]
    pub list_fonts: bool,

    ///Turn debugginf information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    #[arg(short, long)]
    pub line_numbers: bool,

    #[arg(short, long)]
    pub no_windows: bool,

    #[arg(short, long)]
    pub theme: Option<String>,

    #[arg(long, conflicts_with_all = ["input", "output", "font", "theme"], action = clap::ArgAction::SetTrue)]
    pub list_themes: bool,
}
