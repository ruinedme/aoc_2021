use grid::{get_cardinal_neighbors, to_map};
use std::collections::HashSet;
use timer::profile;

pub fn run_day9(inputs: &String) {
    profile! {
    let day9_1 = day9_1(&inputs);
    println!("Day 9-1: {day9_1}");
    }

    profile! {
    let day9_2 = day9_2(&inputs);
    println!("Day 9-2: {day9_2}");
    }
}

fn day9_1(inputs: &String) -> usize {
    let input_map: Vec<Vec<u8>> = to_map(inputs);

    let mut total = 0;
    for (i, row) in input_map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if is_lowpoint((i, j), &input_map) {
                total += *col as usize + 1;
            }
        }
    }
    return total;
}

fn day9_2(inputs: &String) -> usize {
    //parse lines into map
    let input_map: Vec<Vec<u8>> = to_map(inputs);
    let mut low_points: Vec<(usize, usize)> = Vec::new();

    //get low points
    for (i, row) in input_map.iter().enumerate() {
        for (j, _col) in row.iter().enumerate() {
            if is_lowpoint((i, j), &input_map) {
                low_points.push((i, j));
            }
        }
    }
    let mut basin_sizes: Vec<usize> = Vec::with_capacity(low_points.len());
    //find basin
    for p in low_points {
        //Some neighbors overlap utilize HashSet to only keep unique neighbor values
        let mut neighbors: HashSet<(usize, usize)> = HashSet::new();
        neighbors.insert(p);
        let size = map_basin(p, &input_map, &mut neighbors);
        basin_sizes.push(size);
    }
    basin_sizes.sort();
    let len = basin_sizes.len();
    return basin_sizes[len - 1] * basin_sizes[len - 2] * basin_sizes[len - 3];
}

fn is_lowpoint(point: (usize, usize), map: &Vec<Vec<u8>>) -> bool {
    let row_max = map.len() - 1;
    let col_max = map[0].len() - 1;
    //top
    if point.0 == 0 {
        let p = map[point.0][point.1];
        let n1 = map[point.0 + 1][point.1]; //down 1
                                            //left corner
        if point.1 == 0 {
            let n2 = map[point.0][point.1 + 1]; //right 1
            if p < n1 && p < n2 {
                return true;
            }
            return false;
        }
        //right corner
        if point.1 == col_max {
            let n2 = map[point.0][point.1 - 1]; //left 1
            if p < n1 && p < n2 {
                return true;
            }
            return false;
        }
        //rest
        let n2 = map[point.0][point.1 - 1]; //left 1
        let n3 = map[point.0][point.1 + 1]; //right 1
        if p < n1 && p < n2 && p < n3 {
            return true;
        }
        return false;
    }
    //bottom
    if point.0 == row_max {
        let p = map[point.0][point.1];
        let n1 = map[point.0 - 1][point.1]; //up 1
                                            //left corner
        if point.1 == 0 {
            let n2 = map[point.0][point.1 + 1]; //right 1
            if p < n1 && p < n2 {
                return true;
            }
            return false;
        }
        //right corner
        if point.1 == col_max {
            let n2 = map[point.0][point.1 - 1]; //left 1
            if p < n1 && p < n2 {
                return true;
            }
            return false;
        }
        //rest
        let n2 = map[point.0][point.1 - 1]; //left 1
        let n3 = map[point.0][point.1 + 1]; //right 1
        if p < n1 && p < n2 && p < n3 {
            return true;
        }
        return false;
    }
    //middle
    let p = map[point.0][point.1];
    let n1 = map[point.0 - 1][point.1]; //up 1
    let n2 = map[point.0 + 1][point.1]; //down 1
                                        //left edge
    if point.1 == 0 {
        let n3 = map[point.0][point.1 + 1]; //right 1
        if p < n1 && p < n2 && p < n3 {
            return true;
        }
        return false;
    }
    //right edge
    if point.1 == col_max {
        let n3 = map[point.0][point.1 - 1]; //left 1
        if p < n1 && p < n2 && p < n3 {
            return true;
        }
        return false;
    }
    //rest
    let n3 = map[point.0][point.1 - 1]; //left 1
    let n4 = map[point.0][point.1 + 1]; //right 1
    if p < n1 && p < n2 && p < n3 && p < n4 {
        return true;
    }
    return false;
}

fn map_basin(
    point: (usize, usize),
    map: &Vec<Vec<u8>>,
    neighbors: &mut HashSet<(usize, usize)>,
) -> usize {
    //get neighbors
    let mut n = get_cardinal_neighbors(point, &map);

    n = n
        .iter()
        .filter(|x| &map[x.0][x.1] < &9 && &map[x.0][x.1] > &0) //filter out invalid neighbors
        .map(|x| *x)
        .filter(|t| neighbors.insert(*t)) //filter out new unique neighbors
        .map(|t| t)
        .collect();
    //continue to recurse over new neighbors until none are left
    for i in n {
        map_basin(i, &map, neighbors);
    }

    return neighbors.len();
}
