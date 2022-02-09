use std::process::exit;

use chrono::prelude::*;
use chrono::Duration;

use colored::*;

//
// Print the calendar in the terminal
//
pub fn print_calendar(year: i32, month: u32, day: u32, highlight: bool) {
    let current = Local.ymd(year, month, 1);

    // The first day of the month
    let mut cal = Local.ymd(year, month, 1);

    // Validate the day
    if cal.with_day(day) == None {
        let formatted_err = format!(
            "Day must be between 1 and {}",
            (NaiveDate::from_ymd(year, month + 1, 1) - Duration::days(1))
                .day()
                .to_string()
                .red()
        )
        .red();
        eprintln!("{}: {}", "Error".yellow(), formatted_err);
        exit(0);
    }
    println!("");

    let formatted_date = current.format("%B %Y");

    // Get the number of the day starting from sunday.
    let mut counter = cal.weekday().num_days_from_sunday();

    let position = counter as usize * 3;
    // Colors
    let mut r = 128;
    let mut g = 246;
    let mut b = 255;

    // Calendar header
    // Print the remaining days if we are still in the current month
    let su_sa = format!(
        "Su Mo Tu We Th {} {}",
        "Fr".truecolor(r, g, b),
        "Sa".truecolor(r, g, b),
    );

    // Print the current month and the year
    println!("{:^1$}", formatted_date, 20);

    // Print the calendar header
    print!("{}\n", su_sa);

    // Print the first day of the month
    // Position variable is used to print the first day of the month at the correct day name position
    // (Eg: 01/02/202 is a Tue)
    // Su Mo Tu We Th Fr Sa
    //       01 02 03 04 05
    if position > 0 {
        print!("{:width$}", " ", width = position);
    }
    while cal.month() == current.month() {
        // Position of the current day in the calendar
        counter += 1;

        // Colors for the weekends
        let num_day = cal.weekday().num_days_from_sunday();
        if num_day == 5 || num_day == 6 {
            // Weekend color
            r = 128;
            g = 246;
            b = 255;
        } else {
            // Default color
            r = 255;
            g = 255;
            b = 255;
        }
        // If we are at the current day end/or user requested to highlight a day (-d), we highlight it with a blue bg.
        // Or of the user requested not to highlight a day, then no day will be highlighted.
        if !highlight && (day == cal.day() || cal == Local::today()) {
            print!(
                "{:0>2} ",
                cal.day()
                    .to_string()
                    .truecolor(r, g, b)
                    .on_truecolor(0, 119, 194)
            );
        } else {
            print!("{:0>2} ", cal.day().to_string().truecolor(r, g, b));
        }
        // Check if we are at the end of the calendar column (Sa) we return to the next line
        if counter % 7 == 0 {
            println!("");
        }
        // Go to the next day
        cal = cal + Duration::days(1);
    }
}
