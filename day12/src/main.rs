use std::cmp::{max, min};
use std::fs;

// https://rosettacode.org/wiki/Least_common_multiple
fn gcd(a: u64, b: u64) -> u64 {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

#[derive(Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0, z: 0 }
    }

    fn from_vec(vals: Vec<i32>) -> Self {
        Position {
            x: vals[0],
            y: vals[1],
            z: vals[2],
        }
    }
}

type Velocity = Position;

#[derive(Clone)]
struct Body {
    position: Position,
    velocity: Velocity,
}

impl Body {
    fn with_position(pos: Vec<i32>) -> Self {
        Body {
            velocity: Velocity::new(),
            position: Position::from_vec(pos),
        }
    }

    fn potential_energy(&self) -> i32 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    fn kinetic_energy(&self) -> i32 {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }

    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn update_axis_position(&mut self, axis: Axis) {
        match axis {
            Axis::X => self.position.x += self.velocity.x,
            Axis::Y => self.position.y += self.velocity.y,
            Axis::Z => self.position.z += self.velocity.z,
        }
    }
}

struct System {
    bodies: Vec<Body>,
    _initial_state: Vec<Body>,
}

impl System {
    fn new(bodies: Vec<Body>) -> Self {
        System {
            _initial_state: bodies.clone(),
            bodies,
        }
    }

    fn reset_state(&mut self) {
        self.bodies = self._initial_state.clone();
    }

    fn update_axis_velocities(&mut self, i: usize, j: usize, axis: Axis) {
        match axis {
            Axis::X => {
                if self.bodies[i].position.x < self.bodies[j].position.x {
                    self.bodies[i].velocity.x += 1;
                    self.bodies[j].velocity.x -= 1;
                } else if self.bodies[i].position.x > self.bodies[j].position.x {
                    self.bodies[i].velocity.x -= 1;
                    self.bodies[j].velocity.x += 1;
                }
            }
            Axis::Y => {
                if self.bodies[i].position.y < self.bodies[j].position.y {
                    self.bodies[i].velocity.y += 1;
                    self.bodies[j].velocity.y -= 1;
                } else if self.bodies[i].position.y > self.bodies[j].position.y {
                    self.bodies[i].velocity.y -= 1;
                    self.bodies[j].velocity.y += 1;
                }
            }
            Axis::Z => {
                if self.bodies[i].position.z < self.bodies[j].position.z {
                    self.bodies[i].velocity.z += 1;
                    self.bodies[j].velocity.z -= 1;
                } else if self.bodies[i].position.z > self.bodies[j].position.z {
                    self.bodies[i].velocity.z -= 1;
                    self.bodies[j].velocity.z += 1;
                }
            }
        }
    }

    fn update_axis_positions(&mut self, axis: Axis) {
        self.bodies
            .iter_mut()
            .for_each(|body| body.update_axis_position(axis));
    }

    fn total_energy(&self) -> i32 {
        self.bodies.iter().map(|b| b.total_energy()).sum()
    }

    fn step_along_axis(&mut self, axis: Axis) {
        for i in 0..(self.bodies.len() - 1) {
            for j in (i + 1)..self.bodies.len() {
                self.update_axis_velocities(i, j, axis);
            }
        }
        self.update_axis_positions(axis);
    }

    fn step(&mut self, num: usize) {
        for _ in 0..num {
            self.step_along_axis(Axis::X);
            self.step_along_axis(Axis::Y);
            self.step_along_axis(Axis::Z);
        }
    }

    fn bodies_in_initial_position(&self, axis: Axis) -> bool {
        self.bodies
            .iter()
            .zip(&self._initial_state)
            .all(|(body, og)| match axis {
                Axis::X => body.position.x == og.position.x && body.velocity.x == 0,
                Axis::Y => body.position.y == og.position.y && body.velocity.y == 0,
                Axis::Z => body.position.z == og.position.z && body.velocity.z == 0,
            })
    }

    fn get_cycle_length(&mut self, axis: Axis) -> u64 {
        self.step_along_axis(axis);
        let mut counter: u64 = 1;
        while !self.bodies_in_initial_position(axis) {
            self.step_along_axis(axis);
            counter += 1;
        }

        counter
    }

    fn get_first_repeating_state(&mut self) -> u64 {
        let x_cycle = self.get_cycle_length(Axis::X);
        let y_cycle = self.get_cycle_length(Axis::Y);
        let z_cycle = self.get_cycle_length(Axis::Z);
        lcm(lcm(x_cycle, y_cycle), z_cycle)
    }
}

fn get_input() -> System {
    fn get_body_from_line(line: &str) -> Body {
        Body::with_position(
            line.split(", ")
                .map(|x| x.split("=").skip(1).next().unwrap().parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        )
    }

    let input = fs::read_to_string("input").unwrap();
    let bodies = input
        .trim()
        .split('\n')
        .map(|line| get_body_from_line(&line[1..(line.len() - 1)]))
        .collect::<Vec<Body>>();
    System::new(bodies)
}

fn main() {
    let mut sys = get_input();

    sys.step(1000);
    println!("Part1: {:?}", sys.total_energy());

    sys.reset_state();
    println!("Part2: {:?}", sys.get_first_repeating_state());
}
