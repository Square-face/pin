
#[cfg(test)]
mod tests {
    use crate::input::Parsed;
    use std::fs::File;
    use std::io::{ BufReader, BufRead };
    use crate::{ input, check };
    

    fn full_test(input: &str, validity: bool, parsed: [u8;10], output: bool) {
        //! Test all functions for validating a pin number using knowns outputs
        //!
        //! The input string is given to the parser and the result is compared to the given
        //! validity and parse result.
        //!
        //! If the function is expected to return a parsed array it will be compared to `parsed`.
        //! If the function is expected to return an Err then the test will fail if a parsed list
        //! is returned.
        //!
        //! The `parsed` is then given to [check::full] and the result is compared with `output`
        //!
        //! # Arguments
        //! * `input` a example user input string to run the validity and parser checks on
        //! * `validity` if the output from [input::parse] should be Ok or Err
        //! * `parsed` expected output from [input::parse] if it is Ok. Ignored if `validity` is
        //! set to false.
        //! * `output` expected output from [check::full]. Ignored if `validity` is set to false
        //!


        if validity {
            // Compare output to expected
            match input::parse(&input.to_string()) {
                Err(reason) => panic!("{} failed to parse, {}", input, reason),
                Ok(result) => {
                    assert_eq!(result.nums, parsed)
                }
            }

        } else {
            // Since the expected parsing result is that the input is invalid
            // we will expect an error and panic otherwise

            let parsed = input::parse(&input.to_string()); // result from function call

            // fail test if the function does not return an error
            parsed.expect_err(format!("{} parsed wich shouldn't be possible", input).as_str());
        }

        // check the parsed input and compare result with `output`
        assert_eq!(check::full(Parsed{
            nums: parsed,
            plus: false,
        }).is_ok(), output, "{} got unexpected check result", input);
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
                    let parsed = input::parse(&pin).expect(format!("{} failed with invalid format", pin).as_str());
                    assert_eq!(check::full(parsed).is_ok(), true, "{} failed check", pin);
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
