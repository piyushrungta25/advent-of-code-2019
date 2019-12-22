use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

use intcode::{get_computer, IntCodeComputer, Signal};
use std::error::Error;

struct Droid {
    comp: IntCodeComputer,
}

impl Droid {
    fn new(prog: Vec<i64>) -> Self {
        Droid {
            comp: get_computer(&prog, vec![]),
        }
    }

    fn move_dir(&mut self, dir: Direction) -> i64 {
        self.comp.feed_input(dir as i64);
        self.comp.run_till_signal(Signal::ProducedOutput);
        self.comp.get_output().unwrap()
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Oxygen(i64),
    Wall,
    Floor(i64),
}

impl Tile {
    fn from_output(out: i64, path_length: i64) -> Tile {
        match out {
            0 => Tile::Wall,
            1 => Tile::Floor(path_length),
            2 => Tile::Oxygen(path_length),
            _ => panic!("should not happen"),
        }
    }

    fn value(&self) -> Option<i64> {
        match self {
            Tile::Oxygen(v) => Some(*v),
            Tile::Floor(v) => Some(*v),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Direction {
    fn opposite(&self) -> Direction {
        use Direction::*;
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }

    fn get_delta(&self) -> (i64, i64) {
        use Direction::*;
        match self {
            North => (0, 1),
            South => (0, -1),
            East => (1, 0),
            West => (-1, 0),
        }
    }

    fn move_point(&self, (x, y): (i64, i64)) -> (i64, i64) {
        let delta = self.get_delta();
        (x + delta.0, y + delta.1)
    }
}

fn get_input() -> Result<Vec<i64>, Box<dyn Error>> {
    let s = fs::read_to_string("input")?;
    Ok(s.split(',').filter_map(|x| x.parse::<i64>().ok()).collect())
}

fn rec_helper(
    droid: &mut Droid,
    tile_info: &mut HashMap<(i64, i64), Tile>,
    cur_path: i64,
    smallest_oxy: &mut i64,
    oxy_location: &mut (i64, i64),
    cur_position: (i64, i64),
) {
    use Direction::*;
    use Tile::*;

    for dir in [North, South, East, West].iter() {
        let new_pos = dir.move_point(cur_position);
        let cur_path_len = cur_path + 1;

        let out = droid.move_dir(*dir);
        let tile = Tile::from_output(out, cur_path_len);

        match tile {
            Floor(dist) => {
                let mut should_recurse = true;
                tile_info
                    .entry(new_pos)
                    .and_modify(|e| {
                        if new_pos != (0, 0) && dist < e.value().unwrap() {
                            *e = tile;
                        } else {
                            should_recurse = false;
                        }
                    })
                    .or_insert(tile);

                if should_recurse {
                    rec_helper(droid, tile_info, dist, smallest_oxy, oxy_location, new_pos);
                }
                let _ = droid.move_dir(dir.opposite());
            }
            Oxygen(dist) => {
                if dist < *smallest_oxy {
                    *smallest_oxy = dist;
                    *oxy_location = new_pos;
                    tile_info.insert(new_pos, Tile::from_output(out, *smallest_oxy));
                }
                let _ = droid.move_dir(dir.opposite());
            }
            _ => {}
        }
    }
}

fn part1() -> (i64, HashMap<(i64, i64), Tile>, (i64, i64)) {
    let prog = get_input().unwrap();
    let mut droid = Droid::new(prog);
    let mut tile_info: HashMap<(i64, i64), Tile> = HashMap::new();
    let mut smallest_path = std::i64::MAX;
    let mut oxy_location = (-1, -1);

    tile_info.insert((0, 0), Tile::from_output(0, 0));

    rec_helper(
        &mut droid,
        &mut tile_info,
        0,
        &mut smallest_path,
        &mut oxy_location,
        (0, 0),
    );

    (smallest_path, tile_info, oxy_location)
}

fn part2(tile_info: HashMap<(i64, i64), Tile>, oxy_location: (i64, i64)) -> i64 {
    use Direction::*;
    use Tile::*;

    enum Item {
        Pos((i64, i64)),
        Sentinel,
    }

    let mut time = -1;
    let mut has_oxy = HashSet::new();
    has_oxy.insert(oxy_location);

    let mut queue = VecDeque::new();
    queue.push_back(Item::Pos(oxy_location));

    'outer: while queue.len() > 0 {
        queue.push_back(Item::Sentinel);
        time += 1;
        'inner: loop {
            match queue.pop_front().unwrap() {
                Item::Sentinel => {
                    break 'inner;
                }
                Item::Pos(pos) => {
                    for dir in [North, South, East, West].iter() {
                        let new_pos = dir.move_point(pos);
                        match (tile_info.get(&new_pos), has_oxy.contains(&new_pos)) {
                            (Some(Floor(_)), false) => {
                                has_oxy.insert(new_pos);
                                queue.push_back(Item::Pos(new_pos));
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    time
}

fn main() {
    let (part1, area_map, oxy_location) = part1();
    assert_eq!(part1, 234);

    let time = part2(area_map, oxy_location);
    assert_eq!(time, 292);

    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", time);
}
