pub fn run_day20(inputs: &String) {
    let day20_1 = day20_1(&inputs);
    println!("Day 20-1: {day20_1}");

    let day20_2 = day20_2(&inputs);
    println!("Day 20-2: {day20_2}");
}

const HASH: u8 = b'#';
const DOT: u8 = b'.';

fn day20_1(inputs: &String) -> usize {
    let (enhancement_algo, mut grid, image_width) = create_grid(inputs);

    grid = enhance_image(2, grid, enhancement_algo, image_width);

    // count up all the lit pixels
    let sum = grid
        .iter()
        .flatten()
        .fold(0, |acc: usize, &x| if x == HASH { acc + 1 } else { acc });

    return sum;
}

#[allow(dead_code)]
fn day20_2(inputs: &String) -> usize {
    let (enhancement_algo, mut grid, image_width) = create_grid(inputs);

    grid = enhance_image(50, grid, enhancement_algo, image_width);
    // count up all the lit pixels
    // given the logic in enhance_image() this fails on odd numbered iterations for my input because all pixels outside the image would be considered light
    let sum = grid
        .iter()
        .flatten()
        .fold(0, |acc: usize, &x| if x == HASH { acc + 1 } else { acc });

    return sum;
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<u8>>) {
    println!("{:=<125}", "=");
    grid.iter().for_each(|y| {
        y.iter().for_each(|&x| {
            if x == HASH {
                print!("#");
            } else {
                print!(".");
            }
        });
        println!();
    });
    println!("{:=<125}", "=");
}
/// Returns enhancement algorithm index
fn enhance_pixel(pixel: (usize, usize), grid: &Vec<Vec<u8>>, filler: u8) -> usize {
    let x = pixel.0;
    let y = pixel.1;
    let width = grid[0].len() - 1;

    let mut top: [u8; 3] = [filler, filler, filler];
    let mut mid: [u8; 3] = [filler, grid[y][x], filler];
    let mut bot: [u8; 3] = [filler, filler, filler];

    // top
    if y > 0 {
        top[1] = grid[y - 1][x]; //  top center
        if x > 0 {
            top[0] = grid[y - 1][x - 1]; // top left
        }
        if x < width {
            top[2] = grid[y - 1][x + 1]; // top right
        }
    }
    // mid
    if x > 0 {
        mid[0] = grid[y][x - 1];
    }
    if x < width {
        mid[2] = grid[y][x + 1];
    }
    // bot
    if y < width {
        bot[1] = grid[y + 1][x];
        if x > 0 {
            bot[0] = grid[y + 1][x - 1];
        }
        if x < width {
            bot[2] = grid[y + 1][x + 1];
        }
    }

    let pixel_matrix: [[u8; 3]; 3] = [top, mid, bot];

    // convert matrix to index
    let mut p = 0;
    pixel_matrix.iter().flatten().for_each(|&x| {
        p = p << 1;
        if x == HASH {
            p += 1;
        }
    });

    return p;
}

fn enhance_image(
    iterations: usize,
    mut grid: Vec<Vec<u8>>,
    algo: &[u8],
    image_size: usize,
) -> Vec<Vec<u8>> {
    let mut cycles = iterations;
    let mut grid_width = grid.len();
    let mut image_width = image_size;
    while cycles > 0 {
        let mut t_grid = grid.clone();
        // My input was 9 dark pixels(0) = 1 light pixel
        // 9 light pixels (512) = 1 dark pixel
        // This causes the surrounding pixels around the image to fluctuate between light and dark
        // samlple input algorithm index 0 is dark, and last index is light
        // my input first index is light, last index is dark
        // flip filler between light and dark depending on cycle
        // unsure if this pattern holds true for all inputs
        let mut filler = 0; // 0 on odd
        if cycles % 2 == 0 {
            filler = algo.len() - 1; //511 on even
        }

        // scan the whole grid
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let p = enhance_pixel((j, i), &grid, algo[filler]);
                t_grid[i][j] = algo[p];
            }
        }

        grid = t_grid;
        image_width += 2;
        cycles -= 1;
        // print_grid(&grid);

        // resize the grid
        if image_width >= grid_width - 1 {
            grid = resize_grid(grid);
            grid_width = grid.len();
        }
    }

    return grid;
}

//TODO: rework to be similar to resize_grid
fn create_grid(inputs: &String) -> (&[u8], Vec<Vec<u8>>, usize) {
    let mut lines = inputs.lines();
    // parse
    let enhancement_algo = lines.next().unwrap().as_bytes();
    lines.next(); //skip empty line

    // use the first line to gather some info about the input
    let l = lines.next().unwrap();
    let image_width = l.len();

    // create an empty row to use as padding
    let empty_row: Vec<u8> = vec![DOT; image_width * 3]; // Vec::with_capacity(line_width * 3);

    //fill out grid
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(image_width * 3);

    // insert filler rows
    for _ in 0..image_width {
        let e = empty_row.clone();
        grid.push(e);
    }
    let pad = vec![DOT; image_width];
    // add in the the first line
    let mut t = pad.clone();
    let mut x: Vec<u8> = l.as_bytes().iter().map(|&x| x).collect();
    t.append(&mut x);
    t.append(&mut pad.clone());

    // add remainder of lines
    grid.push(t);
    for line in lines {
        let mut t = pad.clone();
        let mut x: Vec<u8> = line.as_bytes().iter().map(|&x| x).collect();
        t.append(&mut x);
        t.append(&mut pad.clone());
        grid.push(t);
    }

    // pad bottom of the grid
    for _ in 0..image_width {
        let e = empty_row.clone();
        grid.push(e);
    }

    return (enhancement_algo, grid, image_width);
}

fn resize_grid(grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let grid_width = grid[0].len();
    let new_width = grid_width * 2;
    let offset = (grid_width as f64 / 2f64).ceil() as usize;

    let mut new_grid: Vec<Vec<u8>> = vec![vec![DOT; new_width]; new_width];

    for i in 0..grid_width {
        for j in 0..grid_width {
            new_grid[i + offset][j + offset] = grid[i][j];
        }
    }

    return new_grid;
}
