use clap::{ Parser, ArgAction };
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
    /// Leave empty to use stdin
    input: Option<String>,

    /// Hide valid pins
    #[arg(short, long, default_value_t = true, action=ArgAction::SetFalse)]
    valid: bool,

    /// Hide invalid pins
    #[arg(short, long, default_value_t = true, action=ArgAction::SetFalse)]
    invalid: bool,

    /// Hide reason for invalid pins
    #[arg(short, long, default_value_t = true, action=ArgAction::SetFalse)]
    reason: bool,

    /// If results should be written in a machine friendly way
    #[arg(short, long, default_value_t = false)]
    porsaline: bool,
}


fn check_pin(
    pin: String,
) -> Result<(), &'static str> {

    let parsed = input::parse(&pin);

    if parsed.is_err() {
        let reason = parsed.unwrap_err();
        return Err(reason);
    }


    let checked = check::full(parsed.unwrap());

    if checked.is_err() {
        let reason = checked.unwrap_err();
        return Err(reason);
    }

    Ok(())
}

fn invalid(pin: String, reason: &str, args: &Cli) {
   if !args.invalid { return; }

   println!(
       "{}",
       match args.porsaline {
           true => {
               match args.reason {
                   true  => format!("! {:15} - {}", pin, reason),
                   false => format!("! {}", pin),
               }
           },

           false => {
               match args.reason {
                   true  => format!("{} is invalid - {}", pin, reason),
                   false => format!("{} is invalid", pin),
               }
           }
       }
   );
}


fn valid(pin: String, args: &Cli) {
   if !args.valid { return; }

  match args.porsaline {
       true  => println!("Y {}", pin),
       false => println!("{} is valid", pin),
   };
}



fn main() {
    let args = Cli::parse();

    if args.input.is_some() {
        let input = args.input.clone().unwrap();
        match check_pin(input.clone()) {
            Ok(()) => {
                valid(input, &args)
            },
            Err(reason) => {
                invalid(input, reason, &args)
            }
        }
        return;
    }

    let stdin = io::stdin();
    let mut valid_count = 0;
    let mut invalid_count = 0;

    loop {
        let mut buffer = String::new();

        match stdin.read_line(&mut buffer) {
            Err(msg) => panic!("{}", msg),
            Ok(0) => {
                println!(
                    "{} valid, {} invalid, {} total",
                    valid_count, invalid_count, valid_count+invalid_count);
                break;
            },
            Ok(_) => {
                match check_pin(buffer.trim().to_string()) {
                    Ok(()) => {
                        valid_count += 1;
                        valid(buffer.trim().to_string(), &args)
                    },
                    Err(reason) => {
                        invalid_count += 1;
                        invalid(buffer.trim().to_string(), reason, &args)
                    }
                }
            }
        }
    }

}


