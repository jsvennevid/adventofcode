#[macro_use] extern crate lazy_static;
extern crate regex;

struct Parser {}

use regex::Regex;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::str;

impl Parser {
    fn load(src: &str) -> Result<String, io::Error> {
        let path = Path::new(src);
        let file = (File::open(path))?;
        let reader = BufReader::new(&file);

        let n = reader.lines()
            .map(|x| x.unwrap())
            .collect();

        Ok(n)
    }
}

struct Formula {
    len: usize,
    rep: usize
}

impl Formula {
    fn parse(inp: &str) -> Option<Formula> {
        lazy_static! {
            static ref RE: Regex = Regex::new("([0-9]+)x([0-9]+)").unwrap();
        }

        match RE.captures(inp) {
            Some(c) => {
                let len = c.at(1).unwrap().parse::<usize>().unwrap();
                let rep = c.at(2).unwrap().parse::<usize>().unwrap();

                Some(Formula { len: len, rep: rep })
            }
            None => None
        }
    }
}

fn decompress(bytes: &[u8], recurse: bool) -> usize {
    let mut size : usize = 0;

    match bytes.iter().position(|b| *b == ('(' as u8)) {
        Some(startofs) => {
            size += startofs;

            let start = startofs + 1;
            let search = &bytes[start..];
            match search.iter().position(|b| *b == (')' as u8)) {
                Some(endofs) => {
                    let formula = str::from_utf8(&bytes[start..(start+endofs)]).unwrap();
                    match Formula::parse(formula) {
                        Some(formula) => {
                            let next = (start + endofs) + 1 + formula.len;

                            if recurse {
                                let deep = &bytes[(start+endofs+1)..next];
                                size += decompress(deep, recurse) * formula.rep;
                            } else {
                                let s = str::from_utf8(&bytes[(start+endofs+1)..next]).unwrap();
                                size += s.len() * formula.rep;
                            }

                            let post = &bytes[next..];
                            size += decompress(post, recurse);
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }
        None => { size += bytes.len(); }
    }

    size
}

fn main() {
    let input = Parser::load("input.txt").unwrap_or("".to_string());
    let bytes = input.as_bytes();

    let size1 = decompress(bytes, false);
    let size2 = decompress(bytes, true);

    println!("u: {}, d1: {}, d2: {}", bytes.len(), size1, size2);
}
