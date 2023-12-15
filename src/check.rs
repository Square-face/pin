
use crate::input::Parsed;




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
fn check_month(month: u8) -> Result<(), &'static str> {

    if month == 0 {
        Err("Month is 0")

    } else if month > 12 {
        Err("Month is over 12")

    } else { // valid month
        Ok(())

    }
}




/// Calculate the maximum possible day for a pin
///
/// Get the day count for the given month.
/// The months april, june, september and november (4, 6, 9, 11) have 30 days
/// All other months except february has 31 days
///
/// February has 28 days most years
/// If the year is divisable by 4, its a leap year meaning february has 29 days. But not if the
/// year is also divisable by 400, then its not longer a leap year
///
/// # Arguments
/// * `month` The month to get day count for
/// * `year` The year to use for leap year calculations
///
/// # returns
/// The number of days in that month
fn max_day(month:u8, year:u16) -> u8 {
    
    // 30 days in april, june, sep and nov
    if [4, 6, 9, 11].contains(&month) {
        return 30
    }
    
    // of remaining only feb doesn't have 31 days
    if month != 2 {
        return 31
    }

    // years divisable by 400 can't be leap a year
    if year % 400 == 0 {
        return 28
    }

    // if the year is divisable by 4 its a leap year
    if year % 4 == 0 {
        return 29
    }

    // all other years are not leap years
    28
}




/// Check if the day is possible given the month and year
fn check_day(day:u8, month:u8, year: u16) -> Result<(), &'static str> {
    let max = max_day(month, year);

    if day > max {
        return if day > max+60 {
            Err("Day over maximum")

        } else if day < 61 {
            Err("Day to high for normal pin, to low for temporary")

        } else {
            // temp number
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
fn date(pin:Parsed) -> Result<(), &'static str>{
    let nums = pin.nums;

    let year    = (nums[0]*10 + nums[1]) as u16 + 2000;
    let month   = nums[2]*10 + nums[3];
    let day     = nums[4]*10 + nums[5];


    match check_month(month) {
        Err(reason) => return Err(reason),
        _ => {}
    }


    match check_day(day, month, year as u16 + 2000) {
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
pub fn full(pin:Parsed) -> Result<(), &'static str>{

    match date(pin) {
        Err(reason) => return Err(reason),
        _ => {}
    }

    if !luhns(pin.nums) {
        return Err("Failed luhns")
    }

    Ok(())
}
