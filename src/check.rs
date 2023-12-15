
fn month(pin:[u8;10]) -> bool {
    //! Check if the given pin has a valid month
    //!
    //! Month can't be over 12 and it can't be 0
    //!
    //! # Arguments
    //! * `pin` array of the digits in a pin.
    //!
    //! # Returns
    //! Ture if the month is valid
    //! False if it is not

    let month = pin[2]*10 + pin[3];
    
    if month > 12 {
        return false
    }

    if month == 0 {
        return false;
    }

    return true;
}

fn day(pin:[u8;10]) -> bool {
    //! Check if the given pin has a valid day
    //!
    //! Day can't be 0
    //! Day can't be more than 30 if the month is 4, 6, 9, 11
    //!
    //! If the month is 2, the day can't be over 28
    //! unless the year is devisable by 4 and not divisable by 400, in those cases it could be up
    //! to 29
    //!
    //! All other cases the max day is 31
    //!
    //! # Arguments
    //! * `pin` array of the digits in a pin.
    //!
    //! # Returns
    //! Ture if the day is valid 
    //! False if it is not

    let year    = pin[0]*10 + pin[1];
    let month   = pin[2]*10 + pin[3];
    let day     = pin[4]*10 + pin[5];

    if day == 0 {
        return false;
    }

    let max = match month {
        2 => {
            if year % 400 == 0 { // if year is divisable by 400, febraury will always have 28 days
                28
            } else if year % 4 == 0 { // if year is divisable by 4 (and not 400) february has 29
                                      // days (leap years)
                29
            } else { // in any other case february has 28 days
                28
            }
        },
        4 | 6 | 9 | 11 => 30,
        _ => 31
    };

    if day > max { return false }
    return true;

}


fn luhns(pin:[u8;10]) -> bool {
    //! Checks luhns algorithm on the given pin
    //!
    //! # Arguments
    //! * `pin` array of the digits in a pin.
    //!
    //! # Returns
    //! true if the sum is divisable by 10.
    //! false if the sum is not divisable by 10.

    let multiples = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
    let mut sum = 0;

    for i in 0..5 {

        sum += multiples[pin[i*2] as usize];
        sum += pin[i*2+1];
    }

    sum % 10 == 0
}

pub fn full(pin:[u8;10]) -> Result<(), &'static str>{
    //! Does all the existing checks on a pin and returns false if any of the checks returned
    //! false.
    //!
    //! # Arguments
    //! * `pin` array of the digits in a pin.
    //!
    //! # Returns
    //! Ok with no value if the pin passed all the tests.
    //! Err with a string explaining witch test failed.

    if !month(pin) {
        return Err("Month outside range")
    }

    if !luhns(pin) {
        return Err("Failed luhns")
    }

    Ok(())
}
