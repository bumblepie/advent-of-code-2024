mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;

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
        },
        5 => match args.part {
            1 => day_5::part_1("inputs/day-5-input.txt"),
            2 => day_5::part_2("inputs/day-5-input.txt"),
            _ => panic!("Unknown part: {}", args.part),
        },
        6 => match args.part {
            1 => day_6::part_1("inputs/day-6-input.txt"),
            2 => day_6::part_2("inputs/day-6-input.txt"),
            _ => panic!("Unknown part: {}", args.part),
        },
        7 => match args.part {
            1 => day_7::part_1("inputs/day-7-input.txt"),
            2 => day_7::part_2("inputs/day-7-input.txt"),
            _ => panic!("Unknown part: {}", args.part),
        },
        8 => match args.part {
            1 => day_8::part_1("inputs/day-8-input.txt"),
            2 => day_8::part_2("inputs/day-8-input.txt"),
            _ => panic!("Unknown part: {}", args.part),
        },
        9 => match args.part {
            1 => day_9::part_1("inputs/day-9-input.txt"),
            2 => day_9::part_2("inputs/day-9-input.txt"),
            _ => panic!("Unknown part: {}", args.part),
        },
        10 => match args.part {
            1 => day_10::part_1("inputs/day-10-input.txt"),
            2 => day_10::part_2("inputs/day-10-input.txt"),
            _ => panic!("Unknown part: {}", args.part),
        },
        11 => match args.part {
            1 => day_11::part_1("inputs/day-11-input.txt"),
            2 => day_11::part_2("inputs/day-11-input.txt"),
            _ => panic!("Unknown part: {}", args.part),
        },
        12 => match args.part {
            1 => day_12::part_1("inputs/day-12-input.txt"),
            2 => day_12::part_2("inputs/day-12-input.txt"),
            _ => panic!("Unknown part: {}", args.part),
        },
        13 => match args.part {
            1 => day_13::part_1("inputs/day-13-input.txt"),
            2 => day_13::part_2("inputs/day-13-input.txt"),
            _ => panic!("Unknown part: {}", args.part),
        }
        _ => panic!("Unknown day: {}", args.day),
    }
}
