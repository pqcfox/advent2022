use clap::Parser;
use std::fs;

mod calorie_counting;
mod camp_cleanup;
mod rock_paper_scissors;
mod rucksack_reorganization;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    day: u8,
    input_file: String,
}

fn main() {
    let args = Args::parse();
    let run: fn(&str) = match args.day {
        1 => calorie_counting::run,
        2 => rock_paper_scissors::run,
        3 => rucksack_reorganization::run,
        4 => camp_cleanup::run,
        _ => panic!("No such day available"),
    };

    let input = fs::read_to_string(args.input_file).expect("Unable to read input file");

    run(&input);
}
