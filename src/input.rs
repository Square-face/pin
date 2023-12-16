use crate::utils::{ Date, get_date, Parsed, Pin };

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
fn parse_12(chars: &[u8], plus: bool) -> Result<Pin, &'static str> {

    // panic if `chars is not 10 long`
    assert_eq!(chars.len(), 12);

    // define array to store results in
    let mut result = [0;10];

    for i in 2..12 {
        let char = chars[i];

        if char > 0x39 || char < 0x30 { // 0x39 = '9' and 0x30 = '0'
            return Err("Input was not all numbers");
        }

        // since the charcodes for numbers come one after another in increasing order
        // i.e 0x30 = '0' 0x31 = '1', 0x32 = '2' etc, we can simply take the charcode minus 0x30 to
        // get the integer it represents.
        result[i-2] = char - 0x30;
    };
    

    // check the first 2 digits
    let char = chars[0];
    if char > 0x39 || char < 0x30 { 
            return Err("Input was not all numbers");
    }
    let millenia = char - 0x30;

    let char = chars[1];
    if char > 0x39 || char < 0x30 { 
            return Err("Input was not all numbers");
    }
    let centry = char - 0x30;

    // OOxxxxxx-xxxx
    let centry = (millenia*10 + centry) as i32;

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
fn parse_10(chars: &[u8], plus: bool) -> Result<Pin, &'static str> {

    // panic if `chars is not 10 long`
    assert_eq!(chars.len(), 10);

    // define array to store results in
    let mut result = [0;10];

    for i in 0..10 {
        let char = chars[i];

        if char > 0x39 || char < 0x30 { // 0x39 = '9' and 0x30 = '0'
            return Err("Input was not all numbers");
        }

        // since the charcodes for numbers come one after another in increasing order
        // i.e 0x30 = '0' 0x31 = '1', 0x32 = '2' etc, we can simply take the charcode minus 0x30 to
        // get the integer it represents.
        result[i] = char - 0x30;
    };


    return Ok(Pin {
        nums: result,
        plus: false,
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


    return match input.len() {
        10 => parse_10(input.as_bytes(), false),
        12 => parse_12(input.as_bytes(), false),

        11 => {
            let chars = input.as_bytes();
            
            if chars[6] != 0x2D && chars[6] != 0x2B { // check for - or + in the 7th spot
                return Err("7th char must be - or +")
            }

            let concated = [&chars[..6], &chars[7..]].concat();

            parse_10(&concated, chars[6] == 0x2B)
        }

        13 => {
            let chars = input.as_bytes();

            if chars[8] != 0x2D { // since we are given the full year, plus can't be used
                return Err("9th char must be -")
            }

            let concated = [&chars[..8], &chars[9..]].concat();

            parse_12(&concated, chars[8] == 0x2B)
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
