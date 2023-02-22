use clap::builder::PossibleValue;
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
    unit: Unit,
}

#[derive(Clone, Debug)]
enum Unit {
    Seconds,
    Minutes,
    Hours,
}

impl ValueEnum for Unit {
    fn value_variants<'a>() -> &'a [Self] {
        &[Unit::Seconds, Unit::Minutes, Unit::Hours]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        let value = match self {
            // These aliases are hidden. See https://github.com/clap-rs/clap/issues/4416.
            Self::Seconds => PossibleValue::new("seconds").alias("second").alias("s"),
            Self::Minutes => PossibleValue::new("minutes").alias("min").alias("m"),
            Self::Hours => PossibleValue::new("hours").alias("hour").alias("h"),
        };
        Some(value)
    }
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
