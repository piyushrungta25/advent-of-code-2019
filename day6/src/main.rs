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

fn main() {
	let m = parse_input().unwrap();
    println!("Part 1: {:?}",orbit_count_checksum(m));
}
