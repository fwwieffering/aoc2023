use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "aoc2023")]
#[command(author = "Freddy Wieffering")]
#[command(version = "1.0")]
#[command(about = "executes advent of code days", long_about = None)]
struct Args {
    // Day of code to execute
    day: String
}

pub mod days;

fn load_day(day: &str) -> String {
    std::fs::read_to_string(format!("src/input/{day}")).unwrap()
}

fn main() {
    let args = Args::parse();
    let input = &load_day(&args.day);

    match args.day.as_str() {
        "one" => {
            println!("part 1: {}", crate::days::day1::part1(input));
            println!("part 2: {}", crate::days::day1::part2(input));

        },

        _ => {
            panic!("invalid or not-yet-implemented day {}", args.day)
        }
    }

}
