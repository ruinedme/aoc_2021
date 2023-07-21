use std::collections::HashMap;

const START: &str = "start";
const END: &str = "end";

pub fn run_day12(inputs: &String) {
    let day12_1 = day12_1(&inputs);
    println!("Day 1-1: {day12_1}");

    let day12_2 = day12_2(&inputs);
    println!("Day 1-2: {day12_2}");
}

fn day12_1(inputs: &String) -> usize {
    //parse
    let caves: HashMap<&str, Vec<&str>> = parse_input(inputs);

    //build paths
    let paths = build_paths(&caves, 1);

    return paths.len();
}

fn day12_2(inputs: &String) -> usize {
    //parse
    let caves: HashMap<&str, Vec<&str>> = parse_input(inputs);

    //build paths
    let paths = build_paths(&caves, 2);

    return paths.len();
}

fn parse_input(inputs: &String) -> HashMap<&str, Vec<&str>> {
    let mut caves: HashMap<&str, Vec<&str>> = HashMap::new();
    caves.insert(START, Vec::new());
    caves.insert(END, Vec::new());

    let lines = inputs.lines();
    for line in lines {
        let line_split: Vec<&str> = line.split('-').collect();
        let left = line_split[0];
        let right = line_split[1];

        if right != START {
            match caves.get_mut(left) {
                Some(x) => {
                    x.push(right);
                }
                None => {
                    caves.insert(left, vec![{ right }]);
                }
            }
        }
        if left != START {
            match caves.get_mut(right) {
                Some(x) => {
                    x.push(left);
                }
                None => {
                    caves.insert(right, vec![{ left }]);
                }
            }
        }
    }
    return caves;
}

//This feels awkward. Is there a better way to do this?
fn is_cave_small(cave: &str) -> bool {
    let cave_chars: Vec<char> = cave.chars().collect();
    return cave_chars[0].is_ascii_lowercase();
}

//returns true is given cave is small and visisted twice
fn has_visited_twice<'a>(path: &Vec<&'a str>, match_cave: &'a str) -> bool {
    if is_cave_small(match_cave) {
        return path.iter().filter(|&x| *x == match_cave).count() == 2;
    }
    return false;
}

//Retrun true if 0 or 1 small caves have been visited
fn filter_good_path(path: &Vec<&str>) -> bool {
    let mut small_cave_count: HashMap<&str, usize> = HashMap::with_capacity(path.len());
    for cave in path {
        if *cave == START || *cave == END {
            continue;
        }
        if is_cave_small(cave) {
            *small_cave_count.entry(cave).or_default() += 1;
        }
    }
    small_cave_count = small_cave_count
        .iter()
        .filter(|(_, &v)| v == 2)
        .map(|(&k, &v)| (k, v))
        .collect();

    return small_cave_count.len() < 2;
}

//There is probably a way to make this purely recursive and not split into 2 functions
//Idea is to get the inital fork from start, then build all possible paths from each of these "starting" caves
//add_forks() will build out the remainder of the paths for the given starting cave
fn build_paths<'a>(caves: &HashMap<&'a str, Vec<&'a str>>, part: u8) -> Vec<Vec<&'a str>> {
    let mut paths: Vec<Vec<&str>> = Vec::new();
    let start_points = caves.get(START).unwrap();

    for point in start_points {
        let start_path = vec![START, point];
        let mut completed_paths = match part {
            1 => add_forks(&start_path, caves),
            2 => add_forks2(&start_path, caves),
            _ => unreachable!(),
        };
        paths.append(&mut completed_paths);
    }

    return paths;
}

// take start path such as: start -> A
// take last point of start path as current cave: A
// get connected caves of current cave: b,c,end
// check if connected cave is small and already in path
// create new path for each connection
// add any path tha ends in "end" to complete paths
// recurse any other cave to continue building forks
fn add_forks<'a>(
    start_path: &Vec<&'a str>,
    caves: &HashMap<&'a str, Vec<&'a str>>,
) -> Vec<Vec<&'a str>> {
    let current_cave = start_path.last().unwrap();
    let forks = caves.get(current_cave).unwrap();

    let mut completed_paths: Vec<Vec<&str>> = Vec::with_capacity(forks.len());
    for cave in forks {
        //if cave is small AND already in the path
        if is_cave_small(cave) && start_path.iter().any(|x| *x == *cave) {
            continue;
        }

        let mut path = start_path.clone();
        if *cave == END {
            path.push(cave);
            completed_paths.push(path);
            continue;
        }

        //add connection
        path.push(cave);
        //and recurse
        let mut new_paths = add_forks(&path, caves);
        completed_paths.append(&mut new_paths);
    }

    return completed_paths;
}

//Can visit 1 small cave twice, all other small caves once, big cave unlimited
fn add_forks2<'a>(
    start_path: &Vec<&'a str>,
    caves: &HashMap<&'a str, Vec<&'a str>>,
) -> Vec<Vec<&'a str>> {
    let current_cave = start_path.last().unwrap();
    let forks = caves.get(current_cave).unwrap();

    let mut completed_paths: Vec<Vec<&str>> = Vec::with_capacity(forks.len());
    for cave in forks {
        //visit any small cave at most twice OR has visited 2 or more small caves twice
        if (is_cave_small(cave) && has_visited_twice(start_path, cave))
            || !filter_good_path(start_path)
        {
            continue;
        }

        let mut path = start_path.clone();
        if *cave == END {
            path.push(cave);
            completed_paths.push(path);
            continue;
        }

        //add connection
        path.push(cave);
        //and recurse
        let mut new_paths = add_forks2(&path, caves);
        completed_paths.append(&mut new_paths);
    }

    return completed_paths;
}
