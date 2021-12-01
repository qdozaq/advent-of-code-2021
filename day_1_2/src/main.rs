use std::fs::File;
use std::io::{BufReader, BufRead};
use std::u32::MAX;
use std::collections::VecDeque;

fn main() {
    let reader = BufReader::new(File::open("input.txt").expect("Cannot open"));

    let mut count = 0;
    let mut prev: u32 = MAX;
    let mut queue = VecDeque::with_capacity(3);
    for line in reader.lines() {
        let num: u32 = match line.unwrap().parse() {
            Ok(num) => num,
            Err(_) => panic!("NaN")
        };

        queue.push_back(num);

        if queue.len() >= 3{
            let num = queue.iter().sum();
            if num > prev {
                count = count + 1;
            }
            prev = num;

            queue.pop_front();
        }

    }

    println!("{}", count);    
}
