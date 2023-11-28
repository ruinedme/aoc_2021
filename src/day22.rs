use timer::profile;

pub fn run_day22(inputs: &String) {
    profile! {
    let day22_1 = day22_1(&inputs);
    println!("Day 22-1: {day22_1}");
    }

    profile! {
    let day22_2 = day22_2(&inputs);
    println!("Day 22-2: {day22_2}");
    }
}

fn day22_1(inputs: &String) -> usize {
    let steps: Vec<Cuboid> = inputs.lines().map(|x| Cuboid::new(x)).collect();
    let region = Cuboid {
        state: true,
        x1: -50,
        x2: 51,
        y1: -50,
        y2: 51,
        z1: -50,
        z2: 51,
    };

    let mut instructions: Vec<Cuboid> = steps
        .iter()
        .filter(|&x| region.overlap(x).is_some())
        .map(|&x| x)
        .collect();

    return process_instructions(&mut instructions);
}

fn day22_2(inputs: &String) -> usize {
    // parse all the steps
    let mut steps: Vec<Cuboid> = inputs.lines().map(|x| Cuboid::new(x)).collect();

    return process_instructions(&mut steps[..]);
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
struct Cuboid {
    state: bool,
    x1: isize,
    x2: isize,
    y1: isize,
    y2: isize,
    z1: isize,
    z2: isize,
}

impl Cuboid {
    /// Expected format "on|off x=-?d+..-?d+,y=-?d+..-?d+,z=-?d+..-?d+"
    ///
    /// Example: "on x=-20..26,y=-36..17,z=-47..7"
    ///
    /// Example: "on x=-22..28,y=-29..23,z=-38..16"
    ///
    /// Example: "off x=18..30,y=-20..-8,z=-3..13"
    ///
    /// Example: "off x=-32..-23,y=11..30,z=-14..3"
    ///
    /// Ranges for x,y,z are inclusive
    ///
    /// *1 garunteed to be smaller than *2
    fn new(input: &str) -> Self {
        // Is there a way to do this without so many type casts?
        // &str -> &[u8] -> char -> String -> isize
        let bytes = input.as_bytes();
        let mut state = false;
        if bytes[1] == b'n' {
            state = true;
        }
        let mut v = String::new();
        let mut values: Vec<isize> = Vec::with_capacity(6);
        for c in bytes.iter() {
            if (*c >= b'0' && *c <= b'9') || *c == b'-' {
                v.push(char::from(*c));
                continue;
            }
            if (*c == b'.' || *c == b',') && !v.is_empty() {
                values
                    .push(v.parse().expect(
                        ("Expected valid isize. Found: ".to_owned() + v.as_str()).as_str(),
                    ));
                v = String::new();
            }
        }
        values.push(v.parse().unwrap());
        return Self {
            state,
            x1: values[0],
            x2: values[1] + 1,
            y1: values[2],
            y2: values[3] + 1,
            z1: values[4],
            z2: values[5] + 1,
        };
    }

    /// returns the volume of the cuboid
    fn volume(&self) -> usize {
        let x = self.x2 - self.x1;
        let y = self.y2 - self.y1;
        let z = self.z2 - self.z1;

        return (x * y * z) as usize;
    }

    /// takes self and another cuboid and returns the overlapping cube with state of self
    fn overlap(&self, other: &Self) -> Option<Self> {
        let overlap = Self {
            state: self.state,
            x1: self.x1.max(other.x1),
            x2: self.x2.min(other.x2),
            y1: self.y1.max(other.y1),
            y2: self.y2.min(other.y2),
            z1: self.z1.max(other.z1),
            z2: self.z2.min(other.z2),
        };
        if overlap.is_valid() {
            return Some(overlap);
        } else {
            return None;
        }
    }

    fn is_valid(&self) -> bool {
        return self.x1 < self.x2 && self.y1 < self.y2 && self.z1 < self.z2;
    }
}

impl Default for Cuboid {
    /// Returns an empty cube with state = false
    fn default() -> Self {
        Self {
            state: false,
            x1: 0,
            x2: 0,
            y1: 0,
            y2: 0,
            z1: 0,
            z2: 0,
        }
    }
}

// Answer derived from https://github.com/Dullstar/Advent_Of_Code/blob/main/python/year2021/day22.py
fn process_instructions(instructions: &mut [Cuboid]) -> usize {
    let mut placed: Vec<Cuboid> = Vec::new();
    let mut volume = 0;

    instructions.reverse();
    for instruction in instructions {
        if instruction.state {
            let mut overlaps: Vec<Cuboid> = Vec::new();
            for cuboid in &placed {
                match instruction.overlap(&cuboid) {
                    Some(x) => {
                        overlaps.push(x);
                    }
                    None => (),
                }
            }
            volume += instruction.volume() - process_instructions(&mut overlaps[..]);
        }
        placed.push(*instruction);
    }
    return volume;
}
