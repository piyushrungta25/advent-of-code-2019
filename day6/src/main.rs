use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs;

fn parse_input<'a>(input: &'a str) -> Result<(HashMap<&'a str, Vec<&'a str>>, HashMap<&'a str, &'a str>) , Box<dyn Error>> {
    let mut orbits: HashMap<&str, Vec<&str>> = HashMap::with_capacity(2000);
    let mut orbits_rev: HashMap<&str, &str> = HashMap::with_capacity(2000);

    input.trim().split("\n").for_each(|line| {
        let mut objs = line.trim().split(")");
        let parent = objs.next().unwrap();
        let child = objs.next().unwrap();
        orbits.entry(parent).or_default().push(child);
        orbits_rev.entry(child).or_insert(parent);
    });

    Ok((orbits, orbits_rev))
}

fn orbit_count_checksum(m: HashMap<&str, Vec<&str>>) -> u32 {
    let mut queue = VecDeque::new();
    queue.push_back(("COM", 1));
    let mut count: u32 = 0;

    while queue.len() > 0 {
        let (parent, cur_count) = queue.pop_back().unwrap();
        if let Some(childs) = m.get(parent) {
            for child in childs {
                count += cur_count;
                queue.push_back((child, cur_count + 1));
            }
        }
    }
    count
}

fn parents<'a>(m: &'a HashMap<&'a str, &'a str>, from: &'static str) -> Vec<&'a str> {
    let mut cur = from;
    let mut parents = Vec::new();
    while cur != "COM" {
        if let Some(parent) = m.get(cur) {
            parents.push(*parent);
            cur = parent;
        }
    }
    parents
}

fn first_common_element(from: Vec<&str>, to: Vec<&str>) -> Option<(usize, usize)> {
    let mut hm: HashMap<&str, usize> = HashMap::new();
    for (i, item) in to.into_iter().enumerate() {
        hm.insert(item, i);
    }
    for (i, f) in from.iter().enumerate() {
        if let Some(j) = hm.get(f) {
            return Some((i, *j));

        }
    }
    None
}

fn min_orbital_jumps(m: HashMap<&str, &str>, from: &'static str, to: &'static str) -> usize {
    let from_parents = parents(&m, from);
    let to_parents = parents(&m, to);

    let (i, j) = first_common_element(from_parents, to_parents).unwrap();
    i + j
}

fn main() {
    let inp = fs::read_to_string("input").unwrap();
    let (m1, m2) = parse_input(&inp).unwrap();
    println!("Part 1: {:?}", orbit_count_checksum(m1));
    println!("Part 2: {:?}", min_orbital_jumps(m2, "YOU", "SAN"));
}
