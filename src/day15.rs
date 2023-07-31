use std::{collections::HashMap, usize::MAX};

use grid::{get_cardinal_neighbors, to_map};

pub fn run_day15(inputs: &String) {
    let day15_1 = day15_1(&inputs);
    println!("Day 15-1: {day15_1}");

    let day15_2 = day15_2(&inputs);
    println!("Day 15-2: {day15_2}");
}

fn day15_1(inputs: &String) -> usize {
    let map = to_map(inputs);

    //start at top left, points are (y,x)
    let current_point: (usize, usize) = (0, 0);
    let taget_point = (map.len() - 1, map[0].len() - 1);
    let risk_level: usize = a_star_search(current_point, taget_point, &map, &manhattan_distance)
        .expect("Expected number found None");

    return risk_level;
}

fn day15_2(_inputs: &String) -> usize {
    return 0;
}

/// Heuristic function that calculates the manhattan distance from a given point to a goal
/// points are (y,x)
fn manhattan_distance(point: &(usize, usize), goal: &(usize, usize)) -> usize {
    return point.0.abs_diff(goal.0) + point.1.abs_diff(goal.1);
}

/// A* finds a path from start to goal.
///
/// f(n) = g(n) + h(n)
///
/// g(n) = is the cost of the path from the start node to n
///
/// h(n) = is a heuristic function that estimates the cost of the cheapest path from n to the goal.
///
/// h: &dyn Fn((usize,usize),(usize,usize)) -> usize, Function to take the current node, and goal and calcluate cost of path from current
fn a_star_search(
    start: (usize, usize),
    goal: (usize, usize),
    map: &Vec<Vec<u8>>,
    h: &dyn Fn(&(usize, usize), &(usize, usize)) -> usize,
) -> Option<usize> {
    let capacity = map.len() * map.len();
    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    // This is usually implemented as a min-heap or priority queue rather than a hash-set.
    let mut open_set: Vec<(usize, usize)> = Vec::with_capacity(capacity);
    open_set.push(start);

    // For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from the start
    // to n currently known.
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::with_capacity(capacity);

    //came_from.insert((0,0), (0,0));
    // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
    // Default value for entry is Infinity
    let mut g_score: HashMap<(usize, usize), usize> = HashMap::with_capacity(capacity);
    g_score.insert(start, 0);

    // For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
    // how cheap a path could be from start to finish if it goes through n.
    let mut f_score: HashMap<(usize, usize), usize> = HashMap::with_capacity(capacity);
    f_score.insert(start, h(&start, &goal));

    println!("starting loop");
    while !open_set.is_empty() {
        // This operation can occur in O(Log(N)) time if openSet is a min-heap or a priority queue
        //  current := the node in openSet having the lowest fScore[] value
        let current = *f_score
            .iter()
            .filter(|&x| open_set.contains(x.0))
            .min_by(|&a, &b| a.1.cmp(&b.1))?
            .0;
        if current == goal {
            println!("found goal:");
            let mut t = current;
            let mut risk = map[current.0][current.1] as usize;
            //let mut path: Vec<(usize,usize)> = Vec::new();
            //path.push(current);
            while t != start {
                let n = *came_from.get(&t)?;
                risk += map[n.0][n.1] as usize;
                //path.push(n);
                t = n;
            }
            risk -= map[start.0][start.1] as usize;
            //path.reverse();
            //print_grid(&path, map);
            return Some(risk);
        }
        //Should only return 1 element, not sure if there is a better way to this
        let current_index: Vec<usize> = open_set
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == current)
            .map(|(i, _)| i)
            .collect();

        open_set.swap_remove(current_index[0]);

        let neighbors = get_cardinal_neighbors(current, map);
        for neighbor in &neighbors {
            //g_score.get(neighbor).get_or_insert(&MAX); //.get_or_insert was not actually inserting values?
            if !g_score.contains_key(neighbor) {
                g_score.insert(*neighbor, MAX);
            }
            // d(current,neighbor) is the weight of the edge from current to neighbor
            // tentative_gScore is the distance from start to the neighbor through current
            // tentative_gScore := gScore[current] + d(current, neighbor)
            let tentative_gscore = *g_score.get(&current)? + map[neighbor.0][neighbor.1] as usize;

            if tentative_gscore < *g_score.get(neighbor)? {
                came_from.insert(*neighbor, current);
                g_score.insert(*neighbor, tentative_gscore);
                f_score.insert(*neighbor, tentative_gscore + h(neighbor, &goal));

                if !open_set.contains(neighbor) {
                    open_set.push(*neighbor);
                }
            }
        }
    }

    return None;
}

/*
fn print_grid(path: &Vec<(usize, usize)>, map: &Vec<Vec<u8>>) {
    for (i, y) in map.iter().enumerate() {
        for (j, _) in y.iter().enumerate() {
            if path.contains(&(i, j)) {
                print!("X");
            } else {
                print!("_");
            }
        }
        println!();
    }
}
*/
