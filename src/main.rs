use std::time::Duration;
use clap::{Arg, App, AppSettings, Values};
use ansi_term::Color::{Red, Green, Cyan, Yellow};

macro_rules! error {
    ($($x:expr),+) => {
        {
            let err = format!($($x),+);
            println!("{} {}", Red.bold().paint("error:"), err);
            println!("\nFor more information try {}", Green.paint("--help"));
            std::process::exit(2);
        }
    }
}

#[inline]
fn parse_duration<'a>(arguments: Option<Values<'a>>) -> Duration {
    let mut duration = Duration::new(0, 0);

    for arg in arguments.unwrap() {
        let (value, unit) = if let Some(index) = arg.find(char::is_alphabetic) {
            (&arg[0..index], &arg[index..])
        } else {
            (arg, "s")
        };

        let number = if let Ok(num) = value.parse::<f64>() {
            num
        } else {
            error!("{} is not a valid floating point number!", Cyan.paint(value));
        };

        duration += match unit {
            "ms" => Duration::from_secs_f64(number / 1000.0),
            "s" => Duration::from_secs_f64(number),
            "m" => Duration::from_secs_f64(number * 60.0),
            "h" => Duration::from_secs_f64(number * 60.0 * 60.0),
            "d" => Duration::from_secs_f64(number * 60.0 * 60.0 * 24.0),
            _ => error!("{} is not a valid unit!", Cyan.paint(unit)),
        };
    }

    duration
}

fn main() {
    let matches = App::new("snore")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("\nRuns for the given time and is similar to the Unix `sleep` command.")
        .after_help(format!(
            "{examples}
    Run {snore} for one second:
        snore 1

    Run {snore} for 200 milliseconds:
        snore 200ms

    Run {snore} for 1 hour and 20 minutes:
        snore 1h 20m

    Run {snore} for 2 seconds and 500 milliseconds:
        snore 2s 500ms

    Run {snore} for one and a half seconds:
        snore 1.5s

    Run {snore} for 0.001 days:
        snore 0.001d
",
            examples = Yellow.paint("EXAMPLES:"),
            snore = Green.paint("snore"),
        ).as_str())
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .global_setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("NUMBER[UNIT]")
                .help("The time to wait before the command exits
Available units are:
  ms    milliseconds
  s     seconds
  m     minutes
  h     hours
  d     days

Multiple times get summed up, so
  snore 1d 10m 4s

will result in the command running for
1 day, 10 minutes, and 4 seconds")
                .required(true)
                .multiple(true)
                .min_values(1)
        )
        .get_matches();

    let duration = parse_duration(matches.values_of("NUMBER[UNIT]"));

    std::thread::sleep(duration);
}
