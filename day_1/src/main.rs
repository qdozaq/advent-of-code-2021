use std::fs::File;
use std::io::{BufReader, BufRead};
use std::u32::MAX;

fn main() {
    let reader = BufReader::new(File::open("input.txt").expect("Cannot open"));

    let mut count = 0;
    let mut prev: u32 = MAX;
    for line in reader.lines() {
        let num: u32 = match line.unwrap().parse() {
            Ok(num) => num,
            Err(_) => panic!("NaN")
        };

        if num > prev {
            count = count + 1;
        }

        prev = num;
    }

    println!("{}", count);    
}
