const GRID_SIZE: usize = 10;

fn main() {
    let octopuses = include_str!("../input.txt")
        .replace('\n', "")
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    println!("{:?}", octopuses);
    star_one(&octopuses);
    star_two(&octopuses);
}

fn get_xy(i: usize) -> (usize, usize) {
    (i % GRID_SIZE, i / GRID_SIZE)
}

const ADJ_PAIRS: [(isize, isize); 9] = [
    (0, 0),
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

fn get_adj(i: usize) -> Vec<usize> {
    let mut adj_coords = Vec::new();
    let (x, y) = get_xy(i);

    for pair in ADJ_PAIRS {
        let (x, y) = (x as isize + pair.0, y as isize + pair.1);
        if x >= 0 && x < GRID_SIZE as isize && y >= 0 && y < GRID_SIZE as isize {
            let coord = y as usize * GRID_SIZE + x as usize;
            adj_coords.push(coord);
        }
    }

    adj_coords
}

fn flash_adj(octs: &mut [u32], i: usize) {
    let adj_coords = get_adj(i);
    // increment to 11 to signify we've been here before
    octs[i] += 1;

    for coord in adj_coords {
        if octs[coord] > 9 {
            continue;
        }

        octs[coord] += 1;

        if octs[coord] > 9 {
            flash_adj(octs, coord);
        }
    }
}

fn flash(octs: &mut [u32]) -> u32 {
    for i in 0..octs.len() {
        if octs[i] > 10 {
            continue;
        }
        if octs[i] > 9 {
            flash_adj(octs, i)
        }
    }

    let mut flashes = 0;
    for energy in octs.iter_mut() {
        if *energy > 9 {
            *energy = 0;
            flashes += 1;
        }
    }

    flashes
}

fn star_one(octopuses: &[u32]) {
    let steps = 100;

    let mut octs = octopuses.to_vec();

    let mut flashes = 0;

    for _ in 0..steps {
        // increment
        octs = octs.iter().map(|o| o + 1).collect();

        // check for flashes
        flashes += flash(&mut octs);
    }

    println!("flashes: {}", flashes);
}

fn star_two(octopuses: &[u32]) {
    let mut octs = octopuses.to_vec();

    let mut flashes = 0;

    let mut steps = 0;
    loop {
        // increment
        octs = octs.iter().map(|o| o + 1).collect();

        // check for flashes
        flash(&mut octs);

        steps += 1;

        if octs.iter().all(|&o| o == 0) {
            break;
        }
    }

    println!("sync at: {}", steps);
}

fn print_map(map: &[u32]) {
    let mut s = String::new();
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let p = map[GRID_SIZE * y + x];
            s.push_str((p.to_string() + " ").as_str());
        }
        s.push('\n');
    }
    println!("{}", s);
}
