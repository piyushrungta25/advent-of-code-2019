use std::cmp::min;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

fn sorted_line(p1: Point, p2: Point) -> Line {
    if p1.x == p2.x {
        let (m1, m2) = if p1.y < p2.y { (p1, p2) } else { (p2, p1) };
        return Line { start: m1, end: m2 };
    } else {
        let (m1, m2) = if p1.x < p2.x { (p1, p2) } else { (p2, p1) };
        return Line { start: m1, end: m2 };
    }
}

fn get_lines(w: String) -> Vec<Line> {
    let mut last_pos = Point { x: 0, y: 0 };
    w.split(",")
        .map(|x| {
            let mut chars = x.chars();
            let direction = chars.next().unwrap();
            let length = chars.collect::<String>().parse::<i64>().unwrap();

            let new_point = match direction {
                'R' => Point {
                    x: last_pos.x + length,
                    y: last_pos.y,
                },
                'L' => Point {
                    x: last_pos.x - length,
                    y: last_pos.y,
                },
                'U' => Point {
                    x: last_pos.x,
                    y: last_pos.y + length,
                },
                'D' => Point {
                    x: last_pos.x,
                    y: last_pos.y - length,
                },
                _ => panic!("wup"),
            };
            let ret = sorted_line(last_pos, new_point);
            last_pos = new_point;
            ret
        })
        .collect()
}

fn get_input() -> Result<(Vec<Line>, Vec<Line>), Box<dyn Error>> {
    let mut f = File::open("input")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    let mut lines: Vec<Vec<Line>> = s.split("\n").map(str::to_owned).map(get_lines).collect();

    let l2 = lines.remove(1);
    let l1 = lines.remove(0);
    Ok((l1, l2))
}

fn lines_intersect(l1: Line, l2: Line) -> Option<Point> {
    let (v, h) = if l1.start.x == l1.end.x {
        (l1, l2)
    } else {
        (l2, l1)
    };
    if h.start.x <= v.start.x
        && h.end.x >= v.start.x
        && h.start.y <= v.end.y
        && h.start.y >= v.start.y
    {
        return Some(Point {
            x: v.start.x,
            y: h.start.y,
        });
    }
    None
}

fn main() {
    let (wire1, wire2) = get_input().unwrap();
    let mut min_distance = ::std::i64::MAX;

    // the input is small enough, lets brute force
    for line1 in wire1 {
        for line2 in &wire2 {
            match lines_intersect(line1, *line2) {
                Some(pt) => min_distance = min(min_distance, pt.x.abs() + pt.y.abs()),
                _ => {}
            }
        }
    }

    println!("{:?}", min_distance);
}
