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

    /// If only invalid pins should be shown
    #[arg(long, default_value_t=false)]
    hide_valid: bool,
}


fn check_pin(
    pin: String,
) -> Result<String, String> {

    let parsed = input::parse(&pin);

    if parsed.is_err() {
        let reason = parsed.unwrap_err();
        return Err(format!("{:15} is invalid - {}", pin, reason));
    }


    let checked = check::full(parsed.unwrap());

    if checked.is_err() {
        let reason = checked.unwrap_err();
        return Err(format!("{:15} is invalid - {}", pin, reason));
    }

    Ok(format!("{:15} is valid", pin))
}

fn output(msg: String) {
    println!("{}", msg);
}


fn main() {
    let args = Cli::parse();

    if args.input.is_some() {
        let input = args.input.unwrap();
        match check_pin(input) {
            Ok(msg) => {
                if !args.hide_valid {output(msg)}
            },
            Err(msg) => {
                output(msg)
            }
        }
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
                match check_pin(buffer.trim().to_string()) {
                    Ok(msg) => {
                        valid += 1;
                        if !args.hide_valid {output(msg)}
                    },
                    Err(msg) => {
                        invalid += 1;
                        output(msg)
                    }
                }
            }
        }
    }

}


