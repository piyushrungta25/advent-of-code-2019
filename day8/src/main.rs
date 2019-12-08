use std::fmt::{self, Display};
use std::fs;

const LAYER_WIDTH: usize = 25;
const LAYER_HEIGHT: usize = 6;

#[derive(Clone)]
struct Layer {
    data: Vec<Vec<char>>,
}

impl Layer {
    fn new(layer: &[char], width: usize) -> Self {
        let mut data = vec![];
        for i in (0..layer.len()).step_by(width) {
            data.push(layer[i..i + width].to_owned());
        }

        Layer { data }
    }

    fn rows(&self) -> impl Iterator<Item = &Vec<char>> {
        self.data.iter()
    }

    fn rows_mut(&mut self) -> impl Iterator<Item = &mut Vec<char>> {
        self.data.iter_mut()
    }

    fn value_at(&self, row: usize, column: usize) -> char {
        self.data[row][column]
    }

    fn _set_value(&mut self, row: usize, column: usize, value: char) {
        self.data[row][column] = value;
    }

    fn count_pixels(&self) -> (u32, u32, u32) {
        let (mut zero, mut one, mut two) = (0, 0, 0);
        for row in self.rows() {
            for c in row {
                match c {
                    '0' => zero += 1,
                    '1' => one += 1,
                    '2' => two += 1,
                    _ => {}
                }
            }
        }
        (zero, one, two)
    }
}

impl Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows() {
            for c in row {
                match c {
                    '0' => write!(f, "{}", ' ')?,
                    '1' => write!(f, "{}", '*')?,
                    _ => {}
                }
            }
            write!(f, "{}", '\n')?;
        }
        Ok(())
    }
}

struct Image {
    data: Vec<Layer>,
}

impl Image {
    fn new(img: &Vec<char>, width: usize, height: usize) -> Self {
        let mut data = vec![];
        let step = height * width;

        for i in (0..img.len()).step_by(step) {
            let layer = &img[i..(i + step)];
            data.push(Layer::new(layer, width));
        }
        Image { data }
    }

    fn layers(&self) -> impl Iterator<Item = &Layer> {
        self.data.iter()
    }

    fn non_transparet_value_at(&self, i: usize, j: usize) -> Option<char> {
        for layer in self.layers() {
            let pixel = layer.value_at(i, j);
            if pixel != '2' {
                return Some(pixel);
            }
        }
        None
    }

    fn flatten(&self) -> Layer {
        let mut final_layer = self.layers().nth(0).unwrap().clone();

        for (i, row) in final_layer.rows_mut().enumerate() {
            for (j, c) in row.iter_mut().enumerate() {
                if *c == '2' {
                    *c = self.non_transparet_value_at(i, j).unwrap();
                }
            }
        }

        final_layer
    }
}

fn part1(image: &Image) -> u32 {
    let mut min_zeros = std::u32::MAX;
    let mut one_times_two = 0;
    for layer in image.layers() {
        let (zero, one, two) = layer.count_pixels();
        if zero < min_zeros {
            one_times_two = one * two;
            min_zeros = zero;
        }
    }
    one_times_two
}

fn part2(image: &Image) {
    println!("{}", image.flatten());
}

fn main() {
    let input: Vec<char> = fs::read_to_string("input").unwrap().chars().collect();
    let image = Image::new(&input, LAYER_WIDTH, LAYER_HEIGHT);
    println!("Part 1: {:?}", part1(&image));
    println!("Part 2:");
    part2(&image);
}
