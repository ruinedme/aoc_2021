use grid::{get_cardinal_neighbors, to_map};
use std::{collections::HashMap, usize::MAX};
use timer::profile;

pub fn run_day15(inputs: &String) {
    profile! {
    let day15_1 = day15_1(&inputs);
    println!("Day 15-1: {day15_1}");
    }

    profile! {
    let day15_2 = day15_2(&inputs);
    println!("Day 15-2: {day15_2}");
    }
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

//same as pt1 but need to expand the map x5 in both directions
fn day15_2(inputs: &String) -> usize {
    let map = to_map(inputs);
    let map_scale: usize = 4;

    let mut big_row = map.clone();
    //expand the columns
    for s in 1..=map_scale {
        for (y, row) in map.iter().enumerate() {
            let mut new_row: Vec<u8> = row
                .iter()
                .map(|&x| {
                    let t = x + s as u8;
                    if t > 9 {
                        10u8.abs_diff(t + 1)
                    } else {
                        t
                    }
                })
                .collect();
            big_row[y].append(&mut new_row);
        }
    }
    let mut big_map = big_row.clone();
    //expand the rows
    for s in 1..=map_scale {
        let mut tile: Vec<Vec<u8>> = big_row
            .iter()
            .map(|y| {
                y.iter()
                    .map(|&x| {
                        let t = x + s as u8;
                        if t > 9 {
                            10u8.abs_diff(t + 1)
                        } else {
                            t
                        }
                    })
                    .collect()
            })
            .collect();
        big_map.append(&mut tile);
    }
    drop(map);
    drop(big_row);

    //start at top left, points are (y,x)
    let current_point: (usize, usize) = (0, 0);
    let taget_point = (big_map.len() - 1, big_map[0].len() - 1);
    let risk_level: usize =
        a_star_search(current_point, taget_point, &big_map, &manhattan_distance)
            .expect("Expected number found None");

    return risk_level;
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
    //let capacity = map.len() * map.len();
    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    // This is usually implemented as a min-heap or priority queue rather than a hash-set.
    let mut open_set: Vec<(usize, usize)> = Vec::new();
    open_set.push(start);

    // For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from the start
    // to n currently known.
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::with_capacity(map.len());
    //came_from.insert((0,0), (0,0));
    // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
    // Default value for entry is Infinity
    let mut g_score: HashMap<(usize, usize), usize> = HashMap::with_capacity(map.len());
    g_score.insert(start, 0);

    // For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
    // how cheap a path could be from start to finish if it goes through n.
    let mut f_score: HashMap<(usize, usize), usize> = HashMap::with_capacity(map.len());
    f_score.insert(start, h(&start, &goal));

    while !open_set.is_empty() {
        // This operation can occur in O(Log(N)) time if openSet is a min-heap or a priority queue
        //  current := the node in openSet having the lowest fScore[] value
        let current = *f_score
            .iter()
            .filter(|&x| open_set.contains(x.0))
            .min_by(|&a, &b| a.1.cmp(&b.1))?
            .0;
        f_score.remove(&current); //minimize vector reallocations, this alone changes debug runtime from 158s to 4s

        if current == goal {
            let (_path, risk) = reconstruct_path(&came_from, current, start, map);

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
            //Slightly faster to insert as we go than pre-calculate each key with "Infinity"
            //~0.5s faster in debug, ~50ms faster in release
            if !g_score.contains_key(&neighbor) {
                g_score.insert(*neighbor, MAX); //Using MAX as sentinel value for Infinity
            }
            // d(current,neighbor) is the weight of the edge from current to neighbor
            // tentative_gScore is the distance from start to the neighbor through current
            // tentative_gScore := gScore[current] + d(current, neighbor)
            let tentative_gscore = *g_score.get(&current)? + map[neighbor.0][neighbor.1] as usize;

            if tentative_gscore < *g_score.get(&neighbor)? {
                came_from.insert(*neighbor, current);
                g_score.insert(*neighbor, tentative_gscore);
                f_score.insert(*neighbor, tentative_gscore + h(&neighbor, &goal));

                if !open_set.contains(&neighbor) {
                    open_set.push(*neighbor);
                }
            }
        }
    }

    return None;
}

// fn print_grid(path: &Vec<(usize, usize)>, map: &Vec<Vec<u8>>) {
//     for (i, y) in map.iter().enumerate() {
//         for (j, _) in y.iter().enumerate() {
//             if path.contains(&(i, j)) {
//                 print!("X");
//             } else {
//                 print!("_");
//             }
//         }
//         println!();
//     }
// }

fn reconstruct_path(
    came_from: &HashMap<(usize, usize), (usize, usize)>,
    current: (usize, usize),
    start: (usize, usize),
    map: &Vec<Vec<u8>>,
) -> (Vec<(usize, usize)>, usize) {
    let mut t = current;
    let mut risk = map[current.0][current.1] as usize;
    let mut path: Vec<(usize, usize)> = Vec::new();
    path.push(current);
    while t != start {
        let n = *came_from.get(&t).unwrap();
        risk += map[n.0][n.1] as usize;
        path.push(n);
        t = n;
    }
    risk -= map[start.0][start.1] as usize;
    path.reverse();

    return (path, risk);
}
