
#[derive(Debug, Clone, Copy)]
pub struct Parsed {
    pub nums: [u8;10],
    pub plus: bool,
}

fn to_nums(chars: &[u8]) -> Result<[u8;10], &'static str> {
    //! Convert a given slice of char codes to the numbers they represent.
    //!
    //! `chars` has to be 10 elements long. Or the function will panic.
    //! If any of the chars given are not valid integer char codes, the function will return Err
    //!
    //! # Arguments
    //! * `chars` slice of char codes
    //!
    //! # Returns
    //! * `Result<[u8;10], &'static str>` Either an array of numbers or a Err if any of the given
    //! chars weren't numbers

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

    return Ok(result);
}

pub fn parse(input: &String) -> Result<Parsed, &'static str> {
    //! Parses a given input string into a array of the individual integers
    //!
    //! # Arguments
    //! * `input` The input string to parse
    //! 
    //! # Returns
    //! * `Result<[u8;10], &'static str>` Array of numbers or a message explaining why pin is
    //! invalid


    return match input.len() {
        10 => { // pin formated without -

            match to_nums(input.as_bytes()){
                Err(reason) => Err(reason),
                Ok(result) => Ok(Parsed{
                    nums: result,
                    plus: false,
                })
            }
        }

        12 => {
            match to_nums(&input.as_bytes()[2..]){
                Err(reason) => Err(reason),
                Ok(result) => Ok(Parsed{
                    nums: result,
                    plus: false,
                })
            }

        }

        11 => {
            let chars = input.as_bytes();
            
            if chars[6] != 0x2D && chars[6] != 0x2B { // check for - or + in the 7th spot
                return Err("7th char must be - or +")
            }

            let concated = [&chars[..6], &chars[7..]].concat();

            match to_nums(&concated) {
                Err(reason) => Err(reason),
                Ok(result) => Ok(Parsed {
                    nums: result,
                    plus: chars[6] == 0x2D
                })
            }
        }

        13 => {
            let chars = input.as_bytes();

            if chars[8] != 0x2D && chars[8] != 0x2B { // check for - or + in the 7th spot
                return Err("7th char must be - or +")
            }

            let concated = [&chars[2..8], &chars[9..]].concat();

            match to_nums(&concated) {
                Err(reason) => Err(reason),
                Ok(result) => Ok(Parsed {
                    nums: result,
                    plus: chars[8] == 0x2D
                })
            }
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
