use grid::{get_all_neighbors, to_map};
use std::collections::HashSet;
use timer::profile;

pub fn run_day11(inputs: &String) {
    profile! {
    let day11_1 = day11_1(&inputs);
    println!("Day 11-1: {day11_1}");
    }

    profile! {
    let day11_2 = day11_2(&inputs);
    println!("Day 11-2: {day11_2}");
    }
}

const STEPS: usize = 100;
fn day11_1(inputs: &String) -> usize {
    let mut input_map: Vec<Vec<u8>> = to_map(&inputs);
    let mut step = 0;
    let mut total_flashes = 0;

    while step < STEPS {
        let mut should_flash: HashSet<(usize, usize)> = HashSet::new();

        //increase each square by 1
        input_map = input_map
            .iter()
            .map(|y| y.iter().map(|x| x + 1).collect())
            .collect();

        // Get squares that should flash for this step;
        let mut row = 0;
        let mut col = 0;
        let max_row = input_map.len();
        let max_col = input_map[0].len();
        //Can this be done with an iterator?
        while row < max_row {
            while col < max_col {
                if input_map[row][col] > 9 {
                    if should_flash.insert((row, col)) {
                        flash_square((row, col), &mut input_map, &mut should_flash);
                    }
                }
                col += 1;
            }
            row += 1;
            col = 0;
        }
        total_flashes += should_flash.len();
        //reset all flashed squares to 0
        input_map = input_map
            .iter()
            .map(|y| {
                y.iter()
                    .map(|x| {
                        if *x > 9 {
                            return 0;
                        } else {
                            return *x;
                        }
                    })
                    .collect()
            })
            .collect();
        step += 1;
    }

    return total_flashes;
}

fn day11_2(inputs: &String) -> usize {
    let mut input_map: Vec<Vec<u8>> = to_map(&inputs);
    let total_squares = input_map.len() * input_map[0].len();
    let mut step = 0;

    loop {
        let mut should_flash: HashSet<(usize, usize)> = HashSet::new();

        //increase each square by 1
        input_map = input_map
            .iter()
            .map(|y| y.iter().map(|x| x + 1).collect())
            .collect();

        // Get squares that should flash for this step;
        let mut row = 0;
        let mut col = 0;
        let max_row = input_map.len();
        let max_col = input_map[0].len();
        //Can this be done with an iterator?
        while row < max_row {
            while col < max_col {
                if input_map[row][col] > 9 {
                    if should_flash.insert((row, col)) {
                        flash_square((row, col), &mut input_map, &mut should_flash);
                    }
                }
                col += 1;
            }
            row += 1;
            col = 0;
        }

        if should_flash.len() == total_squares {
            return step + 1;
        }
        //reset all flashed squares to 0
        input_map = input_map
            .iter()
            .map(|y| {
                y.iter()
                    .map(|x| {
                        if *x > 9 {
                            return 0;
                        } else {
                            return *x;
                        }
                    })
                    .collect()
            })
            .collect();
        step += 1;
    }
}

fn flash_square<'a>(
    point: (usize, usize),
    map: &mut Vec<Vec<u8>>,
    checked_points: &'a mut HashSet<(usize, usize)>,
) {
    //incement neighbors
    let neighbors = get_all_neighbors((point.0, point.1), &map);
    neighbors
        .iter()
        .for_each(|point| map[point.0][point.1] += 1);

    //check for new squares to flash
    for n in neighbors {
        if map[n.0][n.1] > 9 {
            if checked_points.insert(n) {
                //insert new point
                flash_square(n, map, checked_points); //recurse
            }
        }
    }
}
