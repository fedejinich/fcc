use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[clap(about = "\nfcc is another C compiler ('gcc' recreation by fede... then 'fcc')")]
pub struct Cli {
    /// Path to source code
    // #[clap(short, long)] //, parse(from_os_str))]
    #[clap(required = true)] //, parse(from_os_str))]
    pub source_path: String, //std::path::PathBuf,
    /// Prints lexed code
    #[clap(short, long)]
    pub lex: Option<bool>,
    /// Prints parsed code
    #[clap(short, long)]
    pub parse: Option<bool>,
}
