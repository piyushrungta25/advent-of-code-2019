use std::fs;
use std::iter::repeat;

fn get_input(repeat: usize) -> Vec<i64> {
    let input = fs::read_to_string("input").unwrap();
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .cycle()
        .take(input.len() * repeat)
        .collect::<Vec<i64>>()
}

fn iterator_for(times: usize, len: usize) -> impl Iterator<Item = i64> {
    repeat(0)
        .take(times - 1)
        .chain(repeat(1).take(times))
        .chain(repeat(0).take(times))
        .chain(repeat(-1).take(times))
        .cycle()
        .take(len)
}

fn part1() -> String {
    let mut input = get_input(1);
    let n = input.len();

    for _ in 0..100 {
        let mut ans = vec![];

        for i in 1..=n {
            let mut accum = 0;
            for (a, b) in input.iter().zip(iterator_for(i, n)) {
                accum += a * b;
            }
            ans.push(accum.abs() % 10);
        }
        input = ans;
    }

    input
        .iter()
        .take(8)
        .map(|x| x.to_string())
        .collect::<String>()
}

fn part2() -> String {
    let input = get_input(10000);
    let index = input[0..7].iter().fold(0, |acc, x| acc * 10 + x);
    let mut input = input[(index as usize)..(input.len())].to_owned();

    for _ in 0..100 {
        let mut running_sum = 0;
        for i in (0..(input.len())).rev() {
            running_sum += input[i];
            input[i] = running_sum.abs() % 10;
        }
    }

    input
        .iter()
        .take(8)
        .map(|x| x.to_string())
        .collect::<String>()
}

fn main() {
    let part1 = part1();
    let part2 = part2();
    assert_eq!("85726502", part1);
    assert_eq!("92768399", part2);

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
