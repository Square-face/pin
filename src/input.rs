use crate::utils::{ get_date, Pin };

/// Convert a given slice of char codes to the numbers they represent.
///
/// `chars` has to be 12 elements long. Or the function will panic.
/// If any of the chars given are not valid integer char codes, the function will return Err
///
/// # Arguments
/// * `chars` slice of char codes
///
/// # Returns
/// * `Result<Pin, &'static str>` Either a parsed Pin or a Err if any of the given
/// chars weren't numbers
fn parse_12(chars: Vec<char>) -> Result<Pin, &'static str> {

    // panic if `chars is not 10 long`
    assert_eq!(chars.len(), 12);

    // define array to store results in
    let mut result = [0;10];

    for i in 0..10 {
        let char = chars[i+2];
        let digit = char.to_digit(10);

        match digit {
            Some(dig) => result[i] = dig.try_into().unwrap(),
            _ => return Err("Not all numbers")
        };
    };
    

    // check the first 2 digits
    let digit = chars[0].to_digit(10);

    let millenia: i32   = match digit {
        Some(dig) => dig.try_into().unwrap(),
        _ => return Err("Not all numbers")
    };


    let digit = chars[1].to_digit(10);

    let centry: i32     = match digit {
        Some(dig) => dig.try_into().unwrap(),
        _ => return Err("Not all numbers")
    };


    // OOxxxxxx-xxxx
    let centry = millenia*10 + centry;

    return Ok(Pin {
        nums: result,
        plus: false,
        date: get_date(result, false, Some(centry))
    })
}





/// Convert a given slice of char codes to the numbers they represent.
///
/// `chars` has to be 10 elements long. Or the function will panic.
/// If any of the chars given are not valid integer char codes, the function will return Err
/// If `plus` is true and the parser didn't fail for other reasons, the year returned for the
/// date will be reduced by 100. i.e the person is 100 years older than expected
///
/// # Arguments
/// * `chars` slice of char codes
/// * `plus` Wether or not a plus was used instead of a minus.
///
/// # Returns
/// * `Result<Pin, &'static str>` Either a parsed Pin or a Err if any of the given
/// chars weren't numbers
fn parse_10(chars: Vec<char>, plus: bool) -> Result<Pin, &'static str> {

    // panic if `chars is not 10 long`
    assert_eq!(chars.len(), 10);

    // define array to store results in
    let mut result = [0;10];

    for i in 0..10 {
        let char = chars[i];
        let digit = char.to_digit(10);

        match digit {
            Some(dig) => result[i] = dig.try_into().unwrap(),
            _ => return Err("Not all numbers")
        };
    };

    return Ok(Pin {
        nums: result,
        plus,
        date: get_date(result, plus, None)
    })
}




/// Parses a given input string into a array of the individual integers
///
/// # Arguments
/// * `input` The input string to parse
/// 
/// # Returns
/// * `Result<[u8;10], &'static str>` Array of numbers or a message explaining why pin is
/// invalid
pub fn parse(input: &String) -> Result<Pin, &'static str> {
    let mut chars = input.chars().collect::<Vec<char>>();

    return match chars.len() {
        10 => parse_10(chars, false),
        12 => parse_12(chars),

        11 => {
            let div = chars.remove(6);
            
            if div != '-' && div != '+' { // check for - or + in the 7th spot
                return Err("7th char must be - or +")
            }


            parse_10(chars, div == '+')
        }

        13 => {
            let div = chars.remove(8);

            if div != '-' { // since we are given the full year, plus can't be used
                return Err("9th char must be -")
            }


            parse_12(chars)
        }

        _ => {
            // Length is invalid
            
            if input.len() > 11 {
                Err("Too long")
            } else {
                Err("Too short")
            }
        }
    }
}
