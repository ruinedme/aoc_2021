pub fn run_day10(inputs: &String) {
    let day10_1 = day10_1(&inputs);
    println!("Day 10-1: {day10_1}");

    let day10_2 = day10_2(&inputs);
    println!("Day 10-2: {day10_2}");
}

fn day10_1(inputs: &String) -> usize {
    let lines: Vec<&str> = inputs.lines().collect();
    let mut total = 0;
    for line in lines {
        let (score, _stack) = parse_chunk(line);
        total += score;
    }
    return total;
}

fn day10_2(inputs: &String) -> usize {
    let lines: Vec<&str> = inputs.lines().collect();
    let mut scores: Vec<usize> = Vec::new();
    for line in lines {
        let (score, mut stack) = parse_chunk(line);
        if score == 0 {
            let complete_score = complete_line(&mut stack);
            scores.push(complete_score);
        }
    }
    scores.sort();
    return scores[scores.len() / 2];
}

fn match_token(matched: char, expected: char) -> bool {
    if matched != expected {
        return false;
    }
    return true;
}

//Returns 0 if chunk is valid or incomplete,
//Returns > 0 if chunk is corrupted
fn parse_chunk(line: &str) -> (usize, Vec<char>) {
    let mut stack: Vec<char> = Vec::with_capacity(line.len());
    for c in line.chars() {
        match c {
            '(' | '{' | '[' | '<' => {
                stack.push(c);
            }
            ')' => {
                if !match_token(stack.pop().unwrap(), '(') {
                    return (3, Vec::new());
                }
            }
            '}' => {
                if !match_token(stack.pop().unwrap(), '{') {
                    return (1197, Vec::new());
                }
            }
            ']' => {
                if !match_token(stack.pop().unwrap(), '[') {
                    return (57, Vec::new());
                }
            }
            '>' => {
                if !match_token(stack.pop().unwrap(), '<') {
                    return (25137, Vec::new());
                }
            }
            _ => unreachable!("Found Invalid Token"),
        }
    }
    return (0, stack);
}

//Completes the chunk by traversing the remaining stack and returning a score for the line
fn complete_line(stack: &mut Vec<char>) -> usize {
    stack.reverse();
    let mut score = 0;
    for c in stack {
        score *= 5;
        match c {
            '(' => score += 1,
            '[' => score += 2,
            '{' => score += 3,
            '<' => score += 4,
            _ => unreachable!(),
        }
    }
    return score;
}
