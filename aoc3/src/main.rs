use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let mut b = BufReader::new(File::open("input.txt")?);

    let seg1 = read_segments(&mut b)?;
    let seg2 = read_segments(&mut b)?;

    println!("smallest {}", intersect_steps(&seg1, &seg2));
    Ok(())
}

fn intersect(ls: &Vec<Segment>, rs: &Vec<Segment>) -> i32 {
    let mut first_intersect = true;
    let mut min: i32 = 0;
    let mut first_compare = true;

    for a in ls {
        for b in rs {
            if first_compare {
                first_compare = false;
                continue;
            }
            if let Some(d) = a.intersect(b) {
                if first_intersect || d < min {
                    min = d;
                    first_intersect = false
                }
            }
        }
    }
    min
}

fn intersect_steps(ls: &Vec<Segment>, rs: &Vec<Segment>) -> i32 {
    let mut first_intersect = true;
    let mut min: i32 = 0;
    let mut first_compare = true;

    for a in ls {
        for b in rs {
            if first_compare {
                first_compare = false;
                continue;
            }
            if let Some(d) = a.intersect_steps(b) {
                if first_intersect || d < min {
                    min = d;
                    first_intersect = false
                }
            }
        }
    }
    min
}

impl Segment {
    fn intersect(&self, other: &Segment) -> Option<i32> {
        if self.d.horizontal() == other.d.horizontal() {
            return None;
        }
        let (h, v) = if self.d.horizontal() {
            (self, other)
        } else {
            (other, self)
        };
        if h.xintercept(v.origin.x) && v.yintercept(h.origin.y) {
            return Some(v.origin.x.abs() + h.origin.y.abs());
        }
        None
    }

    fn intersect_steps(&self, other: &Segment) -> Option<i32> {
        if self.d.horizontal() == other.d.horizontal() {
            return None;
        }
        let (h, v) = if self.d.horizontal() {
            (self, other)
        } else {
            (other, self)
        };
        if h.xintercept(v.origin.x) && v.yintercept(h.origin.y) {
            return Some(v.disty(h.origin.y) + h.distx(v.origin.x));
        }
        None
    }

    fn disty(&self, y: i32) -> i32 {
        self.distance
            + match self.d {
                Dir::D => self.origin.y - y,
                Dir::U => y - self.origin.y,
                _ => panic!(),
            }
    }
    fn distx(&self, x: i32) -> i32 {
        self.distance
            + match self.d {
                Dir::L => self.origin.x - x,
                Dir::R => x - self.origin.x,
                _ => panic!(),
            }
    }

    fn xintercept(&self, x: i32) -> bool {
        match self.d {
            Dir::L => x <= self.origin.x && x >= self.origin.x - self.l,
            Dir::R => x >= self.origin.x && x <= self.origin.x + self.l,
            _ => panic!(),
        }
    }

    fn yintercept(&self, y: i32) -> bool {
        match self.d {
            Dir::D => y <= self.origin.y && y >= self.origin.y - self.l,
            Dir::U => y >= self.origin.y && y <= self.origin.y + self.l,
            _ => panic!(),
        }
    }
}

fn read_segments(b: &mut BufReader<File>) -> std::io::Result<Vec<Segment>> {
    let mut line = String::new();
    b.read_line(&mut line)?;
    Ok(parse_segments(&line))
}

fn parse_segments(line: &str) -> Vec<Segment> {
    let mut p = Point { x: 0, y: 0 };
    let mut dist: i32 = 0;
    let seg: Vec<Segment> = line
        .split(',')
        .map(|s| add_move(&mut p, &mut dist, s))
        .collect();
    seg
}

enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    fn horizontal(&self) -> bool {
        match self {
            Dir::L => true,
            Dir::R => true,
            _ => false,
        }
    }
}

struct Segment {
    d: Dir,
    l: i32,
    origin: Point,
    distance: i32,
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d = match &self.d {
            Dir::U => 'U',
            Dir::D => 'D',
            Dir::L => 'L',
            Dir::R => 'R',
        };
        write!(f, "{},{}:{}{}", self.origin.x, self.origin.y, d, self.l)
    }
}

fn add_move(p: &mut Point, distance: &mut i32, s: &str) -> Segment {
    let orig_p = *p;
    let orig_distance = *distance;

    if let Some(c) = s.chars().nth(0) {
        let l = s[1..].trim().parse::<i32>().unwrap();
        *distance += l;
        let d = match c {
            'U' => {
                p.y += l;
                Dir::U
            }
            'D' => {
                p.y -= l;
                Dir::D
            }
            'L' => {
                p.x -= l;
                Dir::L
            }
            'R' => {
                p.x += l;
                Dir::R
            }
            _ => panic!("bad direction"),
        };
        Segment {
            d: d,
            l: l,
            origin: orig_p,
            distance: orig_distance,
        }
    } else {
        panic!("bad str")
    }
}

#[test]
fn test_intersect() {
    let test_data = [
        ("R8,U5,L5,D3", "U7,R6,D4,L4", 6),
        (
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
            159,
        ),
        (
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            135,
        ),
    ];
    for (l1, l2, d) in &test_data {
        assert_eq!(
            intersect(&parse_segments(l1), &parse_segments(l2)),
            *d as i32
        );
    }
}

#[test]
fn test_intersect_steps() {
    let test_data = [
        ("R8,U5,L5,D3", "U7,R6,D4,L4", 30),
        (
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
            610,
        ),
        (
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            410,
        ),
    ];
    for (l1, l2, d) in &test_data {
        assert_eq!(
            intersect_steps(&parse_segments(l1), &parse_segments(l2)),
            *d as i32
        );
    }
}

