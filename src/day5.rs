use timer::profile;

pub fn run_day5(inputs: &String) {
    profile! {
    let day5_1 = day5_1(&inputs);
    println!("Day 5-1: {day5_1}");
    }

    profile! {
    let day5_2 = day5_2(&inputs);
    println!("Day 5-2: {day5_2}");
    }
}

const GRID_SIZE: usize = 1024;
#[derive(Debug)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

fn parse_points(inputs: Vec<&str>) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::with_capacity(inputs.len());
    for i in inputs {
        //Is there a way to do this with an iterator?
        let raw_points: Vec<&str> = i.split(" -> ").collect();
        let p1: Vec<&str> = raw_points[0].split(',').collect();
        let p2: Vec<&str> = raw_points[1].split(',').collect();

        lines.push(Line {
            p1: Point {
                x: p1[0].parse().expect("Expected Number"),
                y: p1[1].parse().expect("Expected Number"),
            },
            p2: Point {
                x: p2[0].parse().expect("Expected Number"),
                y: p2[1].parse().expect("Expected Number"),
            },
        });
    }
    return lines;
}

//Lines can be left -> right, right -> left, top -> bottom, bottom -> top
//Process lines only in left -> right, top -> bottom, reording p1 and p2 as needed
fn draw_lines(grid: &mut Vec<[u16; GRID_SIZE]>, lines: &Vec<Line>, draw_diagonal: bool) {
    for line in lines {
        let mut p1 = &line.p1;
        let mut p2 = &line.p2;
        // Vertical Line
        if p1.x == p2.x {
            if p2.y < p1.y {
                p1 = &line.p2;
                p2 = &line.p1;
            }
            let mut index = p1.y as usize;
            while index <= p2.y as usize {
                grid[index][p1.x as usize] += 1;
                index += 1;
            }
            continue;
        }

        //Horizontal Line
        if p1.y == p2.y {
            if p2.x < p1.x {
                p1 = &line.p2;
                p2 = &line.p1;
            }
            let mut index = p1.x as usize;
            while index <= p2.x as usize {
                grid[p1.y as usize][index] += 1;
                index += 1;
            }
            continue;
        }

        if draw_diagonal {
            let mut index = 0;
            if p1.x < p2.x && p1.y > p2.y {
                //handle up, right
                let max_offset = (p2.x - p1.x) as usize;
                while index <= max_offset as usize {
                    grid[p1.y as usize - index][p1.x as usize + index] += 1;
                    index += 1;
                }
            } else if p1.x > p2.x && p1.y < p2.y {
                //handle down,left
                let max_offset = (p1.x - p2.x) as usize;
                while index <= max_offset {
                    grid[p1.y as usize + index][p1.x as usize - index] += 1;
                    index += 1;
                }
            } else if p1.x < p2.x && p1.y < p2.y {
                //handle down,right
                let max_offset = (p2.x - p1.x) as usize;
                while index <= max_offset as usize {
                    grid[p1.y as usize + index][p1.x as usize + index] += 1;
                    index += 1;
                }
            } else if p1.x > p2.x && p1.y > p2.y {
                //up, left
                let max_offset = (p1.x - p2.x) as usize;
                while index <= max_offset as usize {
                    grid[p1.y as usize - index][p1.x as usize - index] += 1;
                    index += 1;
                }
            } else {
                unreachable!("{:?}", line);
            }
        }
    }
}

fn day5_1(inputs: &String) -> usize {
    let lines: Vec<&str> = inputs.lines().collect();
    let line_points = parse_points(lines);

    //Given input does not go past 990 in the X direction
    //And 989 in the Y direction
    //Size 1024 x 1024 is arbitrary just make the grid a square instead of an oblong rectangle
    let mut grid: Vec<[u16; GRID_SIZE]> = vec![[0; GRID_SIZE]; GRID_SIZE];
    //let mut grid: [[u16;GRID_SIZE];GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE]; //Blows the stack in Debug???
    //let mut grid: Box<[[u16; GRID_SIZE]]> = Box::new([[GRID_SIZE as u16; GRID_SIZE]]); //Creates array size of 1???

    //"Draw" Lines on the grid, incrementing each point on the line by 1
    draw_lines(&mut grid, &line_points, false);

    //Count "danger zone" points, any grid sqaure with value >= 2
    //This syntax is terrible...
    let danger_points: Vec<u16> = grid
        .iter()
        .flat_map(|x| x.iter().filter(|y| *y >= &2).map(move |y| *y))
        .collect();
    return danger_points.len();
}

fn day5_2(inputs: &String) -> usize {
    let lines: Vec<&str> = inputs.lines().collect();
    let line_points = parse_points(lines);

    //Given input does not go past 990 in the X direction
    //And 989 in the Y direction
    //Size 1024 x 1024 is arbitrary just make the grid a square instead of an oblong rectangle
    let mut grid: Vec<[u16; GRID_SIZE]> = vec![[0; GRID_SIZE]; GRID_SIZE];
    //let mut grid: [[u16;GRID_SIZE];GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE]; //Blows the stack in Debug???
    //let mut grid: Box<[[u16; GRID_SIZE]]> = Box::new([[GRID_SIZE as u16; GRID_SIZE]]); //Creates array size of 1???

    //"Draw" Lines on the grid, incrementing each square by 1
    draw_lines(&mut grid, &line_points, true);

    //Count "danger zone" points, any grid sqaure with value >= 2
    //This syntax is terrible...
    let danger_points: Vec<u16> = grid
        .iter()
        .flat_map(|x| x.iter().filter(|y| *y >= &2).map(move |y| *y))
        .collect();
    return danger_points.len();
}
