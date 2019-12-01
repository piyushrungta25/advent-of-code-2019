use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::error::Error;

fn get_inputs() -> Result<Vec<i32>, Box<dyn Error>> {
	let f = File::open("input")?;
	let buf = BufReader::new(f);
	let mut inputs: Vec<i32> = vec![];

	for i in buf.lines() {
		inputs.push(i?.trim().parse()?);
	}

	Ok(inputs)
}

fn main() {
	let inputs = match get_inputs() {
		Ok(inp) => inp,
		_ => panic!("bad input file")
	};

	let mut sum: i32 = 0;
	for i in inputs {
		let mut mass  = i;
		loop {
			let req_fuel = (mass/3)-2;
			if req_fuel > 0 {
				mass = req_fuel;
				sum += req_fuel;
			} else {
				break;
			}
		}
	}

	println!("{}", sum);



}
