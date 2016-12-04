extern crate regex;

use regex::Regex;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::io::BufReader;
use std::io;

static LEFT : i32 = -1;
static RIGHT : i32 = 1;

struct Direction {
    dx: i32,
    dz: i32
}

static DIRECTIONS : &'static [Direction] = &[
    Direction{dx: 0, dz: -1},
    Direction{dx: 0, dz: 1},
    Direction{dx: -1, dz: 0},
    Direction{dx: 1, dz: 0}
];

struct Op {
    turn: i32,
    dist: i32
}

fn parse(src: &str) -> Vec<Op> {
    let re = Regex::new(r"(L|R)([0-9]+)").unwrap();

    src
        .split(",")
        .map(|x| x.trim().to_string())
        .map(|x| {
            let c = re.captures(&x).unwrap();
            let turn = match c.at(1) {
                Some("L") => LEFT,
                Some("R") => RIGHT,
                _ => panic!("Invalid direction")
            };
            let dist = match c.at(2) {
                Some(n) => n.parse::<i32>().unwrap(),
                _ => panic!("No distance specified")
            };

            Op {turn: turn, dist: dist}
        })

        .collect()
}

fn load(src : &str) -> Result<Vec<Op>, io::Error> {
    let path = Path::new(src);
    let file = (File::open(path))?;
    let reader = BufReader::new(&file);

    let n : Vec<Op> = reader
        .lines()
        .map(|x| x.unwrap())
        .flat_map(|x| parse(&x))
        .collect();

    Ok(n)
}

struct State {
    x: i32,
    z: i32,
    dir: i32
}

impl State {
    fn turn(&self, turn: i32) -> State {
        let m = (self.dir + turn) & ((DIRECTIONS.len()-1) as i32);
        State {x: self.x, z: self.z, dir: m}
    }

    fn walk(&self, dist: i32) -> State {
        let ref dir = &DIRECTIONS[self.dir as usize];
        State {x: self.x + dist * dir.dx, z: self.z + dist * dir.dz, dir: self.dir}

    }
}

fn main() {
	let ops = load("input.txt").unwrap_or(Vec::new());

    let mut state = State{x: 0, z: 0,dir: 0};

    for op in ops {
        state = state.turn(op.turn).walk(op.dist);
    }

    println!("{}", state.x.abs() + state.z.abs());
}
