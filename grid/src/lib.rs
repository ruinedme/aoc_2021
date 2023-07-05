/// Takes a multi line block of numbers and parses them into a grid
pub fn to_map(inputs: &String) -> Vec<Vec<u8>> {
    let lines = inputs.lines();
    
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

/// Up/Down Left/Right Neighbors, diagonals excluded
/// point: is (y, x)
pub fn get_cardinal_neighbors(point: (usize,usize),map: &Vec<Vec<u8>>) -> Vec<(usize,usize)> {
    let mut n: Vec<(usize, usize)> = Vec::with_capacity(4);
    let row_max = map.len() - 1;
    let col_max = map[0].len() - 1;
    //check up
    if point.0 > 0 {
        n.push((point.0 - 1, point.1));
    }
    //check right
    if point.1 < col_max {
        n.push((point.0, point.1 + 1));
    }
    //check down
    if point.0 < row_max {
        n.push((point.0 + 1, point.1));
    }
    //check left
    if point.1 > 0 {
        n.push((point.0, point.1 - 1));
    }
    return n;
}

/// Get Cardnial and Diagnal Neighbors
/// point: is (y, x)
pub fn get_all_neighbors(point: (usize,usize),map: &Vec<Vec<u8>>) -> Vec<(usize,usize)> {
    let mut n: Vec<(usize, usize)> = Vec::with_capacity(8);
    n.append(&mut get_cardinal_neighbors(point, &map));
    let row_max = map.len() - 1;
    let col_max = map[0].len() - 1;

    //check up/eft
    if point.0 > 0  && point.1 > 0 {
        n.push((point.0 - 1, point.1 - 1));
    }
    //check up/right
    if point.0 > 0 && point.1 < col_max {
        n.push((point.0 - 1, point.1 + 1));
    }
    //check down/left
    if point.0 < row_max && point.1 > 0 {
        n.push((point.0 + 1,point.1 -1));
    }
    //check down/right
    if point.0 < row_max && point.1 < col_max {
        n.push((point.0 +1, point.1 + 1))
    }

    return n;
}
