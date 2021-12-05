use std::fmt::Result;

const SIZE: usize = 5;

#[derive(Debug)]
struct Board {
    numbers: Vec<isize>,
    bingo: bool,
}

impl Board {
    fn new(numbers: Vec<isize>) -> Board {
        Board {
            numbers,
            bingo: false,
        }
    }
    fn callNum(&mut self, called_number: isize) -> bool {
        for num in self.numbers.iter_mut() {
            if (*num).eq(&called_number) {
                *num = -1;
            }
        }

        self.check()
    }

    fn check(&mut self) -> bool {
        // check rows
        for col in 0..SIZE {
            let mut check = 0;
            for row in 0..SIZE {
                if self.numbers[SIZE * row + col] == -1 {
                    check += 1;
                }
            }
            if check == SIZE {
                self.bingo = true;
                return true;
            }
        }

        for row in 0..SIZE {
            let mut check = 0;
            for col in 0..SIZE {
                if self.numbers[SIZE * row + col] == -1 {
                    check += 1;
                }
            }
            if check == SIZE {
                self.bingo = true;
                return true;
            }
        }

        return false;
    }
}
fn main() {
    let numbers: Vec<isize> = include_str!("../nums.txt")
        .to_string()
        .split(",")
        .map(|n| n.parse::<isize>().unwrap())
        .collect();

    let boards_text = include_str!("../boards.txt");
    let mut boards = boards_text
        .split("\n\n")
        .map(|section| {
            // map sections of numbers to a board
            let board_nums = section
                .lines()
                .map(|line| {
                    let mut line = line.chars().filter(|l| *l != '\n').collect::<String>();
                    line.split_ascii_whitespace()
                        .map(|n| n.parse::<isize>().unwrap())
                        .collect::<Vec<isize>>()
                })
                .flatten()
                .collect::<Vec<_>>();
            Board::new(board_nums)
        })
        .collect::<Vec<_>>();

    'outer: for n in numbers {
        for board in boards.iter_mut() {
            if board.callNum(n) {
                let score: isize = board.numbers.iter().filter(|n| **n >= 0).sum();
                let score = score * n;
                println!("{}", score);
                break 'outer;
            }
        }
    }

    // println!("{:?}", boards);
}
