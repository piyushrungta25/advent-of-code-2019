use std::fs;

use std::error::Error;
use std::collections::{HashMap};
use intcode::{get_computer, Signal};
use std::{thread, time};

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};


fn get_input() -> Result<Vec<i64>, Box<dyn Error>> {
    let s = fs::read_to_string("input")?;
    Ok(s.split(',').filter_map(|x| x.parse::<i64>().ok()).collect())
}

struct Display {
	canvas: Canvas<Window>,
	scale: i64,
}

impl Display {
	fn new(width: u32, height: u32, scale: u32) -> Self {
		let sdl_context = sdl2::init().unwrap();
	    let video_subsystem = sdl_context.video().unwrap();

	    let window = video_subsystem.window("arcade", width*scale, height*scale)
	        .position_centered()
	        .build()
	        .unwrap();
	    let mut canvas = window.into_canvas().build().unwrap();

	    canvas.set_draw_color(Color::RGB(0,0,0));
	    canvas.clear();
	    canvas.present();

	    Display {canvas, scale: scale as i64}
	}

	fn draw_tile(&mut self, x:i64, y:i64, tile: Tile) {
		self.canvas.set_draw_color(tile.color());
        self.canvas.fill_rect(Rect::new((x*self.scale) as i32, (y*self.scale) as i32, self.scale as u32, self.scale as u32)).unwrap();
        self.canvas.present();
	}
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
	Empty, Wall, Block, Paddle, Ball
}

impl Tile {
	fn from_id(id: i64) -> Self {
		use Tile::*;
		match id {
			0 => Empty,
			1 => Wall,
			2 => Block,
			3 => Paddle,
			4 => Ball,
			_ => panic!("unexpected output")
		}
	}

	fn color(&self) -> Color {
		use Tile::*;
		match self {
			Empty => Color::RGB(85, 85, 85),
			Wall => Color::RGB(255, 255, 255),
			Block => Color::RGB(255, 61, 61),
			Paddle => Color::RGB(60, 60, 255),
			Ball => Color::RGB(135, 255, 255),
		}

	}
}


fn part1(input: &Vec<i64>) -> usize {
	let mut count = 0;
	let mut hs: HashMap<(i64, i64), i64> = HashMap::new();

	let mut comp = get_computer(&input, vec![]);
	let mut output = vec![];

	'main_loop: loop {
		match comp.run() {
			Signal::Halt => break 'main_loop,
			Signal::ProducedOutput => output.push(comp.get_output().unwrap()),
			_ => {}
		}

		if output.len() == 3 {
			let (x, y, z) = (output[0], output[1], output[2]);
			output.clear();

			if z==2 && (!hs.contains_key(&(x, y)) || *hs.get(&(x, y)).unwrap() != 2) {
				count+=1;
			}
			hs.entry((x, y)).and_modify(|e| {*e = z});
		}
	}
	count
}

fn part2(input: &Vec<i64>) -> i64 {
	use Tile::*;

	let mut score: i64 = 0;
	let mut display = Display::new(38, 21, 10);
	let mut comp = get_computer(&input, vec![]);
	let mut ball_x = -1;
	let mut paddle_x = -1;
	let mut output = vec![];

	loop {
		match comp.run() {
			Signal::Halt => break,
			Signal::ProducedOutput => output.push(comp.get_output().unwrap()),
			Signal::NeedsInput => {
				use std::cmp::Ordering::*;
				let inp = match ball_x.cmp(&paddle_x) {
					Less => -1,
					Equal => 0,
					Greater => 1
				};
				comp.feed_input(inp);
			}
			_ => {}
		}

		if output.len() != 3 {continue;}

		let (x, y, z) = (output[0], output[1], output[2]);
		output.clear();

		if x == -1 && y == 0 {
			score = z;
			continue;
		}

		let tile = Tile::from_id(z);
		match tile {
			Ball => ball_x = x,
			Paddle => paddle_x = x,
			_ => {}
		}

		display.draw_tile(x, y, tile);
		thread::sleep(time::Duration::from_millis(1));
	}

	score
}


fn main() {
	let mut input = get_input().unwrap();
    println!("Part1: {}", part1(&input));

    input[0] = 2;
    println!("Part1: {}", part2(&input));
}
