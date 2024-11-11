use std::process::exit;

use chrono::{DateTime, Local, Utc};
use clap::Parser;
use utils::{parse_datetime, parse_duration};
mod utils;

/// A tool to add or subtract specific durations (weeks, days, hours, minutes, seconds) from a given datetime.
#[derive(Parser, Debug)]
#[command(
    version,
    about = "A utility for performing datetime operations with specific durations.",
    long_about = "A tool to add or subtract specific durations (weeks, days, hours, minutes, seconds) from a given datetime."
)]
struct Args {
    /// The base datetime to operate on. Accepts the following formats:
    ///   - "%Y-%m-%d %H:%M:%S" (e.g., "2023-11-10 12:34:56")
    ///   - "%Y-%m-%dT%H:%M:%S" (e.g., "2023-11-10T12:34:56")
    ///   - "%Y-%m-%d" (e.g., "2023-11-10")
    ///   - "%H:%M:%S" (e.g., "12:34:56")
    ///   - "%H:%M" (e.g., "12:34")
    ///   - "%Y-%m-%d %H:%M:%S %z" (e.g., "2023-11-10 12:34:56 +0000")
    ///   - "%Y-%m-%dT%H:%M:%S%z" (e.g., "2023-11-10T12:34:56+0000")
    ///   - "%Y-%m-%dT%H:%M:%S%.f%:z" (e.g., "2023-11-10T12:34:56.123+00:00")
    ///
    /// If only a time is given, the date defaults to today's UTC date.
    /// If only a date is given, the time defaults to 00:00:00.
    #[clap(verbatim_doc_comment)]
    datetime: Option<String>,

    /// Operation to perform: '+' to add, '-' to subtract
    #[clap(verbatim_doc_comment)]
    op: Option<String>,

    /// Duration to add or subtract from the datetime.
    /// Format as a number followed by a unit, e.g., "3d" for 3 days or "2h" for 2 hours.
    /// Supported units:
    /// - 'w' for weeks
    /// - 'd' for days
    /// - 'h' for hours
    /// - 'm' for minutes
    /// - 's' for seconds
    ///
    /// Examples:
    /// - "5d" for 5 days
    /// - "2h" for 2 hours
    /// - "1w" for 1 week (equivalent to 7 days)
    #[clap(verbatim_doc_comment)]
    duration: Option<String>,

    /// Enable verbose output for more detailed information
    #[arg(long, short, default_value_t = false, verbatim_doc_comment)]
    verbose: bool,

    /// Format string for datetime output
    #[arg(long, short, verbatim_doc_comment)]
    format: Option<String>,
}

fn main() {
    let args = Args::parse();

    let output_format = args.format;

    let operand = match &args.datetime {
        Some(val) => match parse_datetime(val) {
            Ok(res) => res,
            Err(_) => output_err("Could not parse datetime"),
        },
        _ => Utc::now(),
    };

    let op = match &args.op {
        Some(val) if val == "+" || val == "-" => val,
        Some(_) => output_err("Invalid operator"),
        _ => "none",
    };

    if op == "none" {
        output(operand, output_format);
    }

    let duration = match &args.duration {
        Some(val) => match parse_duration(val) {
            Ok(res) => res,
            Err(_) => output_err("Could not parse duration"),
        },
        _ => output_err("Found an operator but no duration was given"),
    };

    let result = match op {
        "+" => operand + duration,
        "-" => operand - duration,
        _ => output_err("Invalid operator"),
    };

    output(result, output_format);
}

fn output(datetime: DateTime<Utc>, fmt: Option<String>) -> ! {
    let local = Local::now().timezone();
    let tz_datetime = datetime.with_timezone(&local);

    match fmt {
        Some(s) => println!("{}", tz_datetime.format(&s)),
        None => println!("{tz_datetime}"),
    }
    exit(0)
}

fn output_err(err: &str) -> ! {
    eprintln!("{err}");
    exit(1);
}
