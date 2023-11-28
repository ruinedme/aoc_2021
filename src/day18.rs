use timer::profile;

pub fn run_day18(inputs: &String) {
    profile! {
    let day18_1 = day18_1(&inputs);
    println!("Day 18-1: {day18_1}");
    }

    profile! {
    let day18_2 = day18_2(&inputs);
    println!("Day 18-2: {day18_2}");
    }
}

fn day18_1(inputs: &String) -> usize {
    let mut numbers: Vec<SnailFish> = parse(inputs);

    let x = add(&mut numbers);

    return mag(x);
}

fn day18_2(inputs: &String) -> usize {
    let numbers: Vec<SnailFish> = parse(inputs);

    let mut magnitudes: Vec<usize> = Vec::with_capacity(numbers.len() * 2);

    for x in &numbers[0..] {
        for y in &numbers[1..] {
            let a = x.clone();
            let b = y.clone();
            //do x + y
            let mut xy = Vec::from_iter([a, b]);
            let c1 = add(&mut xy);
            magnitudes.push(mag(c1));
            //do y + x
            let a = y.clone();
            let b = x.clone();
            let mut yx = Vec::from_iter([a, b]);
            let c2 = add(&mut yx);
            magnitudes.push(mag(c2));
        }
    }

    return *magnitudes.iter().max_by(|&a, &b| a.cmp(&b)).unwrap();
}

//Tuple Vector of (d)epth and a (n)umber in the snailfish pair
//[(d,n),(d,n),....]
type SnailFish = Vec<(u8, u8)>;

//Returns parsed numbers in reversed order
fn parse(inputs: &String) -> Vec<SnailFish> {
    let lines: Vec<&str> = inputs.lines().collect();
    let mut numbers: Vec<SnailFish> = Vec::with_capacity(lines.len());

    for line in lines {
        let mut num = SnailFish::with_capacity(line.len() / 2);
        let mut d: u8 = 0;

        line.as_bytes().iter().for_each(|&x| match x {
            b'[' => d += 1,
            b']' => d -= 1,
            b'0'..=b'9' => num.push((d, x - b'0')),
            b',' => {}
            _ => unreachable!("Found Invalid input: {}", char::from(x)),
        });

        numbers.push(num);
    }

    //reverse it so we can work from the end with pop()
    numbers.reverse();
    return numbers;
}

fn add(numbers: &mut Vec<SnailFish>) -> SnailFish {
    let mut ret_value = numbers
        .pop()
        .expect("Expected a Vec of at least 1, found None");

    if numbers.len() == 0 {
        reduce(&mut ret_value);
        return ret_value;
    }

    while numbers.len() > 0 {
        //"Add" next to value
        let mut next = numbers.pop().unwrap();
        ret_value.append(&mut next);

        //increment depth of all numbers
        ret_value.iter_mut().for_each(|x| {
            x.0 += 1;
        });

        //reduce value
        reduce(&mut ret_value);
    }

    return ret_value;
}

fn reduce(number: &mut SnailFish) {
    let mut can_explode = true;
    let mut can_split = true;

    while can_explode || can_split {
        //If any pair is nested inside four pairs, the leftmost such pair explodes. eg depth is 5 or more.
        can_explode = number.iter().any(|x| x.0 > 4);
        if can_explode {
            let e: Vec<(usize, (u8, u8))> = number
                .iter()
                .enumerate()
                .filter(|(_, &x)| x.0 > 4)
                .map(|(i, &x)| (i, x))
                .collect();
            //There should always be 2 regular numbers when exploding
            let left: (usize, (u8, u8)) = e[0];
            let right: (usize, (u8, u8)) = e[1];
            //add to the left
            if left.0 > 0 {
                number[left.0 - 1].1 += left.1 .1;
            }
            //add to the right
            if right.0 < number.len() - 1 {
                number[right.0 + 1].1 += right.1 .1;
            }
            //replace left and right with zero
            let zero = [((left.1 .0 - 1) as u8, 0)];
            number.splice(left.0..=right.0, zero);
            //back to the top
            continue;
        }
        //If any regular number is 10 or greater, the leftmost such regular number splits.
        can_split = number.iter().any(|x| x.1 > 9);
        if can_split {
            let e: Vec<(usize, (u8, u8))> = number
                .iter()
                .enumerate()
                .filter(|(_, &x)| x.1 > 9)
                .map(|(i, &x)| (i, x))
                .collect();
            //get split values
            let left = (e[0].1 .1 as f32 / 2f32).floor() as u8;
            let right = (e[0].1 .1 as f32 / 2f32).ceil() as u8;

            //replace number with pair
            let pair = [(e[0].1 .0 + 1, left), (e[0].1 .0 + 1, right)];
            number.splice(e[0].0..=e[0].0, pair);
        }
    }
}

//returns magnitude for a given number
fn mag(number: SnailFish) -> usize {
    let mut values: Vec<(u8, usize)> = number.iter().map(|x| (x.0, x.1 as usize)).collect();

    let max_depth = number.iter().max_by(|&a, &b| a.0.cmp(&b.0)).unwrap().0;

    //sum all nested values
    let mut current_depth = max_depth;
    while current_depth > 0 {
        while values.iter().any(|&x| x.0 == current_depth) {
            let e: Vec<(usize, (u8, usize))> = values
                .iter()
                .enumerate()
                .filter(|(_, &x)| x.0 == current_depth)
                .map(|(i, &x)| (i, x))
                .collect();
            //only work on 2 at a time
            let left = e[0].1 .1 * 3;
            let right = e[1].1 .1 * 2;
            //repalce left and right with the new sum
            values.splice(e[0].0..=e[1].0, [(current_depth - 1, left + right)]);
        }
        current_depth -= 1;
    }

    return values[0].1;
}
