use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::cmp::{min, max};


fn get_input(file: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    Ok(fs::read_to_string(file)?
        .split("\n")
        .map(|x| x.chars().map(|x| if x == '#' { 1 } else { 0 }).collect())
        .collect())
}


fn is_reachable(inp: &Vec<Vec<i32>>, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
	let (x1, y1, x2, y2) = (x1 as i64, y1 as i64, x2 as i64, y2 as i64);
	let is_on_line = |x, y| (y-y1)*(x2-x1) == (y2-y1) * (x-x1);

	for x in min(x1, x2)..=max(x1, x2) {
		for y in min(y1, y2)..=max(y1, y2) {
			if (x==x1 && y==y1) || (x==x2 && y==y2) || inp[x as usize][y as usize] == 0 {
				continue;
			}

			if is_on_line(x, y) {
				return false;
			}
		}
	}
	true
}

fn get_all_reachable(inp: &&Vec<Vec<i32>>, x1: usize, y1: usize) -> usize {
	let mut count  = 0;

	for (y, row) in inp.iter().enumerate() {
		for (x, v) in row.iter().enumerate() {
			if *v == 0 || (x==x1 && y==y1) {
				continue;
			}
			if is_reachable(&inp, x1, y1, x, y) {
				count+=1;
			}
		}
	}

	count
}

fn part1(inp: &Vec<Vec<i32>>) -> (usize, Option<(usize, usize)>) {
	let mut max_count = ::std::usize::MIN;
	let mut cords = None;

	for (y, row) in inp.iter().enumerate() {
		for (x, v) in row.iter().enumerate() {
			if *v == 0 {
				continue;
			}
			let c = get_all_reachable(&inp, x, y);
			if c > max_count {
				max_count = c;
				cords = Some((x, y));

			}
		}
	}
	(max_count, cords)
}



fn main() {
   let input = get_input("input").unwrap();
   let (c, cords) = part1(&input);
   println!("Part 1: {}", c);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_one() {
		let (x1, y1, x2, y2) = (3, 4, 1, 0);
		let is_on_line = |x, y| (y-y1)*(x2-x1) == (y2-y1) * (x-x1);
		assert!(is_on_line(2,2));
	}

	#[test]
	fn two() {
	   let input = get_input("input1").unwrap();
	   assert!(!is_reachable(&input, 3, 4, 1, 0));
	}
}