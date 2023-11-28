use timer::profile;

pub fn run_day8(inputs: &String) {
    profile! {
    let day8_1 = day8_1(&inputs);
    println!("Day 8-1: {day8_1}");
    }

    profile! {
    let day8_2 = day8_2(&inputs);
    println!("Day 8-2: {day8_2}");
    }
}

//Count the number of 1,4,7,8 that appear in the output of 7 segment display
fn day8_1(inputs: &String) -> usize {
    let lines: Vec<&str> = inputs.lines().collect();

    let mut count = 0;
    for line in lines {
        let values: Vec<&str> = line.split(" | ").collect();
        let output = values[1].split(" ");
        for s in output {
            let len = s.len();
            match len {
                2 | 3 | 4 | 7 => count += 1,
                1 | 5 | 6 => continue,
                _ => unreachable!("Length should be between 1 and 7"),
            }
        }
    }
    return count;
}

fn day8_2(inputs: &String) -> usize {
    let lines: Vec<&str> = inputs.lines().collect();
    let zero: [u8; 7] = [1, 1, 1, 0, 1, 1, 1];
    let one: [u8; 7] = [0, 0, 1, 0, 0, 1, 0];
    let two: [u8; 7] = [1, 0, 1, 1, 1, 0, 1];
    let three: [u8; 7] = [1, 0, 1, 1, 0, 1, 1];
    let four: [u8; 7] = [0, 1, 1, 1, 0, 1, 0];
    let five: [u8; 7] = [1, 1, 0, 1, 0, 1, 1];
    let six: [u8; 7] = [1, 1, 0, 1, 1, 1, 1];
    let seven: [u8; 7] = [1, 0, 1, 0, 0, 1, 0];
    let eight: [u8; 7] = [1, 1, 1, 1, 1, 1, 1];
    let nine: [u8; 7] = [1, 1, 1, 1, 0, 1, 1];
    let seg_patterns = vec![[zero, one, two, three, four, five, six, seven, eight, nine]];
    let char_numbers = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut total: usize = 0;
    for line in lines {
        let split: Vec<&str> = line.split(" | ").collect();
        let mut patterns: Vec<&str> = split[0].split(' ').collect();
        let value: Vec<&str> = split[1].split(' ').collect();
        //take 10 unique patterns and map map the segments a-g
        let segments = map_patterns(&mut patterns);

        let mut decoded_value = String::new();
        for v in value.iter() {
            let mut value_pattern: [u8; 7] = [0; 7];
            //take each letter in the out put value, and create a value pattern
            for c in v.chars() {
                for (i, s) in segments.iter().enumerate() {
                    if c == *s {
                        value_pattern[i] = 1;
                        break;
                    }
                }
            }
            //match the pattern to the appropriate number
            for s in seg_patterns.iter() {
                for (i, j) in s.iter().enumerate() {
                    if *j == value_pattern {
                        decoded_value.push(char_numbers[i]);
                    }
                }
            }
        }
        total += decoded_value.parse::<usize>().unwrap();
    }

    return total;
}

// 0 top
// 1 top left
// 2 top right
// 3 middle
// 4 bottm left
// 5 bottom right
// 6 bottom
//This seems messy, there is probably a better way to do this
fn map_patterns(patterns: &mut Vec<&str>) -> [char; 7] {
    patterns.sort_by(|a, b| a.len().cmp(&b.len()));

    let one: Vec<char> = patterns[0].chars().collect();
    let seven = patterns[1];
    let four = patterns[2];

    //A=0,G=6
    let segment_map: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let mut char_count: [usize; 7] = [0; 7];
    for p in patterns {
        for c in p.chars() {
            match c {
                'a' => char_count[0] += 1,
                'b' => char_count[1] += 1,
                'c' => char_count[2] += 1,
                'd' => char_count[3] += 1,
                'e' => char_count[4] += 1,
                'f' => char_count[5] += 1,
                'g' => char_count[6] += 1,
                _ => unreachable!("Found invalid char"),
            }
        }
    }

    // 0 top
    // 1 top left
    // 2 top right
    // 3 middle
    // 4 bottm left
    // 5 bottom right
    // 6 bottom
    let mut pattern_map: [char; 7] = ['\0'; 7];
    //set top
    for c in seven.chars() {
        let t: Vec<char> = one.iter().filter(|x| *x == &c).map(|x| *x).collect();
        if t.len() == 0 {
            pattern_map[0] = c;
            break;
        }
    }
    let mut bot_mid_index = [-1, -1];
    //set unique
    for (i, v) in char_count.iter().enumerate() {
        if pattern_map[0] == segment_map[i] {
            continue;
        }
        match v {
            4 => pattern_map[4] = segment_map[i],
            6 => pattern_map[1] = segment_map[i],
            7 => {
                if bot_mid_index[0] == -1 {
                    bot_mid_index[0] = i as i32;
                } else {
                    bot_mid_index[1] = i as i32;
                }
            }
            8 => pattern_map[2] = segment_map[i], //The other 8 is top segment
            9 => pattern_map[5] = segment_map[i],
            _ => unreachable!("v is not right: {v}"),
        }
    }

    //set middle
    for c in four.chars() {
        if c != pattern_map[0] && c != pattern_map[1] && c != pattern_map[2] && c != pattern_map[5]
        {
            pattern_map[3] = c;
        }
    }

    //set bottom
    for i in bot_mid_index {
        if segment_map[i as usize] != pattern_map[3] {
            pattern_map[6] = segment_map[i as usize];
        }
    }

    return pattern_map;
}
