use clap::{Parser, ValueEnum};
use notify_rust::error::Error;
use notify_rust::{Hint, Notification, Timeout};
use std::env::args;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    duration: f32,

    #[clap(value_enum)]
    unit: Unit,
}

#[derive(Clone, Debug, ValueEnum)]
enum Unit {
    Seconds,
    Minutes,
    Hours,
}

fn main() -> Result<(), Error> {
    let duration = parse_args(args());

    println!("Hello, world!");
    println!("Waiting for {} seconds...", duration.as_secs());

    sleep(duration);

    println!("\nFinished!");

    Notification::new()
        .summary("Timer")
        .body("Your timer has finished!")
        .icon("firefox")
        .hint(Hint::Resident(true)) // does not work on kde
        .timeout(Timeout::Never) //
        .show()?;

    Ok(())
}

fn parse_args(args: std::env::Args) -> Duration {
    let args = Args::parse_from(args);

    match args.unit {
        Unit::Seconds => Duration::from_secs(args.duration.round() as u64),
        Unit::Minutes => Duration::from_secs((args.duration * 60.0) as u64),
        Unit::Hours => Duration::from_secs((args.duration * 3600.0) as u64),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert!(true)
    }
}
