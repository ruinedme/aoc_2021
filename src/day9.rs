use std::collections::HashSet;

pub fn run_day9(inputs: &String) {
    let day9_1 = day9_1(&inputs);
    println!("Day 9-1: {day9_1}");

    let day9_2 = day9_2(&inputs);
    println!("Day 9-2: {day9_2}");
}

fn day9_1(inputs: &String) -> usize {
    //parse lines into map
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

fn to_map(inputs: &String) -> Vec<Vec<u8>> {
    let lines = inputs.lines();
    //parse lines into map
    let mut input_map: Vec<Vec<u8>> = Vec::new();
    for (row, line) in lines.enumerate() {
        input_map.push(Vec::new());
        for c in line.chars() {
            let x: u8 = String::from(c).as_str().parse().unwrap();
            input_map[row].push(x)
        }
    }

    return input_map;
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

fn check_neighbor(
    point: (usize, usize),
    row_max: usize,
    col_max: usize,
    map: &Vec<Vec<u8>>,
) -> Option<Vec<(usize, usize)>> {
    let mut n: Vec<(usize, usize)> = Vec::with_capacity(4);
    //check up
    if point.0 > 0 {
        let t_value = map[point.0 - 1][point.1];
        if t_value < 9 && t_value > 0 {
            n.push((point.0 - 1, point.1));
        }
    }
    //check right
    if point.1 < col_max {
        let t_value = map[point.0][point.1 + 1];
        if t_value < 9 && t_value > 0 {
            n.push((point.0, point.1 + 1));
        }
    }
    //check down
    if point.0 < row_max {
        let t_value = map[point.0 + 1][point.1];
        if t_value < 9 && t_value > 0 {
            n.push((point.0 + 1, point.1));
        }
    }
    //check left
    if point.1 > 0 {
        let t_value = map[point.0][point.1 - 1];
        if t_value < 9 && t_value > 0 {
            n.push((point.0, point.1 - 1));
        }
    }
    if n.len() > 0 {
        return Some(n);
    }
    return None;
}

fn map_basin(
    point: (usize, usize),
    map: &Vec<Vec<u8>>,
    neighbors: &mut HashSet<(usize, usize)>,
) -> usize {
    let row_max = map.len() - 1;
    let col_max = map[0].len() - 1;

    let n = check_neighbor(point, row_max, col_max, &map);
    match n {
        Some(mut x) => {
            //filter out new unique neighbors
            x = x
                .iter()
                .filter(|t| neighbors.insert(**t))
                .map(|t| *t)
                .collect();
            //continue to recurse over new neighbors until none are left
            for i in x {
                map_basin(i, &map, neighbors);
            }
        }
        None => (),
    }

    return neighbors.len();
}
