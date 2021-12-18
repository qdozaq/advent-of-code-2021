use std::ops::RangeInclusive;

fn main() {
    let (x_range, y_range) = (20..=30, -10..=-5);
    // let (x_range, y_range) = (287..=309, -76..=-48);

    // star_one(&x_range, &y_range);
    star_two(&x_range, &y_range);
}

fn min_vx(range: &RangeInclusive<i32>) -> i32 {
    let mut n = 1;

    loop {
        let summation = n * (n + 1) / 2;
        if range.contains(&summation) {
            break;
        }
        n += 1;

        if n > *range.start() {
            panic!("This shouldn't happen")
        }
    }

    n
}

// this is aweful. if you see this i'm sorry
fn star_one(x_range: &RangeInclusive<i32>, y_range: &RangeInclusive<i32>) {
    let (vx, mut vy) = (min_vx(x_range), 0);

    println!("min x {}", vx);

    let mut highest = vy;
    let steps = vx;

    let loops = 2000;

    // println!("steps {}", steps);

    for _ in 0..loops {
        let mut initial_vy = vy;
        let mut local_highest = 0;
        let mut y: i32 = 0;
        // println!("{}", initial_vy);

        loop {
            // println!("steping {}", y);

            if y > local_highest {
                local_highest = y;
            }

            y += initial_vy;
            initial_vy -= 1;

            if y < *y_range.end() {
                break;
            }
        }
        // println!("end {}", y);

        // println!("local h {}", local_highest);

        if y_range.contains(&y) {
            // println!("contains {}", y);
            if local_highest > highest {
                // println!("setting local{}", local_highest);
                highest = local_highest;
                // println!("h {}", highest);
            }
        }

        vy += 1;
    }

    println!("highest: {}, with ({},{})", highest, vx, vy);
}

fn min_max_vx(range: &RangeInclusive<i32>) -> (i32, i32) {
    let mut n = 1;

    loop {
        let summation = n * (n + 1) / 2;
        if range.contains(&summation) {
            break;
        }
        n += 1;

        if n > *range.start() {
            panic!("This shouldn't happen")
        }
    }

    let min = n;

    (min, *range.end())
}

fn value_at_step(step: i32, initial: i32) -> i32 {
    let mut i = initial;
    let mut n = 0;
    for _ in 0..step {
        n += i;
        i -= 1
    }

    n
}

fn star_two(x_range: &RangeInclusive<i32>, y_range: &RangeInclusive<i32>) {
    let (min_vx, max_vx) = min_max_vx(x_range);

    let mut count = 0;
    let mut vy = *y_range.end();
    let loops = 100;
    for _ in 0..loops {
        let mut initial_vy = vy;
        let mut y = 0;

        let mut steps = 1;

        loop {
            y += initial_vy;

            if y < *y_range.end() {
                break;
            }

            if y_range.contains(&y) {
                for i in min_vx..=max_vx {
                    let x = value_at_step(steps, i);
                    if x_range.contains(&x) {
                        println!("x: {}, y: {}", x, y);
                        count += 1;
                    }
                }
            }

            steps += 1;
            initial_vy -= 1;
        }

        vy += 1
    }

    println!("{}", count);
}
