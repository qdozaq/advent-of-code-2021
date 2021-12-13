use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

type Dot = (isize, isize);

fn main() {
    let (dots, folds) = include_str!("../input.txt")
        .split("\n\n")
        .collect_tuple()
        .unwrap();

    let mut dots = dots
        .lines()
        .map(|pair| {
            pair.split(',')
                .map(|n| n.parse::<isize>().unwrap())
                .collect_tuple::<Dot>()
                .unwrap()
        })
        .collect::<HashSet<_>>();

    let re = Regex::new(r"(\w+)=(\d+)").unwrap();
    let folds = folds
        .lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            (
                cap.get(1).map(|c| c.as_str()).unwrap(),
                cap.get(2)
                    .map(|n| n.as_str().parse::<isize>().unwrap())
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();

    for fold in folds {
        dots = do_fold(fold, dots);
    }

    println!("size: {}", dots.len());
    print_map(&dots);
}

fn do_fold(fold: (&str, isize), dots: HashSet<Dot>) -> HashSet<Dot> {
    let mut new_dots = HashSet::new();
    for dot in dots {
        let reflected_dot = reflect(fold, dot);
        new_dots.insert(reflected_dot);
    }

    new_dots
}

fn reflect(fold: (&str, isize), dot: Dot) -> Dot {
    let (axis, line) = fold;
    match axis {
        "x" => {
            if dot.0 > line {
                let diff = (dot.0 - line).abs();
                (dot.0 - diff * 2, dot.1)
            } else {
                dot
            }
        }
        "y" => {
            if dot.1 > line {
                let diff = (dot.1 - line).abs();
                (dot.0, dot.1 - diff * 2)
            } else {
                dot
            }
        }
        _ => panic!("bad axis"),
    }
}

fn print_map(dots: &HashSet<Dot>) {
    let (minw, minh, maxw, maxh) = dots
        .iter()
        .fold((isize::MAX, isize::MAX, 0, 0), |acc, cur| {
            let (mut minw, mut minh, mut w, mut h) = acc;
            if cur.0 > w {
                w = cur.0;
            }
            if cur.0 < minw {
                minw = cur.0;
            }
            if cur.1 > h {
                h = cur.1
            }
            if cur.1 < minh {
                minh = cur.1;
            }
            (minw, minh, w, h)
        });

    let mut s = String::new();

    for col in minh..=maxh {
        for row in minw..=maxw {
            if dots.contains(&(row, col)) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }

    println!("{}", s);
}
