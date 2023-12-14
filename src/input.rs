pub fn parse(input: &String) -> Result<[u8;10], &'static str> {
    //! Parses a given input string into a array of the individual integers
    //!
    //! # Arguments
    //! * `input` The input string to parse
    //! 
    //! # Returns
    //! * `Result<[u8;10], &'static str>` Array of numbers or a message explaining why pin is
    //! invalid


    match input.len() {
        10 => {}

        11 => {
            let chars = input.as_bytes();
            
            if chars[7] != 0x2D || chars[7] != 0x2B { // check for - or + in the 7th spot
                return Err("7th char must be - or +")
            }

            
        }

        _ => {
            // Length is invalid
            
            if input.len() > 11 {
                return Err("Too long")
            }

            if input.len() < 10 {
                return Err("Too short")
            }
        }
    }

    return Ok([0;10]);
}
