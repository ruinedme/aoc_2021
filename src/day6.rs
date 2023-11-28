use timer::profile;

pub fn run_day6(inputs: &String) {
    profile! {
    let day6_1 = day6_1(&inputs);
    println!("Day 6-1: {day6_1}");
    }

    profile! {
    let day6_2 = day6_2(&inputs);
    println!("Day 6-2: {day6_2}");
    }
}

fn day6_1(inputs: &String) -> usize {
    let mut fish: Vec<i32> = inputs.split(',').map(|x| x.parse().unwrap()).collect();
    let max_days = 80;
    let mut day = 0;
    while day < max_days {
        //each day 0 becomes 6
        //append an 8 for each 0
        //decrement every OTHER number for the current day

        //Get count of all zeros, insert 8's into the vec to be appened at the end of fish
        let mut z: Vec<i32> = fish.iter().filter(|x| *x == &0).map(|_x| 8).collect();

        //decrement, or set 0 to 6
        fish = fish
            .iter()
            .map(|x| {
                if x > &0 {
                    return x - 1;
                } else {
                    return 6;
                }
            })
            .collect();

        //append 8 for each 0 that occurred this day
        fish.append(&mut z);
        day += 1;
    }
    return fish.len();
}

//Vecs/iterators are too slow, and large for large values of days
//Approach 1 (day6_1) is not scalable, adjusting to have a 9 length array to track the sum of each stage of fish.
//Add all stages at the end for correct count;
fn day6_2(inputs: &String) -> usize {
    let fish: Vec<usize> = inputs.split(',').map(|x| x.parse().unwrap()).collect();
    let mut fish_stages: [usize; 9] = [0; 9];
    // prime stages with initial state
    for f in fish {
        fish_stages[f] += 1;
    }

    let max_days = 256;
    let mut day = 0;
    while day < max_days {
        //each day 0 becomes 6
        //append an 8 for each 0
        //decrement every OTHER number for the current day
        let mut index = 0;
        //save sum of zeros for cycle
        let zeros = fish_stages[0];
        //"decrement" each other value by 1
        while index < 8 {
            fish_stages[index] = fish_stages[index + 1];
            index += 1;
        }
        //turn 0's to 6's
        fish_stages[6] += zeros;
        //add 8 for each zero
        fish_stages[8] = zeros;
        day += 1;
    }
    return fish_stages.iter().sum();
}
