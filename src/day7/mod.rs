pub fn run_day7(inputs: &String) {
    let day7_1 = day7_1(&inputs);
    println!("Day 7-1: {day7_1}");

    let day7_2 = day7_2(&inputs);
    println!("Day 7-2: {day7_2}");
}

pub fn day7_1(inputs: &String) -> usize {
    let mut positions: Vec<usize> = inputs.split(',').map(|x| x.parse().unwrap()).collect();
    positions.sort();

    let mid = positions[positions.len() / 2];
    return positions.iter().map(|x| x.abs_diff(mid)).sum();
}

pub fn day7_2(inputs: &String) -> usize {
    let mut positions: Vec<usize> = inputs.split(',').map(|x| x.parse().unwrap()).collect();
    positions.sort();
    let mut mode = positions[positions.len() / 2]; // == 367
    let mid = positions.len() / 2; // 500
    println!("mid: {mid}");

    let mut fuel: usize = positions.iter().map(|x| sum_fuel(x.abs_diff(mode))).sum();
    while mode <= mid {
        let new_fuel: usize = positions.iter().map(|x| sum_fuel(x.abs_diff(mode))).sum();
        if new_fuel < fuel {
            fuel = new_fuel;
        }
        mode += 1;
    }

    return fuel;
}

fn sum_fuel(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    return (n * (n - 1) / 2) + n;
}
