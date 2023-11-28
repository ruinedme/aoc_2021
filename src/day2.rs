use timer::profile;

pub fn run_day2(inputs: &String) {
    profile! {
    let day2_1 = day2_1(&inputs);
    println!("Day 2-1: {day2_1}");
    }

    profile! {
    let day2_2 = day2_2(&inputs);
    println!("Day 2-2: {day2_2}");
    }
}

enum SubDirection {
    Forward,
    Down,
    Up,
}

impl SubDirection {
    fn to(input: &str) -> Self {
        match input {
            "forward" => SubDirection::Forward,
            "down" => SubDirection::Down,
            "up" => SubDirection::Up,
            _ => panic!("Invalid Direction: {}", input),
        }
    }
}

fn day2_1(inputs: &String) -> usize {
    use SubDirection::{Down, Forward, Up};
    let lines: Vec<&str> = inputs.lines().collect();
    let mut h_pos: usize = 0;
    let mut depth: usize = 0;
    for line in lines {
        let s: Vec<&str> = line.split(" ").collect();
        let dir = SubDirection::to(s[0]);
        let amt: usize = s[1].parse().unwrap();
        match dir {
            Forward => {
                h_pos += amt;
            }
            Down => {
                depth += amt;
            }
            Up => {
                depth -= amt;
            }
        }
    }
    return h_pos * depth;
}

fn day2_2(inputs: &String) -> usize {
    use SubDirection::{Down, Forward, Up};
    let lines: Vec<&str> = inputs.lines().collect();
    let mut h_pos: usize = 0;
    let mut depth: usize = 0;
    let mut aim: usize = 0;
    for line in lines {
        let s: Vec<&str> = line.split(" ").collect();
        let dir = SubDirection::to(s[0]);
        let amt: usize = s[1].parse().unwrap();
        match dir {
            Forward => {
                h_pos += amt;
                depth += aim * amt;
            }
            Down => {
                aim += amt;
            }
            Up => {
                aim -= amt;
            }
        }
    }
    return h_pos * depth;
}
