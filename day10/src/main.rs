use std::error::Error;
use std::fs;
use std::cmp::{min, max};
use std::f64::consts::PI;


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


fn get_binned(inp: &Vec<Vec<i32>>,  (x0, y0): (usize, usize)) -> Vec<Vec<((i32, i32), f64, f64)>> {
	let mut mapped = vec![];
	for (y, row) in inp.iter().enumerate() {
		for (x, v) in row.iter().enumerate() {
			if *v != 1 || (x==x0 && y == y0) {
				continue;
			}
			let (x, y) = ((x as i32 - x0 as i32), (y as i32 - y0 as i32));
			mapped.push(
				((x, y), angle_from_y_axis((x, y)), distance_from_origin((x, y)))
			);
		}
	}

	mapped.sort_by(|x, y| (x.1).partial_cmp(&y.1).unwrap());

	let mut i = 0;
	let n = mapped.len();
	let mut binned = vec![];
	while i < n {
		let mut bin = vec![mapped[i]];
		i+=1;
		while i<n && (mapped[i].1 - bin[0].1).abs() < 1e-4 {
			bin.push(mapped[i]);
			i+=1;
		}
		bin.sort_by(|x, y| (x.2).partial_cmp(&y.2).unwrap());
		binned.push(bin);
	}
	binned

}

fn part1(inp: &Vec<Vec<i32>>) -> (usize, Option<(usize, usize)>) {
	let mut max_count = ::std::usize::MIN;
	let mut cords = None;

	for (y, row) in inp.iter().enumerate() {
		for (x, v) in row.iter().enumerate() {
			if *v == 0 {
				continue;
			}
			let c = get_binned(inp,(x, y)).len();
			if c > max_count {
				max_count = c;
				cords = Some((x, y));

			}
		}
	}
	(max_count, cords)
}

fn angle_from_y_axis((x, y): (i32, i32)) -> f64 {
	let mut q = (x as f64).atan2((y*-1) as f64);
    if q < 0.0 {
        q += 2.0*PI;
    }
    q
}

fn distance_from_origin((x, y): (i32, i32)) -> f64 {
	((x*x + y*y) as f64).sqrt()
}


fn part2(inp: &Vec<Vec<i32>>, (x0, y0): (usize, usize)) -> (i32, i32) {
	let mut binned = get_binned(inp, (x0, y0));

	let mut count = 1;
	let mut last_coord = (-1, -1);

	'main_loop: while count <= 200 {
		for bin in binned.iter_mut() {
			if bin.len() > 0 {
				let next = bin.remove(0);
				last_coord = ((next.0).0 + x0 as i32, (next.0).1 + y0 as i32);
				count +=1;
				if count > 200 {
					break 'main_loop;
				}
			}
		}
	}

	last_coord
}


fn main() {
   let input = get_input("input").unwrap();

   let (part1, cords) = part1(&input);
   let (x, y) = part2(&input, cords.unwrap());
   let part2 = x*100 + y;

   assert_eq!(part1, 247);
   assert_eq!(part2, 1919);

   println!("Part 1: {}", part1);
   println!("Part 2: {}", part2);
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