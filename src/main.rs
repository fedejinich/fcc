use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CompilerDriver  {
    program_path: String,

    #[arg(long)]
    lex: bool,

    #[arg(long)]
    parse: bool,

    #[arg(long)] // todo this shoulde be code-gen
    code_gen: bool,
}

fn main() {
    let driver = CompilerDriver::parse();
    println!("{:?}", driver);
}
