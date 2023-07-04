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

    //Answer is 489, not sure if there is a better way to determine the preciese answer.
    let mode = positions[positions.len() / 2]; // 367
    let mut mid = positions.len() / 2; // 500

    let mut fuel: usize = positions.iter().map(|x| sum_fuel(x.abs_diff(mid))).sum();
    //Go backwards from mid to mode just to save on iterations.
    //Sample input answer was exactly mid, puzzle input was mid - 11
    while mid >= mode {
        mid -= 1;
        let new_fuel: usize = positions.iter().map(|x| sum_fuel(x.abs_diff(mid))).sum();
        //Fuel costs will go down on each iteration until the min is found, then cost starts increasing.
        //break as soon as new_fuel > fuel;
        if new_fuel < fuel {
            fuel = new_fuel;
            continue;
        }
        break;
    }

    return fuel;
}

//n is the number of steps taken
fn sum_fuel(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    return (n * (n - 1) / 2) + n;
}
