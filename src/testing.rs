
#[cfg(test)]
mod tests {
    use std::fs::File;
    use glob::glob;
    use std::io::{ BufReader, BufRead };
    use crate::{ input, check };
    
    /// Test all functions for validating a pin number using knowns outputs
    ///
    /// The input string is given to the parser and the result is compared to the given
    /// validity and parse result.
    ///
    /// If the function is expected to return a parsed array it will be compared to `parsed`.
    /// If the function is expected to return an Err then the test will fail if a parsed list
    /// is returned.
    ///
    /// The `parsed` is then given to [check::full] and the result is compared with `output`
    ///
    /// # Arguments
    /// * `input` a example user input string to run the validity and parser checks on
    /// * `validity` if the output from [input::parse] should be Ok or Err
    /// * `parsed` expected output from [input::parse] if it is Ok. Ignored if `validity` is
    /// set to false.
    /// * `output` expected output from [check::full]. Ignored if `validity` is set to false
    ///
    #[allow(dead_code)]
    fn full_test(input: &str, validity: bool, parsed: [u8;10], output: bool) {

        let actual_parsed = input::parse(&input.to_string()); // result from function call

        if !validity {
            // Since the expected parsing result is that the input is invalid
            // we will expect an error and panic otherwise

            // fail test if the function does not return an error
            actual_parsed.expect_err(format!("{} parsed wich shouldn't be possible", input).as_str());
            return;
        }

        match actual_parsed {
            Err(reason) => panic!("{} failed to parse, {}", input, reason),
            Ok(result) => {
                assert_eq!(result.nums, parsed)
            }
        }

        // check the parsed input and compare result with `output`
        assert_eq!(
            check::full(
                actual_parsed.unwrap()
            ).is_ok(),
            output,
            "{} got unexpected check result", input);
    }



    #[test]
    fn skatteverket() {
        //! run tests on files provided by skatteverket where all pins are known to be valid
        //! 
        //! The files must be formated with each line being a single pin

        let path = "src/tests/skatteverket/*.txt";

        for entry in glob(path).expect("Failed to get files") {
            if let Ok(path) = entry {

                let file = File::open(path.clone()).expect(format!("{} not found", path.display()).as_str());
                let reader = BufReader::new(file);
                
                // parse file contents
                for line in reader.lines() {
                    match line {
                        Err(_) => {}
                        Ok(pin) => {
                            let parsed = input::parse(&pin).expect(format!("{} failed with invalid format", pin).as_str());
                            assert_eq!(check::full(parsed).is_ok(), true, "{} failed check", pin);
                        }
                    }
                }
            }
        }
    }


    #[test]
    fn file_mixed() {
        //! run tests on a file with both valid and invalid pins, and compares its own checks with
        //! the expected result provided by the file.
        //! 
        //! The file must be formated with each line being a single pin
        //! The line always starts with "v " or "i " before any input
        //!
        //! If the line starts with "v " it means the pin is valid (v for valid)
        //! If the line starts with "i " it means the pin is invalid (i for invalid)
        //!

        let path = "src/tests/mixed.txt";
        let file = File::open(path).expect(format!("{} not found", path).as_str());
        let reader = BufReader::new(file);
        
        // parse file contents
        for line in reader.lines() {
            match line {
                Err(_) => {}
                Ok(pin) => {
                    // split line by spaces backwards
                    let split = pin.split_once("  ");

                    if split.is_none() {
                        continue;
                    }


                    let (pin, expected) = split.unwrap();

                    
                    match input::parse(&pin.to_string()){
                        Err(reason) => {
                            if expected == "!" {
                                println!("{} failed to parse - {}", pin, reason);
                            } else {
                                panic!("{} failed to parse - {}", pin, reason);
                            }
                        }
                        Ok(parsed) => {
                            match check::full(parsed) {
                                Err(reason) => {
                                    if expected == "!" {
                                        println!("{} failed check - {}", pin, reason);
                                    } else {
                                        panic!("{} failed check - {}", pin, reason);
                                    }
                                },
                                Ok(()) => {
                                    if expected == "Y" {
                                        println!("{} succeded", pin);
                                    } else {
                                        panic!("{} should not be valid", pin);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
