use std::process::exit;

use chrono::prelude::*;
use chrono::Duration;
use clap::Parser;
use colored::*;

/// The cal utility displays a simple calendar in traditional format
#[derive(Parser, Debug)]
#[clap(author("Djoudi BENARFA"), version, about, long_about = None)]
struct Args {
    /// The month
    #[clap(short, long, default_value_t = Local::today().month())]
    month: u32,
    /// The year
    #[clap(short, long, default_value_t = Local::today().year())]
    year: i32,
    /// Select a specific day in the calendar
    #[clap(short, long, default_value_t = Local::today().day())]
    select: u32,
    /// Deactivate the highlighting of days in the calendar
    #[clap(short, long)]
    noselect: bool,
}
fn main() {
    // Command line option parsing
    let args = Args::parse();
    // The current date and time
    let current;
    // let current_month: u32;
    if !(1..13).contains(&args.month) {
        eprintln!(
            "{}: {}",
            "Error".yellow(),
            "Month must be between 1 an 12".red()
        );
        exit(0);
    }

    if args.year < 1900 || args.year > 96363 {
        eprintln!(
            "{}: {}",
            "Error".yellow(),
            "Year must be between 1900 and 96363".red()
        );
        exit(0);
    }

    current = Local.ymd(args.year, args.month, 1);

    // The first day of the month
    let mut cal = Local.ymd(args.year, args.month, 1);

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
        if cal.weekday().num_days_from_sunday() == 5 || cal.weekday().num_days_from_sunday() == 6 {
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
        // If we are at the current day end/or user selected day, we highlight it with a blue bg.
        if !args.noselect && (args.select == cal.day() || cal == Local::today()) {
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
    // Print a blank line between the calender and the terminal prompt
    println!("\n");
}
