extern crate itertools;

use itertools::Itertools;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::io;

struct Parser {}

impl Parser {
    fn load(src: &str) -> Result<Vec<String>, io::Error> {
        let path = Path::new(src);
        let file = (File::open(path))?;
        let reader = BufReader::new(&file);

        let n = reader.lines()
            .map(|x| x.unwrap())
            .collect();

        Ok(n)
    }
}

fn main() {
    let text = Parser::load("input.txt").unwrap_or(Vec::new());
    let empty = "".to_string();
    let length = text.get(0).unwrap_or(&empty).len();

    let message = (0..length).map(|i| {
        let n = text
            .iter()
            .map(|x| x.as_bytes()[i] as char)
            .sorted()
            .into_iter()
            .collect::<String>();

        let g = n.chars()
            .group_by(|&x| x)
            .into_iter()
            .map(|(x, n)| (-(n.count() as i32), x))
            .sorted()
            .into_iter()
            .take(1)
            .map(|(_, x)|x)
            .collect::<String>();
        g
    }).collect::<String>();

    let message2 = (0..length).map(|i| {
        let n = text
            .iter()
            .map(|x| x.as_bytes()[i] as char)
            .sorted()
            .into_iter()
            .collect::<String>();

        let g = n.chars()
            .group_by(|&x| x)
            .into_iter()
            .map(|(x, n)| ((n.count() as i32), x))
            .sorted()
            .into_iter()
            .take(1)
            .map(|(_, x)|x)
            .collect::<String>();
        g
    }).collect::<String>();

    println!("{}", message);
    println!("{}", message2);
}
