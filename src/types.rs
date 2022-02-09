use chrono::prelude::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author("Djoudi BENARFA"), version, about, long_about = None)]
pub struct Args {
    /// The month
    #[clap(short, long, default_value_t = Local::today().month())]
    pub month: u32,
    /// The year
    #[clap(short, long, default_value_t = Local::today().year())]
    pub year: i32,

    /// Select a specific day in the calendar
    #[clap(short, long, default_value_t = Local::today().day(), conflicts_with("noselect"))]
    pub day: u32,
    /// Deactivate the highlighting of days in the calendar
    #[clap(short, long, conflicts_with("day"))]
    pub noselect: bool,
}
