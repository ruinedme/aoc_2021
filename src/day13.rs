use timer::profile;

pub fn run_day13(inputs: &String) {
    profile! {
    let day13_1 = day13_1(&inputs);
    println!("Day 13-1: {day13_1}");
    }

    profile! {
    let day13_2 = day13_2(&inputs);
    println!("Day 13-2: Code: ");
    day13_2.print_code();
    }
}

fn day13_1(inputs: &String) -> usize {
    let mut parsed_input = ParsedInput::new(inputs);

    parsed_input.fold_grid(parsed_input.folds[0]);

    return parsed_input.count_dots();
}

fn day13_2(inputs: &String) -> ParsedInput {
    let mut parsed_input = ParsedInput::new(inputs);

    let mut index = 0;
    while index < parsed_input.folds.len() {
        parsed_input.fold_grid(parsed_input.folds[index]);
        index += 1;
    }
    return parsed_input;
}

/// points: Vec<(x,y)>
/// folds: Vec<(0|1, column|row), 0 = x, y = 1
/// grid: y * x vector to hold populated points
struct ParsedInput {
    folds: Vec<(usize, usize)>,
    grid: Vec<Vec<u8>>,
}

impl ParsedInput {
    fn new(inputs: &String) -> Self {
        let mut points: Vec<(usize, usize)> = Vec::new();
        let mut folds: Vec<(usize, usize)> = Vec::new();
        let mut max_x = 0;
        let mut max_y = 0;
        let mut is_point = true;

        for line in inputs.lines() {
            if line == "" {
                is_point = false;
                continue;
            }
            // get points
            if is_point {
                let split: Vec<&str> = line.split(',').collect();
                let point = (split[0].parse().unwrap(), split[1].parse().unwrap());
                points.push(point);
                if point.0 > max_x {
                    max_x = point.0;
                }
                if point.1 > max_y {
                    max_y = point.1;
                }
            } else {
                //parse folding lines
                let split: Vec<&str> = line.split('=').collect();
                if split[0].ends_with('x') {
                    folds.push((0, split[1].parse().unwrap()));
                } else {
                    folds.push((1, split[1].parse().unwrap()));
                }
            }
        }
        //increase max x/y by 1 to account for array indexing
        max_x += 1;
        max_y += 1;
        //create grid
        let mut grid: Vec<Vec<u8>> = Vec::with_capacity(max_y);
        let mut i = 0;
        while i < max_y {
            let row: Vec<u8> = vec![0; max_x];
            grid.push(row);
            i += 1;
        }

        //populate points in grid
        for point in &points {
            grid[point.1][point.0] = 1;
        }

        return ParsedInput { folds, grid };
    }

    fn fold_grid(&mut self, fold: (usize, usize)) {
        //fold along column
        if fold.0 == 0 {
            let mut left: Vec<Vec<u8>> = Vec::with_capacity(self.grid.len());
            let mut right: Vec<Vec<u8>> = Vec::with_capacity(self.grid.len());

            for row in &self.grid {
                let (l, r) = row.split_at(fold.1);
                left.push(l.to_vec());
                let mut r = r.to_vec();
                r.reverse(); // fold to the left
                right.push(r);
            }

            //combine left and right
            for (y, _) in right.iter().enumerate() {
                right[y]
                    .iter()
                    .enumerate()
                    .filter(|(_, &col)| col == 1)
                    .for_each(|(x, _)| left[y][x] = 1);
            }
            self.grid = left;
        } else {
            //fold along row
            //There has to be a better way of doing this
            let top: Vec<&Vec<u8>> = self.grid[0..fold.1].iter().collect();
            let mut bottom: Vec<&Vec<u8>> = self.grid[fold.1 + 1..].iter().collect();

            //Fold the paper "bottom up"
            bottom.reverse();

            //combine top and bottom
            let mut new_grid: Vec<Vec<u8>> = Vec::with_capacity(top.len());
            for (y, &row) in top.iter().enumerate() {
                let mut new_row = row.clone();
                bottom[y]
                    .iter()
                    .enumerate()
                    .filter(|(_, &col)| col == 1)
                    .for_each(|(x, _)| new_row[x] = 1);

                new_grid.push(new_row);
            }
            self.grid = new_grid;
        }
    }

    fn count_dots(&self) -> usize {
        return self
            .grid
            .iter()
            .flatten()
            .fold(0, |acc: usize, &x| acc + x as usize) as usize;
    }

    fn print_code(&self) {
        for row in &self.grid {
            for col in row {
                if *col == 1 {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}
