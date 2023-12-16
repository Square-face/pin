use crate::utils::{ max_day, Pin, Parsed };





/// Check if the given month is possible
///
/// Month can't be over 12 and it can't be 0
///
/// # Arguments
/// * `month` the month to test
///
/// # Returns
/// Ture if the month is valid
/// False if it is not
fn check_month(month: u32) -> Result<(), &'static str> {

    if month == 0 {
        Err("Month is 0")

    } else if month > 12 {
        Err("Month is over 12")

    } else { // valid month
        Ok(())

    }
}








/// Check if the day is possible given the month and year
fn check_day(pin: Pin) -> Result<(), &'static str> {
    let day = pin.date.day;
    let max = max_day(pin.date.month, pin.date.year);

    if day > max {
        return if day > max+60 {
            Err("Day over maximum")

        } else if day < 61 {
            Err("Day to high for normal pin, to low for temporary")

        } else {
            Ok(())
        }
    }

    if day == 0 {
        return Err("Day is 0")

    }

    // permanent
    Ok(())
}




/// Check if the given pin has a valid date
///
/// 
/// Uses a few checks to verify that the given pin has a date that is possible to have.
/// i.e month can be 1 to 12
/// day can be from 0 to 28-31 depending on month or if it is a leap year.
///
/// # Arguments
/// * `pin` array of the digits in a pin.
///
/// # Returns
/// Ture if the date is valid 
/// False if it is not
fn date(pin: Pin) -> Result<(), &'static str>{

    match check_month(pin.date.month) {
        Err(reason) => return Err(reason),
        _ => {}
    }


    match check_day(pin) {
        Err(reason) => return Err(reason),
        _ => {}
    }

    Ok(())
}




/// Checks luhns algorithm on the given pin
///
/// # Arguments
/// * `pin` array of the digits in a pin.
///
/// # Returns
/// true if the sum is divisable by 10.
/// false if the sum is not divisable by 10.
fn luhns(pin:[u8;10]) -> bool {
    dbg!(pin);

    let multiples = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
    let mut sum = 0;

    for i in 0..5 {

        sum += multiples[pin[i*2] as usize];
        sum += pin[i*2+1];
    }

    sum % 10 == 0
}




/// Does all the existing checks on a pin and returns false if any of the checks returned false.
///
/// # Arguments
///
/// * `pin` array of the digits in a pin.
///
/// # Returns
///
/// Ok with no value if the pin passed all the tests.
/// Err with a string explaining witch test failed.
pub fn full(pin:Pin) -> Result<(), &'static str>{

    match date(pin) {
        Err(reason) => return Err(reason),
        _ => {}
    }

    if !luhns(pin.nums) {
        return Err("Failed luhns")
    }

    Ok(())
}
