use itertools::Itertools;
use std::collections::VecDeque;

type Image = VecDeque<VecDeque<char>>;
fn main() {
    let (algo, input) = include_str!("../input.txt")
        .split("\n\n")
        .into_iter()
        .collect_tuple()
        .unwrap();

    let mut img: Image = input.lines().map(|line| line.chars().collect()).collect();

    img = pad(img);

    for _ in 0..50 {
        img = pad(img);
        img = expand(img, &algo);
    }

    print_img(&img);

    let count = img.into_iter().flatten().fold(0, |acc, c| match c {
        '#' => acc + 1,
        _ => acc,
    });

    println!("lit: {}", count);
}

fn expand(img: Image, algo: &str) -> Image {
    let mut new_img: Image =
        VecDeque::from(vec![VecDeque::from(vec!['.'; img[0].len()]); img.len()]);
    for row in 0..img.len() {
        for col in 0..img[0].len() {
            let b = get_binary(&img, row, col);
            let px = algo.chars().nth(b).expect("should have a filtered pixel");
            new_img[row][col] = px;
        }
    }

    new_img
}

fn get_binary(img: &Image, row: usize, col: usize) -> usize {
    let mut s = String::new();
    let void_char = void_char(&img);
    for r in row as isize - 1..=row as isize + 1 {
        for c in col as isize - 1..=col as isize + 1 {
            let px = img
                .get(r as usize)
                .map_or(&void_char, |v| v.get(c as usize).unwrap_or(&void_char));
            s.push(match px {
                '#' => '1',
                '.' => '0',
                _ => panic!("bad image input"),
            });
        }
    }

    if s.len() < 9 {
        panic!("what {}", s);
    }

    usize::from_str_radix(s.as_str(), 2).expect("should be a number")
}

fn void_char(img: &Image) -> char {
    match img[0].iter().all(|c| c.eq(&'#')) {
        true => '#',
        false => '.',
    }
}

fn pad(mut img: Image) -> Image {
    let pad = 1;
    let height = img.len();

    let void_char = void_char(&img);

    for _ in 0..pad {
        img.push_front(VecDeque::from(vec![void_char; height]));
        img.push_back(VecDeque::from(vec![void_char; height]));
    }

    for row in img.iter_mut() {
        for _ in 0..pad {
            row.push_front(void_char);
            row.push_back(void_char);
        }
    }

    img
}

fn print_img(img: &Image) {
    let mut output = String::new();

    for line in img {
        for char in line {
            output.push(*char);
        }
        output.push('\n');
    }

    println!("{}", output);
}
