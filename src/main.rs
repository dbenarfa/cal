use clap::Parser;
mod calendar;
mod types;
/// The cal utility displays a simple calendar in traditional format

fn main() {
    // Command line option parsing
    let args = types::Args::parse();
    // Validate the month

    match calendar::print_calendar(args.year, args.month, args.day, args.noselect) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
    // Print a blank line between the calender and the terminal prompt
    println!("\n");
}
