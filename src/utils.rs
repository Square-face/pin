use chrono::prelude::*;


#[derive(Debug, Clone, Copy)]
pub struct Parsed {
    pub nums: [u8;10],
    pub plus: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Pin {
    pub nums: [u8;10],
    pub plus: bool,
    pub date: Date,
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
pub fn max_day(month:u32, year:i32) -> u32 {
    
    // 30 days in april, june, sep and nov
    if [4, 6, 9, 11].contains(&month) {
        return 30
    }
    
    // of remaining only feb doesn't have 31 days
    if month != 2 {
        return 31
    }

    // years divisable by 400 can't be leap a year
    if year % 100 == 0 {
        if year % 400 == 0 {
            return 29
        } else {
            return 28
        }
    }

    // if the year is divisable by 4 its a leap year
    if year % 4 == 0 {
        return 29
    }

    // all other years are not leap years
    28
}

fn get_year(syear: i32, month:u32, day:u32) -> i32 {

    // get current time
    let time = Utc::now();

    let mut centry: i32 = time.year() / 100; // OOxx
    let decade: i32 = time.year() % 100; // xxOO


    if syear > decade { // if the pins year has a decade higher than the current decade, it must be
                        // from the previous centry
        centry -= 1;
    }

    if syear == decade {
        if month > time.month() {
            centry -= 1;
        }
        if month == time.month() {
            if day > time.day() {
                centry -= 1;
            }
        }
    }

    let year = syear + centry*100;

    return year;
}

pub fn get_date(nums: [u8;10], plus: bool, centry: Option<i32>) -> Date {

    let decade  = (nums[0]*10 + nums[1]) as i32; // OOxxxx-xxxx
    let month   = (nums[2]*10 + nums[3]) as u32; // xxOOxx-xxxx
    let day     = (nums[4]*10 + nums[5]) as u32; // xxxxOO-xxxx

    let year = if centry.is_some() {
        centry.unwrap()*100 + decade
    } else {
        get_year(decade, month, day) - 100*(plus as i32)
    };

    Date {
        year,
        month,
        day,
    }
}
