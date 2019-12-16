use std::iter::repeat;
use std::fs;


fn iterator_for(times: usize, len: usize) -> impl Iterator<Item=i64> {
	repeat(0).take(times).chain(repeat(1).take(times)).chain(repeat(0).take(times)).chain(repeat(-1).take(times)).cycle().skip(1).take(len)
}

fn get_input() -> Vec<i64> {
	let input = fs::read_to_string("input").unwrap();
	input.chars().map(|x| x.to_digit(10).unwrap() as i64).collect::<Vec<i64>>()
}


fn transform_phase(input: Vec<i64>, num: usize) -> String {
	let mut input = input;
	let n = input.len();

	for _ in 0..num {
		let mut ans = vec![];

		for i in 1..=n {
			let mut accum = 0;
			for (a, b) in input.iter().zip(iterator_for(i, n)) {
				accum += (a*b);
			}
			ans.push(accum.abs()%10);
		}
		input = ans;
	}
	// println!("{:?}", input);
	input.iter().take(8).map(|x| x.to_string()).collect::<Vec<String>>().join("")

}

fn main() {
	let mut input = get_input();
	let ans = transform_phase(input.clone(), 100);
	println!("Part1: {}", ans);

}
