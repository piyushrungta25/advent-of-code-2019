use std::error::Error;
use std::fs;
use std::collections::{HashMap, VecDeque};

fn parse_input() -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let buf: String = fs::read_to_string("input")?;
    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();

    buf.trim().split("\n").for_each(|line| {
    	let mut objs = line.trim().split(")").map(str::to_string);
        let parent = objs.next().unwrap();
        let child = objs.next().unwrap();
        orbits.entry(parent).or_default().push(child);

    });

    Ok(orbits)
}

fn parse_input_reverse() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let buf: String = fs::read_to_string("input")?;
    let mut orbits: HashMap<String, String> = HashMap::new();

    buf.trim().split("\n").for_each(|line| {
    	let mut objs = line.trim().split(")").map(str::to_string);
        let parent = objs.next().unwrap();
        let child = objs.next().unwrap();
        orbits.entry(child).or_insert(parent);

    });

    Ok(orbits)
}

fn orbit_count_checksum(m: HashMap<String, Vec<String>>) -> u32 {
	let mut queue = VecDeque::new();
	queue.push_back(("COM", 1));
	let mut count: u32 = 0;


	while queue.len() > 0 {
		let (parent, cur_count) = queue.pop_back().unwrap();
		if let Some(childs) = m.get(parent) {
			for child in childs {
				count += cur_count;
				queue.push_back((child, cur_count+1));
			}
		}
	}
	count
}

fn parents<'a>(m: &'a HashMap<String, String>, from: &'static str) -> Vec<&'a str> {
	let mut cur = from;
	let mut parents = Vec::new();
	while cur != "COM" {
		if let Some(parent) = m.get(cur) {
			parents.push(parent.as_str());
			cur = parent;
		}

	}
	parents
}

fn first_common_element(from: &Vec<&str>, to: &Vec<& str>) -> Option<(usize, usize)> {
	for (i, f) in from.iter().enumerate() {
		for (j, t) in to.iter().enumerate() {
			if f == t {
				return Some((i, j));
			}
		}
	}
	None
}

fn min_orbital_jumps(m: HashMap<String, String>, from: &'static str, to: &'static str) -> usize {
	let from_parents = parents(&m, from);
	let to_parents = parents(&m, to);

	let (i, j) = first_common_element(&from_parents, &to_parents).unwrap();
	i+j
}

fn main() {
	let m = parse_input().unwrap();
    println!("Part 1: {:?}",orbit_count_checksum(m));

    let m = parse_input_reverse().unwrap();
    println!("Part 2: {:?}",min_orbital_jumps(m, "YOU", "SAN"));
}
