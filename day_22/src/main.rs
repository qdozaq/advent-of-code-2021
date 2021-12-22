use regex::Regex;
use std::{collections::HashMap, ops::RangeInclusive};

type Cmd = (
    bool,
    RangeInclusive<isize>,
    RangeInclusive<isize>,
    RangeInclusive<isize>,
);

type Cube = HashMap<(isize, isize, isize), bool>;
fn main() {
    let input = include_str!("../input.txt");

    let cmd_regex =
        Regex::new(r"(\w+) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();
    let cmds: Vec<Cmd> = input
        .lines()
        .map(|line| {
            let caps = cmd_regex.captures(line).expect("successful capture");

            let on = caps.get(1).map(|w| w.as_str() == "on").unwrap();
            let xr = caps[2].parse::<isize>().unwrap()..=caps[3].parse::<isize>().unwrap();
            let yr = caps[4].parse::<isize>().unwrap()..=caps[5].parse::<isize>().unwrap();
            let zr = caps[6].parse::<isize>().unwrap()..=caps[7].parse::<isize>().unwrap();

            (on, xr, yr, zr)
        })
        .collect();

    let mut cube = HashMap::new();
    for cmd in cmds {
        process_cmd(cmd, &mut cube);
    }

    let mut count = 0;
    for (_, on) in cube {
        if on {
            count += 1;
        }
    }

    println!("on: {}", count);
}

fn process_cmd(cmd: Cmd, cube: &mut Cube) {
    let (on, x_range, y_range, z_range) = cmd;
    let limit = -50..=50;

    for x in x_range {
        if !limit.contains(&x) {
            continue;
        }
        for y in y_range.clone() {
            if !limit.contains(&y) {
                continue;
            }
            for z in z_range.clone() {
                if !limit.contains(&z) {
                    continue;
                }
                *cube.entry((x, y, z)).or_default() = on;
            }
        }
    }
}
