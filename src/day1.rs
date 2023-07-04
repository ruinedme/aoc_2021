pub fn run_day1(inputs: &String) {
    let day1_1 = day1_1(&inputs);
    println!("Day 1-1: {day1_1}");

    let day1_2 = day1_2(&inputs);
    println!("Day 1-2: {day1_2}");
}

fn day1_1(inputs: &String) -> usize {
    let lines = inputs.lines();
    let mut previous = "000";
    let mut times_increased: usize = 0;
    for line in lines {
        if line > previous {
            times_increased += 1;
        }
        previous = line;
    }

    return times_increased;
}

fn day1_2(inputs: &String) -> usize {
    let lines: Vec<u32> = inputs.lines().map(|x| x.parse().unwrap()).collect();
    let len = lines.len() - 3;
    let mut i = 0;
    let mut times_increased = 0;
    let mut previous = 0;
    while i < len {
        let a = lines[i] + lines[i + 1] + lines[i + 2];
        if a > previous {
            times_increased += 1;
        }
        previous = a;
        i += 1;
    }
    return times_increased;
}
