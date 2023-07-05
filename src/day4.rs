pub fn run_day4(inputs: &String) {
    let day4_1 = day4_1(&inputs);
    println!("Day 4-1: {day4_1}");

    let day4_2 = day4_2(&inputs);
    println!("Day 4-2: {day4_2}");
}

const BOARD_SIZE: usize = 5;
fn build_boards(inputs: &[&str]) -> Vec<[[u8; BOARD_SIZE]; BOARD_SIZE]> {
    let mut boards: Vec<[[u8; BOARD_SIZE]; BOARD_SIZE]> = Vec::new();

    let mut index = 0;
    let mut board_index = 0;

    while index < inputs.len() {
        boards.push([[0; BOARD_SIZE]; BOARD_SIZE]);
        let mut row_index = 0;
        while row_index < BOARD_SIZE {
            let row: Vec<u8> = inputs[index + row_index]
                .split(' ')
                .filter(|x| x.len() > 0)
                .map(|x| x.parse().expect("Expected Number"))
                .collect();
            let mut col_index = 0;
            while col_index < BOARD_SIZE {
                boards[board_index][row_index][col_index] = row[col_index];
                col_index += 1;
            }
            row_index += 1;
        }
        index += BOARD_SIZE + 1;
        board_index += 1;
    }

    return boards;
}

fn check_boards(boards: &mut Vec<[[u8; BOARD_SIZE]; BOARD_SIZE]>, n: u8) {
    for board in &mut *boards {
        'top: for row in board {
            for c in row {
                if *c == n {
                    *c = 100;
                    break 'top;
                }
            }
        }
    }
}

fn check_winners(boards: &Vec<[[u8; BOARD_SIZE]; BOARD_SIZE]>) -> isize {
    let mut board_index = 0;
    while board_index < boards.len() {
        //check rows
        let mut row_index = 0;
        while row_index < BOARD_SIZE {
            let mut col_index = 0;
            let mut row_total: usize = 0;
            while col_index < BOARD_SIZE {
                row_total += boards[board_index][row_index][col_index] as usize;
                col_index += 1;
            }
            if row_total == (100 * BOARD_SIZE) as usize {
                return board_index as isize;
            }
            row_index += 1;
        }
        //check columns
        let mut col_index = 0;
        while col_index < BOARD_SIZE {
            let mut row_index = 0;
            let mut col_total = 0;
            while row_index < BOARD_SIZE {
                col_total += boards[board_index][row_index][col_index] as usize;
                row_index += 1;
            }
            if col_total == (100 * BOARD_SIZE) as usize {
                return board_index as isize;
            }
            col_index += 1;
        }

        board_index += 1;
    }
    return -1;
}

fn sum_unmarked(board: &[[u8; BOARD_SIZE]; BOARD_SIZE]) -> usize {
    let mut sum: usize = 0;

    for row in board {
        //let x: usize = *row.iter().filter(|x| *x < &100).sum();
        for c in row {
            if c < &100 {
                sum += *c as usize;
            }
        }
    }

    return sum;
}

fn day4_1(inputs: &String) -> usize {
    let lines: Vec<&str> = inputs.lines().collect();
    //number sequence
    let sequence: Vec<u8> = lines[0]
        .split(',')
        .map(|x| x.parse().expect("expected number"))
        .collect();

    //build boards
    let mut boards = build_boards(&lines[2..]);

    //begin game
    for n in sequence {
        check_boards(&mut boards, n);
        let winner = check_winners(&boards);
        if winner >= 0 {
            let sum = sum_unmarked(&boards[winner as usize]);
            return (sum * n as usize) as usize;
        }
    }

    return 0;
}

fn day4_2(inputs: &String) -> usize {
    let lines: Vec<&str> = inputs.lines().collect();
    //number sequence
    let sequence: Vec<u8> = lines[0]
        .split(',')
        .map(|x| x.parse().expect("expected number"))
        .collect();
    //build boards
    let mut boards = build_boards(&lines[2..]);

    //begin game
    for n in sequence {
        check_boards(&mut boards, n);
        let mut winner = check_winners(&boards);
        //multiple boards can be marked as a winner in a single iteration
        //loop until all winner boards are removed
        while winner >= 0 {
            if boards.len() > 1 {
                boards.swap_remove(winner as usize);
                winner = check_winners(&boards);
            } else {
                let sum = sum_unmarked(&boards[winner as usize]);
                return (sum * n as usize) as usize;
            }
        }
    }
    return 0;
}
