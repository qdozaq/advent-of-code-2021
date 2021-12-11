use std::{collections::VecDeque, convert::Infallible, string::ParseError};

fn main() {
    let lines = include_str!("../test.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    star_one(&lines);
}

fn star_one(lines: &[Vec<char>]) {
    for line in lines {}
}

fn process_line(line: &[char]) -> Result<Vec<char>, ParseError> {
    let pairs = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

    let mut close_stack = line.to_vec();
    close_stack.reverse();

    let mut open_stack = Vec::new();
    open_stack.push(close_stack.pop().unwrap());

    loop {
        if close_stack.is_empty() {
            break;
        }

        let pair = (*open_stack.last().unwrap(), *close_stack.last().unwrap());

        if pairs.contains(&pair) {
            open_stack.pop();
            close_stack.pop();
        }

        open_stack.push(close_stack.pop().unwrap());

        let peek = close_stack.last().unwrap();
        for (_, c) in pairs {
            if c == *peek {
                println!("corrupt");
                return Err(Infallible);
            }
        }

        if pairs.contains(close_stack.last().unwrap()) {}
    }

    Ok(open_stack)
}
