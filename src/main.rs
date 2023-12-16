use clap::Parser;
use std::io;

mod check;
mod input;
mod utils;
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


fn check_pin(pin: String) -> bool {

    let parsed = input::parse(&pin);

    if parsed.is_err() {
        let reason = parsed.unwrap_err();
        println!("x {} - {}", pin, reason);
        return false
    }


    let checked = check::full(parsed.unwrap());

    if checked.is_err() {
        let reason = checked.unwrap_err();
        println!("x {} - {}", pin, reason);
        return false
    }

    println!("  {}", pin);

    true
}


fn main() {
    let args = Cli::parse();

    if args.input.is_some() {
        let input = args.input.unwrap();
        check_pin(input);
        return;
    }

    let stdin = io::stdin();
    let mut valid = 0;
    let mut invalid = 0;

    loop {
        let mut buffer = String::new();

        match stdin.read_line(&mut buffer) {
            Err(msg) => panic!("{}", msg),
            Ok(0) => {
                println!(
                    "{} valid, {} invalid, {} total",
                    valid, invalid, valid+invalid);
                break;
            },
            Ok(_) => {
                if check_pin(buffer.trim().to_string()) {
                    valid += 1;
                } else {
                    invalid += 1;
                }
            }
        }
    }

}


