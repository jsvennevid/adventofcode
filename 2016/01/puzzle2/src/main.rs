extern crate regex;

use regex::Regex;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::BTreeSet;
use std::cmp::Ordering;
use std::io;

static LEFT : i32 = -1;
static RIGHT : i32 = 1;

struct Direction {
    dx: i32,
    dz: i32
}

static DIRECTIONS : &'static [Direction] = &[
    Direction{dx: 0, dz: -1},
    Direction{dx: -1, dz: 0},
    Direction{dx: 0, dz: 1},
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

struct Pos {
    x: i32,
    z: i32
}

impl Ord for Pos {
    fn cmp(&self, other: &Pos) -> Ordering {
        let cx = self.x.cmp(&other.x);
        let cz = self.z.cmp(&other.z);

        match cx {
            Ordering::Equal => cz,
            _ => cx
        }
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Pos) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Pos) -> bool {
        return self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Pos {}

struct History {
    data: BTreeSet<Pos>
}

impl History {
    fn new() -> History {
        History {data: BTreeSet::new()}
    }

    fn track(&mut self, pos: Pos) -> Option<Pos> {
        if self.data.contains(&pos) {
            Some(pos)
        } else {
            self.data.insert(pos);
            None
        }
    }
}

struct State {
    pos: Pos,
    dir: i32
}

impl State {
    fn new() -> State {
        State {pos: Pos {x: 0, z: 0}, dir: 0}
    }

    fn turn(&mut self, turn: i32) {
        self.dir = (self.dir + turn) & ((DIRECTIONS.len()-1) as i32);
    }

    fn walk(&mut self, dist: i32, history: &mut History) -> Option<Pos> {
        let ref dir = &DIRECTIONS[self.dir as usize];
        for i in 1..(dist+1) {
            match history.track(Pos {x: self.pos.x + i * dir.dx, z: self.pos.z + i * dir.dz}) {
                Some(pos) => return Some(pos),
                None => ()
            }
        }

        self.pos.x = self.pos.x + dist * dir.dx;
        self.pos.z = self.pos.z + dist * dir.dz;
        None
    }
}

fn main() {
    let ops = load("../input.txt").unwrap_or(Vec::new());

    let mut state = State::new();
    let mut history = History::new();

    for op in ops {
        state.turn(op.turn);
        match state.walk(op.dist, &mut history) {
            Some(pos) => {
                println!("found pos: {} {} ({})", pos.x, pos.z, pos.x.abs() + pos.z.abs());
                return
            },
            None => ()
        }
    }

    println!("{}", state.pos.x.abs() + state.pos.z.abs());
}
