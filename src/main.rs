use std::process::exit;

use chrono::prelude::*;
use clap::Parser;
use colored::*;
mod calendar;
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
    #[clap(short, long, default_value_t = Local::today().day(), conflicts_with("noselect"))]
    day: u32,
    /// Deactivate the highlighting of days in the calendar
    #[clap(short, long, conflicts_with("day"))]
    noselect: bool,
}

fn main() {
    // Command line option parsing
    let args = Args::parse();
    // Validate the month
    if !(1..13).contains(&args.month) {
        eprintln!(
            "{}: {}",
            "Error".yellow(),
            "Month must be between 1 and 12".red()
        );
        exit(0);
    }

    // Validate the year
    if args.year < 1900 || args.year > 96363 {
        eprintln!(
            "{}: {}",
            "Error".yellow(),
            "Year must be between 1900 and 96363".red()
        );
        exit(0);
    }
    calendar::print_calendar(args.year, args.month, args.day, args.noselect);
    // Print a blank line between the calender and the terminal prompt
    println!("\n");
}
