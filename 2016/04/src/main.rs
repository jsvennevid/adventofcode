struct Parser {}

extern crate regex;
extern crate itertools;

use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::io;

struct Room {
    name: String,
    id: i32,
    checksum: String
}

impl Room {
    fn new(name: &str, id: i32, checksum: &str) -> Room {
        Room {name: name.to_string(), id: id, checksum: checksum.to_string()}
    }

    fn valid(&self) -> bool {
        let groups = self.name.chars().into_iter().group_by(|&x| x);
        for group in groups {
            println!("{}", group.sum::<i32>().abs());
        }

        self.name.len() > 0
    }
}

impl Parser {
    fn parse(str: &str, re: &Regex) -> Option<Room> {
        match re.captures(str) {
            Some(c) => {
                let name = c
                    .at(1)
                    .unwrap()
                    .chars()
                    .filter(|x| x != &'-')
                    .collect::<String>();
                let id = c.at(2).unwrap().parse::<i32>().unwrap();
                let checksum = c.at(3).unwrap();

                Some(Room::new(&name, id, checksum))
            }
            None => None
        }
    }

    fn load(src: &str) -> Result<Vec<Room>,io::Error> {
        let path = Path::new(src);
        let file = (File::open(path))?;
        let reader = BufReader::new(&file);
        let re = Regex::new(r"([a-z-]+)-([0-9]+)\[([a-z]+)\]").unwrap();

        let n = reader
            .lines()
            .map(|x| x.unwrap())
            .map(|x| Parser::parse(&x, &re))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();

        Ok(n)
    }
}

fn main() {
    let rooms = Parser::load("input.txt").unwrap_or(Vec::new());

    for i in rooms {
        println!("room: {} {} {} {}", i.name, i.id, i.checksum, i.valid());
    }
}
