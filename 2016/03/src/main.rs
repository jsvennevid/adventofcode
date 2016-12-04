extern crate regex;

use regex::Regex;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::io;

struct Shape {
    a: i32,
    b: i32,
    c: i32
}

impl Shape {
    fn new(a: i32, b: i32, c: i32) -> Shape {
        Shape {a: a, b: b, c: c}
    }
    fn is_triangle(&self) -> bool {
        (self.a + self.b) > self.c &&
        (self.a + self.c) > self.b &&
        (self.c + self.b) > self.a
    }
}

struct Parser {}

impl Parser {
    fn parse(line: &str, re: &Regex) -> Option<Shape> {
        match re.captures(line) {
            Some(c) => {
                Some(Shape {
                    a: c.at(1).unwrap_or("0").parse::<i32>().unwrap(),
                    b: c.at(2).unwrap_or("0").parse::<i32>().unwrap(),
                    c: c.at(3).unwrap_or("0").parse::<i32>().unwrap()
                })
            }
            None => None
        }
    }

    fn load(src: &str) -> Result<Vec<Shape>, io::Error> {
        let path = Path::new(src);
        let file = (File::open(path))?;
        let reader = BufReader::new(&file);
        let re = Regex::new(r" *([0-9]+) +([0-9]+) +([0-9]+)").unwrap();

        let n = reader
            .lines()
            .map(|x| x.unwrap())
            .map(|x| Parser::parse(&x, &re))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        Ok(n)
    }

    fn transform(inp: &Vec<Shape>) -> Vec<Shape> {
        assert!(inp.len() % 3 == 0);

        (0..inp.len()).map(|i| {
            let index = (i / 3) * 3;
            let offset = i - index;

            let s1 = &inp[(index + 0) as usize];
            let s2 = &inp[(index + 1) as usize];
            let s3 = &inp[(index + 2) as usize];

            match offset {
                0 => Shape::new(s1.a, s2.a, s3.a),
                1 => Shape::new(s1.b, s2.b, s3.b),
                2 => Shape::new(s1.c, s2.c, s3.c),
                _ => panic!("Index out of bounds")
            }
        }).collect()
    }
}



fn main() {
    let shapes1 = Parser::load("input.txt").unwrap_or(Vec::new());

    println!("possible 1: {}", shapes1.iter().filter(|x| x.is_triangle()).count());

    let shapes2 = Parser::transform(&shapes1);

    println!("possible 2: {}", shapes2.iter().filter(|x| x.is_triangle()).count());
}
