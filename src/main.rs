use std::{env, fs};

fn main() {
    use aoc_2021::*;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing Input File");
        println!("Usage: {} [1-25] input_file", args[0]);
        return;
    }

    let day: &u8 = &args[1].parse().unwrap();
    if day < &1 || day > &25 {
        println!("Day must be between 1 and 25 inclusive");
        return;
    }
    //let inputs = fs::read(&args[1]).unwrap();
    let inputs = fs::read_to_string(&args[2]).unwrap();

    match day {
        1 => day1::run_day1(&inputs),
        2 => day2::run_day2(&inputs),
        3 => day3::run_day3(&inputs),
        4 => day4::run_day4(&inputs),
        5 => day5::run_day5(&inputs),
        6 => day6::run_day6(&inputs),
        7 => day7::run_day7(&inputs),
        8 => day8::run_day8(&inputs),
        8..=25 => println!("Not Implemented"),
        _ => panic!("Invalid Day"),
    }
}
