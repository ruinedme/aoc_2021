use std::usize;

pub fn run_day21(inputs: &String) {
    let day21_1 = day21_1(&inputs);
    println!("Day 21-1: {day21_1}");

    let day21_2 = day21_2(&inputs);
    println!("Day 21-2: {day21_2}");
}

fn day21_1(inputs: &String) -> usize {
    let lines: Vec<&str> = inputs.lines().collect();

    let x = lines[0].as_bytes();
    let mut p1: usize = (x[x.len() - 1] - b'0') as usize;

    let x = lines[1].as_bytes();
    let mut p2: usize = (x[x.len() - 1] - b'0') as usize;

    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut turn = 1;
    let mut deterministic_die = 1;
    let max_die = 100;
    let mut loser_score = 0;
    while p1_score < 1000 && p2_score < 1000 {
        let mut roll: usize = 0;
        for _ in 0..3 {
            roll += deterministic_die;
            deterministic_die += 1;
            if deterministic_die > max_die {
                deterministic_die = 1;
            }
        }
        if turn % 2 == 1 {
            p1 = (p1 + roll) % 10;
            if p1 == 0 {
                p1 = 10;
            }
            p1_score += p1;
            if p1_score >= 1000 {
                loser_score = p2_score;
            }
        } else {
            p2 = (p2 + roll) % 10;
            if p2 == 0 {
                p2 = 10;
            }
            p2_score += p2;
            if p2_score >= 1000 {
                loser_score = p1_score;
            }
        }
        turn += 1;
    }

    return loser_score * (turn * 3 - 3);
}

fn day21_2(inputs: &String) -> usize {
    // parse input
    let lines: Vec<&str> = inputs.lines().collect();

    // get starting positions (techincally doesn't work if starting position is 10)
    let x = lines[0].as_bytes();
    let p1: usize = (x[x.len() - 1] - b'0') as usize;

    let x = lines[1].as_bytes();
    let p2: usize = (x[x.len() - 1] - b'0') as usize;

    let p = [scores(p1), scores(p2)];

    // sum up games for player 1 and 2 and return the max
    return (1..11)
        .map(|t| p[0].0[t] * p[1].1[t - 1])
        .sum::<usize>()
        .max((1..11).map(|t| p[1].0[t - 1] * p[0].1[t - 1]).sum());
}

// solution from: https://github.com/timvisee/advent-of-code-2021/blob/master/day21b/src/main.rs
// How are these probabilites calcuated?
const PROB: [usize; 7] = [1, 3, 6, 7, 6, 3, 1];

fn scores(pos: usize) -> ([usize; 11], [usize; 11]) {
    let mut tab = [[[0; 22]; 11]; 11];
    tab[0][pos][0] = 1;
    // player 1
    (1..11).for_each(|t| {
        // player 2
        (1..11).for_each(|p| {
            // combinations of the 2 players /2 - 1? 2C11 = 45, floor(45/2) - 1 = 21
            // or simply just 11 + 11?
            (0..21).for_each(|s| {
                PROB.iter().enumerate().for_each(|(i, w)| {
                    let q = ((p + i + 2) % 10) + 1; // players new position
                    let v = (q + s).min(21); // the score for the turn
                    tab[t][q][v] += w * tab[t - 1][p][s];
                });
            });
        });
    });

    let mut out = ([0; 11], [0; 11]);
    tab.iter().enumerate().for_each(|(t, tab)| {
        tab[1..].iter().for_each(|tab| {
            tab[..21].iter().for_each(|tab| out.1[t] += tab); // games where player 2 won?
            out.0[t] += tab[21]; // games where player 1 won?
        });
    });
    return out;
}
