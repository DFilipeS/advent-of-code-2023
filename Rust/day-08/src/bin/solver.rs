use clap::Parser;
use day_08::part1;
use day_08::part2;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, short = 'p', help = "Problem part number (1 or 2)")]
    part: u8,
}

fn main() {
    let process = match Cli::parse() {
        Cli { part: 1 } => part1::process,
        Cli { part: 2 } => part2::process,
        _ => panic!("Invalid part number"),
    };

    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}
