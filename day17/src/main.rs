use intcode::{get_computer, Signal};
use std::error::Error;
use std::fmt;
use std::fs;

fn get_input() -> Result<Vec<i64>, Box<dyn Error>> {
    let s = fs::read_to_string("input")?;
    Ok(s.split(',').filter_map(|x| x.parse::<i64>().ok()).collect())
}

struct Maze {
    matrix: Vec<Vec<Item>>,
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in self.matrix.iter() {
            for j in i {
                write!(f, "{}", j.to_ascii())?
            }
            write!(f, "\n",)?
        }
        Ok(())
    }
}

impl Maze {
    fn point_in_bounds(&self, (x, y): (i64, i64)) -> bool {
        x >= 0 && y >= 0 && x < self.matrix.len() as i64 && y < self.matrix[0].len() as i64
    }

    fn is_scaffold(&self, (x, y): (i64, i64)) -> bool {
        self.matrix[x as usize][y as usize] == Item::Scaffold
    }

    fn is_intersection(&self, (x, y): (i64, i64)) -> bool {
        use Item::*;

        let (x, y) = (x as usize, y as usize);

        self.matrix[x - 1][y] == Scaffold
            && self.matrix[x + 1][y] == Scaffold
            && self.matrix[x][y + 1] == Scaffold
            && self.matrix[x][y - 1] == Scaffold
    }

    fn robot_position(&self) -> (i64, i64) {
        for i in 1..(self.matrix.len() - 1) {
            for j in 1..(self.matrix[0].len() - 1) {
                match self.matrix[i][j] {
                    Item::Robot(_) => return (i as i64, j as i64),
                    _ => {}
                }
            }
        }

        panic!("this shouldn't happen");
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_int(i: i64) -> Direction {
        use Direction::*;

        match i as u8 as char {
            '^' => Up,
            '>' => Right,
            '<' => Left,
            'v' => Down,
            _ => panic!("shouldn't happen"),
        }
    }

    fn to_char(&self) -> char {
        use Direction::*;
        match self {
            Up => '^',
            Right => '>',
            Left => '<',
            Down => 'v',
        }
    }
    fn get_delta(&self) -> (i64, i64) {
        use Direction::*;
        match self {
            Down => (1, 0),
            Up => (-1, 0),
            Right => (0, 1),
            Left => (0, -1),
        }
    }

    fn move_point(&self, (x, y): (i64, i64)) -> (i64, i64) {
        let delta = self.get_delta();
        (x + delta.0, y + delta.1)
    }

    fn other_directions(&self) -> Vec<Direction> {
        use Direction::*;

        match self {
            Up => vec![Left, Right],
            Down => vec![Left, Right],
            Right => vec![Up, Down],
            Left => vec![Up, Down],
        }
    }

    fn get_turn_instructions(&self, to_dir: Direction) -> Vec<String> {
        use Direction::*;
        let insts = match self {
            Up => match to_dir {
                Up => vec![],
                Left => vec!["L"],
                Right => vec!["R"],
                Down => vec!["R", "R"],
            },
            Down => match to_dir {
                Up => vec!["R", "R"],
                Left => vec!["R"],
                Right => vec!["L"],
                Down => vec![],
            },
            Left => match to_dir {
                Up => vec!["R"],
                Left => vec![],
                Right => vec!["R", "R"],
                Down => vec!["L"],
            },
            Right => match to_dir {
                Up => vec!["L"],
                Left => vec!["R", "R"],
                Right => vec![],
                Down => vec!["R"],
            },
        };

        insts.iter().map(|x| x.to_string()).collect()
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Item {
    Scaffold,
    Empty,
    Robot(Direction),
}

impl Item {
    fn from_int(i: i64) -> Item {
        match i {
            35 => Item::Scaffold,
            46 => Item::Empty,
            _ => Item::Robot(Direction::from_int(i)),
        }
    }

    fn to_ascii(&self) -> char {
        match self {
            Item::Scaffold => '#',
            Item::Empty => '.',
            Item::Robot(dir) => dir.to_char(),
        }
    }

    fn get_dir(&self) -> Option<Direction> {
        match self {
            Item::Robot(d) => Some(*d),
            _ => None,
        }
    }
}

fn get_maze() -> Maze {
    let input = get_input().unwrap();

    let mut comp = get_computer(&input, vec![]);

    let mut maze: Vec<Vec<Item>> = vec![];
    let mut temp: Vec<Item> = vec![];
    loop {
        match comp.run() {
            Signal::ProducedOutput => {
                let out = comp.get_output().unwrap();
                match out {
                    10 => {
                        maze.push(temp.clone());
                        temp.clear();
                    }
                    _ => temp.push(Item::from_int(out)),
                }
            }
            Signal::Halt => break,
            _ => {}
        }
    }
    maze.pop(); // there is an extra new line at the end
    Maze { matrix: maze }
}

fn sum_of_alignment(maze: &Maze) -> i64 {
    let mut sm = 0;

    for i in 1..(maze.matrix.len() - 1) {
        for j in 1..(maze.matrix[0].len() - 1) {
            let (i, j) = (i as i64, j as i64);
            if maze.is_scaffold((i, j)) && maze.is_intersection((i, j)) {
                sm += i * j;
            }
        }
    }

    sm
}

fn get_uncompressed_path(maze: &Maze) -> Vec<String> {
    // get the position of the robot
    let mut pos = maze.robot_position();

    let mut commands = vec![];
    let mut dir = maze.matrix[pos.0 as usize][pos.1 as usize]
        .get_dir()
        .unwrap();

    'outer: loop {
        let mut count = 0;
        'inner: loop {
            let (x1, y1) = dir.move_point(pos);
            if maze.point_in_bounds((x1, y1)) && maze.is_scaffold((x1, y1)) {
                count += 1;
                pos = (x1, y1);
            } else {
                if count > 0 {
                    commands.push(count.to_string());
                }
                for d in dir.other_directions() {
                    let (x1, y1) = d.move_point(pos);
                    if maze.point_in_bounds((x1, y1)) && maze.is_scaffold((x1, y1)) {
                        let move_inst = dir.get_turn_instructions(d);
                        commands.extend_from_slice(&move_inst);
                        dir = d;
                        break 'inner;
                    }
                }

                break 'outer;
            }
        }
    }

    commands
}

fn part2(maze: &Maze) -> i64 {
    let path = get_uncompressed_path(&maze);

    // solve this by hand using the path above, it pretty easy

    // A,B,B,A,B,C,A,C,B,C
    // A = L,4,L,6,L,8,L,12
    // B = L,8,R,12,L,12
    // C = R,12,L,6,L,6,L,8

    let mut input = get_input().unwrap();
    input[0] = 2;

    let instructions =
        "A,B,B,A,B,C,A,C,B,C\nL,4,L,6,L,8,L,12\nL,8,R,12,L,12\nR,12,L,6,L,6,L,8\nn\n";
    let instructions = instructions
        .chars()
        .map(|x| x as u8 as i64)
        .collect::<Vec<i64>>();

    let mut comp = get_computer(&input, instructions);

    comp.run_till_signal(Signal::Halt);
    comp.get_output().unwrap()
}

fn main() {
    let maze = get_maze();
    println!("{}", maze);

    let part1 = sum_of_alignment(&maze);
    assert_eq!(part1, 6448);
    println!("Part1: {:?}", part1);

    let part2 = part2(&maze);
    assert_eq!(part2, 914900);
    println!("Part2: {:?}", part2);
}
