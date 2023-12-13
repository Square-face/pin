
#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{ BufReader, BufRead };
    use crate::{ input, check };
    

    fn full_test(input: &str, validity: bool, parsed: [u8;10], output: bool) {
        //! Test all functions for validating a pin number using knowns outputs
        //!
        //! The input string is given to the validator and the result is compared to the given
        //! validity.
        //!
        //! If the validity is expected to be false, the function will stop after the validity
        //! check, otherwise it will continue
        //!
        //! The input is then given to [input::parse] and compared to [parsed]
        //!
        //! The parsed input is then given to [check:full] and the result is compared with [output]
        //!
        //! # Arguments
        //! * `input` a example user input string to run the validity and parser checks on
        //! * `validity` expected output from [input::validate]
        //! * `parsed` expected output from [input::parse]
        //! * `output` expected output from [check:full]
        //!


        assert_eq!(input::validate(&input.to_string()), validity, "{} failed input validation", input);

        if !validity {return}

        assert_eq!(input::parse(&input.to_string()), parsed, "{} failed parsing", input);
        assert_eq!(check::full(parsed), output, "{} failed check", input);
    }

    #[test]
    fn known() {
        let output = true;
        let parsed = [0,6,1,0,0,9,2,4,5,4];

        full_test("0610092454", output, parsed, output);
        full_test("061009-2454", output, parsed, output);
        full_test("200610092454", output, parsed, output);
        full_test("20061009-2454", output, parsed, output);


        let output = true;
        let parsed = [0,6,1,1,0,1,0,5,9,6];

        full_test("0611010596", output, parsed, output);
        full_test("061101-0596", output, parsed, output);
        full_test("200611010596", output, parsed, output);
        full_test("20061101-0596", output, parsed, output);
    }


    #[test]
    fn file_all_valid() {
        //! run tests on a file where all pins are known to be valid
        //! 
        //! The file must be formated with each line being a single pin
        //!

        let path = "src/tests/valid.txt";
        let file = File::open(path).expect(format!("{} not found", path).as_str());
        let reader = BufReader::new(file);
        
        // parse file contents
        for line in reader.lines() {
            match line {
                Err(_) => {}
                Ok(pin) => {
                    assert_eq!(input::validate(&pin), true, "{} failed validation", pin);

                    let parsed = input::parse(&pin);

                    assert_eq!(check::full(parsed), true, "{} failed check", pin);
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

        let file = File::open(path).expect(format!("{} does not exist", path).as_str());
        let mut reader = BufReader::new(file);
        
        for line in reader.lines() {
            match line {
                Err(_) => {}
                Ok(contents) => {
                    dbg!(contents);
                }
            }
        }
    }
}
