use std::isize;

fn main() {
    let numbers = include_str!("../input.txt")
        .split(',')
        .map(|n| n.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    star_one(&numbers);
    star_two(&numbers);
}

fn star_one(crabs: &Vec<isize>) {
    let mut least_fuel = isize::MAX;
    for position in 0..crabs.len() {
        let mut fuel_used = 0;
        for gas in crabs {
            fuel_used += (position as isize - *gas).abs();
        }
        if fuel_used < least_fuel {
            least_fuel = fuel_used;
        }
    }
    println!("{}", least_fuel);
}

fn star_two(crabs: &Vec<isize>) {
    let mut least_fuel = isize::MAX;
    for position in 0..crabs.len() {
        let mut fuel_used = 0;
        for gas in crabs {
            let steps = (position as isize - *gas).abs();
            for fuel in 1..=steps {
                fuel_used += fuel;
            }
        }
        if fuel_used < least_fuel {
            least_fuel = fuel_used;
        }
    }
    println!("{}", least_fuel);
}
