#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::cmp;

struct Address {}

impl Address {

    fn supports_tls(address: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\[)?([a-z]+)(\])?").unwrap();
        }

        let mut count = 0;
        for cap in RE.captures_iter(address) {
            let is_hypernet = cap.at(1).is_some();
            let address = cap.at(2).unwrap();
            let is_abba = Address::has_abba(address);

            if is_abba && !is_hypernet {
                count += 1;
            } else if is_abba && is_hypernet {
                return false;
            }
        }

        count > 0
    }

    fn supports_ssl(address: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\[)?([a-z]+)(\])?").unwrap();
        }

        let mut normal = Vec::new();
        let mut hypernet = Vec::new();

        for cap in RE.captures_iter(address) {
            let is_hypernet = cap.at(1).is_some();
            let address = cap.at(2).unwrap();
            if is_hypernet {
                hypernet.push(address);
            } else {
                normal.push(address);
            }
        }

        let mut count = 0;
        for slice in normal.iter() {
            let aba = Address::get_aba(slice);
            if aba.is_empty() {
                continue;
            }

            for i in aba.iter() {
                let mut found = false;
                let bytes = i.as_bytes();
                let test = String::from_utf8(vec![bytes[1], bytes[0], bytes[1]]).unwrap();

                for j in hypernet.iter() {
                    match j.find(&test) {
                        Some(_) => {
                            found = true;
                            break;
                        }
                        None => {}
                    }
                }

                if found {
                    count += 1;
                }
            }
        }

        count > 0
    }

    fn get_aba(inp: &str) -> Vec<&str> {
        let len = cmp::max(0, inp.len()-2);
        let n = (0..len)
            .map(|i|{
                let s = &inp[(i)..(i+3)];
                let bytes = s.as_bytes();
                if (bytes[0] == bytes[2]) && (bytes[0] != bytes[1]) {
                    Some(s)
                } else {
                    None
                }
            })
            .filter(|n| n.is_some())
            .map(|n| n.unwrap())
            .collect();
        n
    }

    fn has_abba(inp: &str) -> bool {
        let len = cmp::max(0, inp.len()-3);
        for i in 0..len {
            let bytes = inp[(i)..(i+4)].as_bytes();
            if (bytes[0] == bytes[3]) && (bytes[1] == bytes[2]) && (bytes[0] != bytes[1]) {
                return true;
            }
        }

        false
    }
}

struct Parser {}

impl Parser {
    fn load(src: &str) -> Result<Vec<String>, io::Error> {
        let path = Path::new(src);
        let file = (File::open(path))?;
        let reader = BufReader::new(&file);

        let n = reader
            .lines()
            .map(|x| x.unwrap())
            .collect();

        Ok(n)
    }
}

fn main() {
    let addresses = Parser::load("input.txt").unwrap_or(Vec::new());

    let count = addresses.iter().filter(|a| Address::supports_tls(a)).count();
    println!("1 {}", count);

    let count2 = addresses.iter().filter(|a| Address::supports_ssl(a)).count();
    println!("2 {}", count2);
}
