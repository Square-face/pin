use crate::utils::{ get_date, Pin };

/// Convert a given vec of chars into the numbers they represent.
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
        // get the digit the char represents
        let char = chars[i];
        let digit = char.to_digit(10);

        match digit {
            Some(dig) => {
                result[i] = dig.try_into().unwrap()
            },
            None => return Err("Not all numbers")
        };
    };

    // Create parsed pin
    return Ok(Pin {
        nums: result,
        plus,
        date: get_date(
            result,
            plus,
            None
        )
    })
}










/// Convert a given vec chars into the numbers they represent.
///
/// `chars` has to be 12 elements long. Or the function will panic.
/// If any of the chars given are not valid integer char codes, the function will return Err
///
/// Since the full year is given no flag for if a plus was used is required.
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
        // get the digit the char represents
        let char = chars[i+2];
        let digit = char.to_digit(10);

        match digit {
            Some(dig) => {
                result[i] = dig.try_into().unwrap()
            },
            None => return Err("Not all numbers")
        };
    };
    

    let digit = chars[0].to_digit(10);

    if digit.is_none() {
        return Err("Not all numbers")
    }

    let millenia: i32 = digit.unwrap().try_into().unwrap();




    let digit = chars[1].to_digit(10);

    if digit.is_none() {
        return Err("Not all numbers")
    }

    let centry: i32 = digit.unwrap().try_into().unwrap();




    // OOxxxxxx-xxxx
    let centry = millenia*10 + centry;


    // Create parsed pin
    return Ok(Pin {
        nums: result,
        plus: false,
        date: get_date(
            result,
            false,
            Some(centry)
        )
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
        11 => {
            // extract the 7th char
            let extra = chars.remove(6);
            
            // check for - or + in the 7th spot
            if extra != '-' && extra != '+' { 
                return Err("7th char must be - or +")
            }

            // parse numbers
            parse_10(chars, extra == '+')
        }

        12 => parse_12(chars),
        13 => {
            // extract the 9th char
            let extra = chars.remove(8);

            // check if the 9th spot is a -
            // since we are given the full year, plus can't be used
            if extra != '-' { 
                return Err("9th char must be -")
            }

            // parse numbers
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
