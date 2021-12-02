type Cmd<'a> = (&'a str, u32);
fn main() {
    let input: String = include_str!("../input.txt").to_string();
    let lines = input.split('\n');
    let commands:Vec<Vec<&str>> = lines.map(|l| l.split(' ').collect()).collect();
    let commands:Vec<Cmd> = commands.into_iter().map(|c| (c[0], c[1].parse::<u32>().unwrap_or_default())).collect();
    star_one(&commands);
    star_two(&commands);
}

fn star_one(commands: &Vec<Cmd>) {
    let mut d = 0;
    let mut h = 0;
    for c in commands {
        match c.0 {
            "forward" => h += c.1,
            "down" => d += c.1,
            "up" => d -= c.1,
            _ => println!("=(")
        }
    }

    let position = d * h;

    println!("{}", position);
}

fn star_two(commands: &Vec<Cmd>) {
    let mut d = 0;
    let mut h = 0;
    let mut a = 0;
    for c in commands {
        match c.0 {
            "forward" => {h += c.1;
                d += a * c.1
            },
            "down" => a += c.1,
            "up" => a -= c.1,
            _ => println!("=(")
        }
    }

    let position = d * h;

    println!("{}", position);
}
