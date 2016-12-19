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
    raw_name: String,
    id: i32,
    checksum: String
}

impl Room {
    fn new(name: &str, raw_name: &str, id: i32, checksum: &str) -> Room {
        Room {name: name.to_string(), raw_name: raw_name.to_string(), id: id, checksum: checksum.to_string()}
    }

    fn valid(&self) -> bool {
        let c1 = self.name.chars()
            .sorted();
        let groups = c1
            .iter()
            .group_by(|&x| x);
        let chars = groups
            .into_iter()
            .map(|(c,n)| (-(n.count() as i32), c))
            .sorted();
        let c3 = chars.iter()
            .map(|&(_,c)|c)
            .cloned()
            .collect::<String>();
        let c4 = &c3[0..5];

        self.checksum == c4
    }

    fn decrypt(&self) -> String {
        let start = 'a' as i32;
        let stop = 'z' as i32;
        let c1 = self.raw_name.chars()
            .map(|c| {
                match c {
                    '-' => { ' ' }
                    c => ((start + (((c as i32)-start) + self.id) % (1 + (stop-start))) as u8) as char
                }
            })
            .collect::<String>();

        c1
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
                let raw_name = c
                    .at(1)
                    .unwrap()
                    .chars()
                    .collect::<String>();
                let id = c.at(2).unwrap().parse::<i32>().unwrap();
                let checksum = c.at(3).unwrap();

                Some(Room::new(&name, &raw_name, id, checksum))
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

    let sum = rooms.iter().filter(|&r| r.valid()).map(|ref r| r.id).sum::<i32>();

    println!("SUM {}", sum);

    match rooms.iter().find(|&r| r.valid() && r.decrypt() == "northpole object storage") {
        Some(room) => { println!("ROOM {}", room.id) }
        None => println!("NO ROOM")
    }
}
