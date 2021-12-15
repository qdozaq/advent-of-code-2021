use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;
fn main() {
    let (polymer, pairs) = include_str!("../test.txt")
        .split("\n\n")
        .collect_tuple()
        .unwrap();

    let pair_regex = Regex::new(r"(\w{2}) -> (\w)").unwrap();
    let pairs = pairs
        .lines()
        .map(|line| {
            let caps = pair_regex.captures(line).unwrap();
            (
                caps.get(1).map(|c| c.as_str()).unwrap(),
                caps.get(2).map(|c| c.as_str()).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let pairs = premap_pairs(&pairs);

    // println!("{}", polymer);
    // println!("{:?}", pairs);

    let mut polymer = polymer.to_string();

    // println!("{}", polymer);
    let steps = 40;
    for _ in 0..steps {
        polymer = process(&polymer, &pairs);
        // println!("{}", polymer);
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in polymer.chars() {
        *counts.entry(c).or_insert(1) += 1;
    }
    println!("{:?}", counts);

    let mut least_common = usize::MAX;
    let mut most_common = 0;

    for (_, v) in counts {
        if v < least_common {
            least_common = v;
        }

        if v > most_common {
            most_common = v;
        }
    }

    let answer = most_common - least_common;
    println!("{}", answer);
}

fn premap_pairs(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    let mut mapped = HashMap::new();

    for (pattern, insert) in pairs {
        let mut end_pattern = pattern.to_string();
        end_pattern.insert(1, insert.chars().next().unwrap());
        mapped.insert(pattern.to_string(), end_pattern);
    }

    mapped
}

fn process(polymer: &str, pairs: &HashMap<String, String>) -> String {
    let mut new_polymer = String::new();

    for i in 1..polymer.len() {
        let p = polymer[i - 1..=i].to_string();
        if let Some(s) = pairs.get(&p) {
            new_polymer.push_str(s);
        } else {
            new_polymer.push_str(&p);
        }

        if i != polymer.len() - 1 {
            new_polymer.pop();
        }
    }

    new_polymer
}
