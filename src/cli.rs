use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
pub struct Cli {
    /// The path to source code
    #[clap(short, long, parse(from_os_str))]
    pub source_path: std::path::PathBuf,
    /// Prints lexed code
    #[clap(short, long)]
    pub lex: bool,
    /// Prints parsed code
    #[clap(short, long)]
    pub parse: bool,
}
