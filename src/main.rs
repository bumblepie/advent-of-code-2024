mod day_1;
mod day_2;
mod day_3;
mod day_4;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    day: u8,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    part: u8,
}
fn main() {
    let args = Args::parse();
    match args.day {
        1 => match args.part {
            1 => day_1::part_1("inputs/day-1-input.txt"),
            2 => day_1::part_2("inputs/day-1-input.txt"),
            _ => panic!("Unknown part: {}", args.part),
        },
        2 => match args.part {
            1 => day_2::part_1("inputs/day-2-input.txt"),
            2 => day_2::part_2("inputs/day-2-input.txt"),
            _ => panic!("Unknown part: {}", args.part),
        }
        3 => match args.part {
            1 => day_3::part_1("inputs/day-3-input.txt"),
            2 => day_3::part_2("inputs/day-3-input.txt"),
            _ => panic!("Unknown part: {}", args.part),
        }
        4 => match args.part {
            1 => day_4::part_1("inputs/day-4-input.txt"),
            2 => day_4::part_2("inputs/day-4-input.txt"),
            _ => panic!("Unknown part: {}", args.part),
        }
        _ => panic!("Unknown day: {}", args.day),
    }
}
