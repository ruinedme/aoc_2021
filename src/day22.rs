pub fn run_day22(inputs: &String) {
    let day22_1 = day22_1(&inputs);
    println!("Day 22-1: {day22_1}");

    let day22_2 = day22_2(&inputs);
    println!("Day 22-2: {day22_2}");
}

fn day22_1(inputs: &String) -> usize {
    let offset: isize = 50; //x,y,z can be +-50, offset input index by 50 for 0 based usized index
    let grid_len = offset * 2;
    // causes stack overflow when combined with other statements in Debug
    // let mut reactor: [[[bool;100];100];100] = [[[false;100];100];100];
    let mut reactor = [false; 1000000]; // 100^3

    let lines: Vec<&str> = inputs.lines().collect();

    // ((z + offset) * (y.len() * x.len()) + ((y + offset) * x.len()) + (x + offset)
    for line in lines {
        let step = Cuboid::new(line);
        if step.x1 < -(offset as isize) || step.x1 > offset as isize {
            break;
        }

        for x in step.x1..step.x2 {
            for y in step.y1..step.y2 {
                for z in step.z1..step.z2 {
                    let i = (((z + offset) * (grid_len * grid_len))
                        + ((y + offset) * grid_len)
                        + (x + offset)) as usize;
                    reactor[i] = step.state;
                }
            }
        }
    }

    return reactor
        .iter()
        .fold(0, |acc, &x| if x { acc + 1 } else { acc });
}

fn day22_2(_inputs: &String) -> usize {
   return  0;
}

#[allow(dead_code)]
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
