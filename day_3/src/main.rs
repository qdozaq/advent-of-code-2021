use std::num::ParseIntError;

const BIT_SIZE: usize = 12;

fn main() {
    let input: String = include_str!("../input.txt").to_string();
    let lines: Vec<&str> = input.lines().collect();

    star_one(&lines);
    star_two(&lines);
}

fn star_one(lines: &Vec<&str>) -> Result<(), ParseIntError> {
    let mut lean = vec![0; BIT_SIZE];
    for line in lines {
        let mut pos = 0;
        for c in line.chars() {
            match c {
                '0' => lean[pos] -= 1,
                '1' => lean[pos] += 1,
                _ => panic!("why =("),
            }
            pos += 1;
        }
    }

    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();

    for pos in lean {
        if pos > 0 {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
    }

    let gamma = isize::from_str_radix(&gamma, 2)?;
    let epsilon = isize::from_str_radix(&epsilon, 2)?;

    let solution = gamma * epsilon;
    println!("{}", solution);

    Ok(())
}

fn get_lean_at(n: usize, lines: &Vec<&str>) -> char {
    let mut lean: isize = 0;
    for line in lines {
        match line.chars().nth(n) {
            Some(c) => match c {
                '0' => lean -= 1,
                '1' => lean += 1,
                _ => panic!("no =("),
            },
            None => panic!("why =("),
        }
    }

    if lean >= 0 {
        '1'
    } else {
        '0'
    }
}

fn star_two(lines: &Vec<&str>) -> Result<(), ParseIntError> {
    let oxygen = get_rating(lines, true)?;
    let co2 = get_rating(lines, false)?;

    let solution = oxygen * co2;
    println!("{}", solution);
    Ok(())
}

fn get_rating(lines: &Vec<&str>, is_oxygen: bool) -> Result<isize, ParseIntError> {
    let mut filtered = lines.clone();

    for i in 0..BIT_SIZE {
        let lean = get_lean_at(i, &filtered);
        filtered = filtered
            .into_iter()
            .filter(|line| {
                if is_oxygen {
                    line.chars().nth(i) == Some(lean)
                } else {
                    line.chars().nth(i) != Some(lean)
                }
            })
            .collect::<Vec<&str>>();
        if filtered.len() == 1 {
            break;
        }
    }

    let rating = filtered[0];
    let rating = isize::from_str_radix(&rating, 2)?;
    Ok(rating)
}
