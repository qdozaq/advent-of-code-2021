use regex::Regex;
#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

type Line = (Point, Point);

const MAP_SIZE: usize = 1000;
fn main() {
    let re = Regex::new(r"(\d+),(\d+).* (\d+),(\d+)").unwrap();
    let input = include_str!("../input.txt");
    let mut lines: Vec<Line> = Vec::new();
    for line in input.lines() {
        let cap = re.captures(line).map(|c| {
            c.iter()
                .map(|n| n.unwrap().as_str().parse::<usize>().unwrap_or_default())
                .collect::<Vec<_>>()
        });

        let cap = match cap {
            Some(v) => v,
            None => continue,
        };

        let p1 = Point {
            x: *cap.get(1).unwrap(),
            y: *cap.get(2).unwrap(),
        };
        let p2 = Point {
            x: *cap.get(3).unwrap(),
            y: *cap.get(4).unwrap(),
        };

        lines.push((p1, p2));
    }

    star_one(&lines);
    star_two(&lines);
}

fn star_one(lines: &Vec<Line>) {
    // build map
    let mut map = vec![0; MAP_SIZE * MAP_SIZE];

    for (p1, p2) in lines {
        if p1.x == p2.x {
            let x = p1.x;

            let (lower, upper) = lower_upper(p1.y, p2.y);

            for y in lower..=upper {
                map[MAP_SIZE * x + y] += 1;
            }
        }
        if p1.y == p2.y {
            let y = p1.y;
            let (lower, upper) = lower_upper(p1.x, p2.x);
            for x in lower..=upper {
                map[MAP_SIZE * x + y] += 1;
            }
        }
    }

    let mut count = 0;
    for point in map {
        if point > 1 {
            count += 1;
        }
    }

    println!("star one count {}", count);
}

fn star_two(lines: &Vec<Line>) {
    // build map
    let mut map = vec![0; MAP_SIZE * MAP_SIZE];

    for (p1, p2) in lines {
        if p1.x == p2.x {
            let x = p1.x;

            let (lower, upper) = lower_upper(p1.y, p2.y);

            for y in lower..=upper {
                map[MAP_SIZE * x + y] += 1;
            }
            continue;
        }
        if p1.y == p2.y {
            let y = p1.y;
            let (lower, upper) = lower_upper(p1.x, p2.x);
            for x in lower..=upper {
                map[MAP_SIZE * x + y] += 1;
            }
            continue;
        }

        // is diagonal
        let xincreasing = if p1.x < p2.x { true } else { false };
        let yincreasing = if p1.y < p2.y { true } else { false };
        let mut x = p1.x;
        let mut y = p1.y;

        while (xincreasing && x <= p2.x) || (!xincreasing && x >= p2.x) {
            map[MAP_SIZE * x + y] += 1;
            if xincreasing {
                x += 1;
            } else {
                x -= 1;
            };
            if yincreasing {
                y += 1;
            } else {
                y -= 1;
            };
        }
    }

    let mut count = 0;
    for point in map {
        if point > 1 {
            count += 1;
        }
    }

    println!("star two count {}", count);
}

fn lower_upper(a: usize, b: usize) -> (usize, usize) {
    match a < b {
        true => (a, b),
        false => (b, a),
    }
}

fn print_map(map: &Vec<i32>) {
    let mut s = String::new();
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            let p = map[MAP_SIZE * x + y];
            match p {
                0 => s.push_str("."),
                _ => s.push_str(p.to_string().as_str()),
            }
        }
        s.push_str("\n");
    }
    println!("{}", s);
}
