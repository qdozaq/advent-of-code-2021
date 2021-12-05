use std::fmt::Result;

const WIDTH: usize = 5;
struct Board {
    numbers: Vec<isize>,
}

impl Board {
    fn new(numbers: Vec<isize>) -> Board {
        Board { numbers }
    }
    fn callNum(&mut self, called_number: &isize) -> bool {
        for num in self.numbers.iter_mut() {
            if num.eq(&called_number) {
                *num = -1;
            }
        }

        self.check()
    }

    fn check(&self) -> bool {
        // check rows
        for col in 0..WIDTH {}

        true
    }
}
fn main() {
    let numbers: Vec<isize> = include_str!("../test_nums.txt")
        .to_string()
        .split(",")
        .map(|n| n.parse::<isize>().unwrap())
        .collect();

    let boards_text = include_str!("../test_boards.txt");
    let boards = boards_text
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", boards);
}
