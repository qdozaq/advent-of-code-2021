use std::collections::{HashMap, HashSet};

const SIDES: usize = 100;

const BOARD_SIZE: usize = 10;

fn main() {
    // star_one();
    star_two();
}

fn star_one() {
    // let (mut p1, mut p2) = (4, 8);
    let (mut p1, mut p2) = (3, 7);
    let (mut p1_points, mut p2_points) = (0, 0);

    let mut dice = 0;
    let mut rolls = 0;

    let mut turn = true;

    loop {
        let (player, points) = if turn {
            (&mut p1, &mut p1_points)
        } else {
            (&mut p2, &mut p2_points)
        };

        let mut mv = 0;
        for _ in 0..3 {
            dice = roll(dice);
            rolls += 1;
            mv += dice
        }
        *player = (*player + mv) % BOARD_SIZE;
        *points += if *player == 0 { 10 } else { *player };

        if *points >= 1000 {
            break;
        }

        // println!("mv: {} player: {} points {}", mv, *player, *points);

        turn = !turn;
    }

    let lesser = if turn { p2_points } else { p1_points };

    println!("{} * {} = {}", lesser, rolls, lesser * rolls);
}

fn roll(die: usize) -> usize {
    let mut d = die + 1;

    if d > SIDES {
        d = 1;
    }

    d
}

fn star_two() {
    let (p1_start, p2_start) = (3, 7);

    let movesets = turn_outcomes();
}

fn turn_outcomes() -> HashMap<usize, usize> {
    let mut outcomes = HashMap::new();

    roll_split(&mut outcomes, 0, 0);

    println!("{:?}", outcomes);
    outcomes
}

fn roll_split(outcomes: &mut HashMap<usize, usize>, total: usize, rolls: usize) {
    if rolls >= 3 {
        *outcomes.entry(total).or_insert(1) += 1;
        return;
    }

    roll_split(outcomes, total + 1, rolls + 1);
    roll_split(outcomes, total + 2, rolls + 1);
    roll_split(outcomes, total + 3, rolls + 1);
}
