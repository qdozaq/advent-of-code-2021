use petgraph::Graph;
use std::collections::HashMap;

fn main() {
    let lines = include_str!("../test.txt").lines().collect::<Vec<_>>();
}

enum Size {
    Big,
    Small,
}

struct Cave<'a> {
    size: Size,
    name: String,
    connections: Vec<&'a Cave<'a>>,
}

impl Cave<'static> {
    fn new(name: String) -> Cave<'static> {
        let size = if name.chars().all(|c| c.is_uppercase()) {
            Size::Big
        } else {
            Size::Small
        };
        Cave {
            name: name,
            size,
            connections: Vec::new(),
        }
    }
}

fn build_graph(lines: Vec<&str>) {
    let mut graph: HashMap<&str, Cave<'static>> = HashMap::new();

    for line in lines {
        let mut split = line.split('-');
        let (c1, c2) = (split.next().unwrap(), split.next().unwrap());

        if !graph.contains_key(c1) {
            graph.insert(c1, Cave::new(c1.to_string()));
        }

        if !graph.contains_key(c2) {
            graph.insert(c2, Cave::new(c2.to_string()));
        }

        let c1 = graph.get_mut(c1).unwrap();
        let c2 = graph.get_mut(c2).unwrap();

        c1.connections.push(c2);
        c2.connections.push(c1);
    }
}
