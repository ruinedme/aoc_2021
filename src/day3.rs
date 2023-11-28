use timer::profile;

pub fn run_day3(inputs: &String) {
    profile! {
    let day3_1 = day3_1(&inputs);
    println!("Day 3-1: {day3_1}");
    }

    profile! {
    let day3_2 = day3_2(&inputs);
    println!("Day 3-1: {day3_2}");
    }
}

fn day3_1(inputs: &String) -> usize {
    let lines: Vec<&str> = inputs.lines().collect();

    let line_len = lines[0].len();
    let mut line_idx = 0;
    let mut gamma = String::with_capacity(line_len);

    while line_idx < line_len {
        let mut one_count = 0;
        let mut zero_count = 0;
        //line is double referenced? &&str
        for line in &lines {
            let mut chars = line.chars();
            match chars.nth(line_idx) {
                Some('0') => {
                    zero_count += 1;
                }
                Some('1') => {
                    one_count += 1;
                }
                Some(_) => unreachable!("found invalid char"),
                None => unreachable!("found invalid char"),
            }
        }
        if zero_count > one_count {
            gamma.push('0');
        } else {
            gamma.push('1');
        }
        line_idx += 1;
    }
    let gamma: u32 = u32::from_str_radix(&gamma, 2).unwrap();
    //This feels hacky
    //0 out left bits past line length
    let shift = 32 - line_len;
    let epsilon = !gamma << shift >> shift;

    return (gamma * epsilon) as usize;
}

#[derive(Debug)]
enum LifeSupportMode {
    Oxygen,
    CO2,
}

fn bit_scan<'a>(lines: &Vec<&'a str>, mode: &LifeSupportMode, line_idx: usize) -> usize {
    use LifeSupportMode::*;
    let mut result: usize = 0;
    if lines.len() > 1 {
        let line_len = lines[0].len();
        let mut one_count: Vec<&str> = Vec::with_capacity(lines.len() / 2);
        let mut zero_count: Vec<&str> = Vec::with_capacity(lines.len() / 2);

        while line_idx < line_len {
            for line in lines {
                let mut chars = line.chars();
                match chars.nth(line_idx) {
                    Some('0') => {
                        zero_count.push(line);
                    }
                    Some('1') => {
                        one_count.push(line);
                    }
                    Some(_) => unreachable!("found invalid char"),
                    None => unreachable!("found invalid char"),
                }
            }
            match &mode {
                Oxygen => {
                    if one_count.len() >= zero_count.len() {
                        if one_count.len() == 1 {
                            return usize::from_str_radix(one_count[0], 2).unwrap();
                        }
                        result = bit_scan(&one_count, mode, line_idx + 1);
                        break;
                    } else {
                        if zero_count.len() == 1 {
                            return usize::from_str_radix(zero_count[0], 2).unwrap();
                        }
                        result = bit_scan(&zero_count, mode, line_idx + 1);
                        break;
                    }
                }
                CO2 => {
                    if zero_count.len() <= one_count.len() {
                        if zero_count.len() == 1 {
                            return usize::from_str_radix(zero_count[0], 2).unwrap();
                        }
                        result = bit_scan(&zero_count, mode, line_idx + 1);
                        break;
                    } else {
                        if one_count.len() == 1 {
                            return usize::from_str_radix(one_count[0], 2).unwrap();
                        }
                        result = bit_scan(&one_count, mode, line_idx + 1);
                        break;
                    }
                }
            }
        }
    }

    return result;
}

fn day3_2(inputs: &String) -> usize {
    let lines: Vec<&str> = inputs.lines().collect();
    let oxygen = bit_scan(&lines, &LifeSupportMode::Oxygen, 0);
    let co2 = bit_scan(&lines, &LifeSupportMode::CO2, 0);
    return oxygen * co2;
}
