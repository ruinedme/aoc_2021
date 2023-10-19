pub fn run_day17(inputs: &String) {
    let day17_1 = day17_1(&inputs);
    println!("Day 17-1: {day17_1}");

    let day17_2 = day17_2(&inputs);
    println!("Day 17-2: {day17_2}");
}

fn day17_1(inputs: &String) -> usize {
    let target_range = TargetRange::parse(inputs);
    println!("{:?}", target_range);

    let mut max_y_pos = 0;
    for i in 0..=target_range.maxy_velocity {
        max_y_pos += i;
    }

    return max_y_pos as usize;
}

fn day17_2(inputs: &String) -> usize {
    let target_range = TargetRange::parse(inputs);

    return target_range.get_starting_velocity_count();
}

#[allow(dead_code)]
#[derive(Debug)]
struct TargetRange {
    min_x: isize,
    max_x: isize,
    minx_velocity: isize,
    min_y: isize,
    max_y: isize,
    maxy_velocity: isize,
}

impl TargetRange {
    //Input formatting:
    //target area: x=20..30, y=-10..-5
    fn parse(inputs: &String) -> Self {
        let x: Vec<&str> = inputs.split("x=").collect::<Vec<&str>>()[1]
            .split(", ")
            .collect::<Vec<&str>>()[0]
            .split("..")
            .collect();
        let y: Vec<&str> = inputs.split("y=").collect::<Vec<&str>>()[1]
            .split("..")
            .collect();

        let min_x: isize = x[0].parse().unwrap();
        //drag is uniform on x axis, so we can precalculate the minimum
        //required x velocity to at least reach the target regardless of y velocity
        let mut minx_velocity = 0;
        for i in 0..=min_x {
            minx_velocity += i;
            if minx_velocity >= min_x {
                minx_velocity = i;
                break;
            }
        }

        let min_y: isize = y[0].parse().unwrap();
        //This just happens to work for the example as well as my own input
        //Unsure if it works for other values
        let maxy_velocity = min_y.abs() - 1;

        return TargetRange {
            min_x,
            max_x: x[1].parse().unwrap(),
            minx_velocity,
            min_y,
            max_y: y[1].parse().unwrap(),
            maxy_velocity,
        };
    }

    //calculate the number of distinct starting velocities that will land in the target zone
    fn get_starting_velocity_count(&self) -> usize {
        let mut ret_value = 0;

        let mut starting_x_velocity = self.minx_velocity; //increment x velocity
        let mut starting_y_velocity = self.maxy_velocity; //decrement y velocity

        //change to max_x after testing
        while starting_x_velocity <= self.max_x {
            while starting_y_velocity >= self.min_y {
                //perform steps to find if given velocity will land in target
                let mut current_x_velocity = starting_x_velocity;
                let mut current_y_velocity = starting_y_velocity;
                let mut x_pos = 0;
                let mut y_pos = 0;

                while x_pos < self.max_x && y_pos > self.min_y {
                    x_pos += current_x_velocity;
                    y_pos += current_y_velocity;

                    //apply drag
                    if current_x_velocity > 0 {
                        current_x_velocity -= 1;
                    }
                    //apply gravity
                    current_y_velocity -= 1;

                    //check if we're in target
                    if x_pos >= self.min_x
                        && x_pos <= self.max_x
                        && y_pos <= self.max_y
                        && y_pos >= self.min_y
                    {
                        ret_value += 1;
                        //some velocities land in the target twice. Only count the first
                        break;
                    }
                }
                starting_y_velocity -= 1;
            }
            //reset for next iteration
            starting_y_velocity = self.maxy_velocity;
            starting_x_velocity += 1;
        }
        return ret_value;
    }
}
