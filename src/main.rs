use clap::Parser;

mod check;
mod input;
mod testing;



#[derive(Parser, Debug)]
#[command(
    author="Linus Michelsson",
    version="0.0.1",
    about="cli tool for checking and generating swedish personal identity numbers",
)]
struct Cli {
    /// Input string, leave empty to use stdin
    input: Option<String>,

    /// path to input file
    #[arg(short, long)]
    file: Option<String>,

    /// path to output
    #[arg(short, long)]
    output: Option<String>,
}



fn main() {
    let args = Cli::parse();

    if args.input.is_none() { unimplemented!(); }
}


