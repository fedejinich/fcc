use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
pub struct Cli {
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    pub source_path: std::path::PathBuf,
}

// fn foo() {
//     let args = Cli::parse();
//     args.source_path
// }
