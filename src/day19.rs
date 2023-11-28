use timer::profile;

pub fn run_day19(inputs: &String) {
    profile! {
    let day19_1 = day19_1(&inputs);
    println!("Day 19-1: {day19_1}");
    }

    profile! {
    let day19_2 = day19_2(&inputs);
    println!("Day 19-2: {day19_2}");
    }
}

fn day19_1(_inputs: &String) -> usize {
    return 0;
}

fn day19_2(_inputs: &String) -> usize {
    return 0;
}

//https://jlmartin.ku.edu/courses/math105-F11/Lectures/chapter6-part2.pdf
// const ALIGNMENT_THRESHOLD: usize = 12;
// const EDGE_THRESHOLD: usize = ALIGNMENT_THRESHOLD * (ALIGNMENT_THRESHOLD -1) / 2;

//All rotations a cube can be in
const _ROTATIONS: [[isize; 3]; 24] = [
    [0, 0, 0], //identity?
    [1, 0, 0],
    [0, 1, 0],
    [0, 0, 1],
    [1, 1, 0],
    [0, 1, 1],
    [1, 0, 1],
    [1, 1, 1],
    [-1, 0, 0],
    [0, -1, 0],
    [0, 0, -1],
    [-1, -1, 0],
    [0, -1, -1],
    [-1, 0, -1],
    [-1, -1, -1],
    [-1, 1, 1],
    [1, -1, 1],
    [1, 1, -1],
    [-1, -1, 1],
    [1, -1, -1],
    [-1, 1, -1],
    [-1, 1, 0],
    [1, -1, 0],
    [1, 0, -1],
];

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Scanner {
    id: usize,
    beacons: Vec<Point>,
}

impl Scanner {
    /// Returns a Vec of Scanners
    fn _parse(inputs: &String) -> Vec<Self> {
        let lines: Vec<&str> = inputs.lines().collect();
        let mut ret_value = Vec::new();
        let mut beacons: Vec<Point> = Vec::new();
        let mut scanner_id = 0;
        for line in lines {
            if line.starts_with("---") {
                continue;
            }
            if line == "" {
                ret_value.push(Scanner {
                    id: scanner_id,
                    beacons: beacons.clone(),
                });
                beacons.clear();
                scanner_id += 1;
                continue;
            }
            let split: Vec<&str> = line.split(",").collect();
            let x = split[0].parse().unwrap();
            let y = split[1].parse().unwrap();
            let z = split[2].parse().unwrap();

            beacons.push(Point { x, y, z });
        }

        ret_value.push(Scanner {
            id: scanner_id,
            beacons,
        });

        return ret_value;
    }
}
