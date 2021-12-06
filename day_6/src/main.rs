use std::collections::VecDeque;

fn main() {
    let numbers = include_str!("../input.txt")
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    simulate(&numbers, 256);
}

fn simulate(initial: &Vec<usize>, days: usize) {
    let mut queue: VecDeque<Option<usize>> = VecDeque::from(vec![None; 9]);
    // init
    for days_left in initial {
        let num_fish = queue.get_mut(*days_left).unwrap();
        *num_fish = match num_fish {
            None => Some(1),
            Some(fish) => Some(*fish + 1),
        }
    }

    for _ in 0..days {
        if let Some(fish) = queue.pop_front() {
            match fish {
                Some(num_fish) => {
                    queue.push_back(Some(num_fish));
                    if let Some(fish_at_6) = queue.get_mut(6) {
                        *fish_at_6 = match fish_at_6 {
                            Some(f) => Some(*f + num_fish),
                            None => Some(num_fish),
                        }
                    }
                }
                None => queue.push_back(None),
            }
        }
    }

    let mut total = 0;
    for num_fish in queue.into_iter() {
        if let Some(f) = num_fish {
            total += f;
        }
    }

    println!("Number of fish after {} days: {}", days, total);
}
