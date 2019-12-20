use std::error::Error;
use std::fs;
use std::f64::consts::PI;


fn get_input(file: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    Ok(fs::read_to_string(file)?
        .split("\n")
        .map(|x| x.chars().map(|x| if x == '#' { 1 } else { 0 }).collect())
        .collect())
}


fn angle_from_y_axis((x, y): (i32, i32)) -> f64 {
	// we invert the y-axis here
	let mut q = (x as f64).atan2((y*-1) as f64);
    if q < 0.0 {
        q += 2.0*PI;
    }
    q
}

fn distance_from_origin((x, y): (i32, i32)) -> f64 {
	((x*x + y*y) as f64).sqrt()
}

// calculates the angle every point makes with y-axis when origin is shifted to (x0, y0)
// bins the point on same length and sort the points in each bin by their distance from (x0, y0)
fn get_binned(inp: &Vec<Vec<i32>>,  (x0, y0): (usize, usize)) -> Vec<Vec<((i32, i32), f64, f64)>> {
	let mut mapped = vec![];

	for (y, row) in inp.iter().enumerate() {
		for (x, v) in row.iter().enumerate() {
			if *v != 1 || (x==x0 && y == y0) {
				continue;
			}

			// shift the origin to (x0, y0)
			let (x, y) = ((x as i32 - x0 as i32), (y as i32 - y0 as i32));
			mapped.push(
				((x, y), angle_from_y_axis((x, y)), distance_from_origin((x, y)))
			);
		}
	}

	// sort by the angle the make with the y-axis
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

		// sort the bin by distance from origin
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


fn part2(inp: &Vec<Vec<i32>>, (x0, y0): (usize, usize)) -> (i32, i32) {
	let mut binned = get_binned(inp, (x0, y0));

	let mut count = 1;
	let mut last_coord = (-1, -1);


	// for the sweeping motion, we continiously loop over the bins and take the
	// first value from each bin
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
