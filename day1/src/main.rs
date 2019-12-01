use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::error::Error;

fn get_inputs() -> Result<Vec<u64>, Box<dyn Error>> {
	let f = File::open("input")?;
	let buf = BufReader::new(f);
	let mut inputs: Vec<u64> = vec![];

	for i in buf.lines() {
		inputs.push(i?.trim().parse()?);
	}

	Ok(inputs)
}

fn get_mass(i: u64) -> u64 {
	(i/3).saturating_sub(2)
}

fn get_total_mass(i: u64) -> u64 {
	let mut mass = i;
	let mut total_fuel = 0;
	loop {
		let req_fuel = get_mass(mass);
		if req_fuel > 0 {
			total_fuel += req_fuel;
			mass = req_fuel;
		} else {
			break
		}
	}

	total_fuel
}

fn main() {
	let inputs = match get_inputs() {
		Ok(inp) => inp,
		_ => panic!("bad input file")
	};

	println!("Part 1: {}", inputs.iter().map(|x| get_mass(*x)).sum::<u64>());
	println!("Part 2: {}", inputs.iter().map(|x| get_total_mass(*x)).sum::<u64>());

}
