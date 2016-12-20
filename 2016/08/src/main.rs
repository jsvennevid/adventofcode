#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::io;

struct Screen {
    buffer: Vec<char>
}

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

impl Screen {
    fn new() -> Screen {
        let mut buffer = Vec::new();
        buffer.resize((WIDTH * HEIGHT) as usize, '.');

        Screen { buffer: buffer }
    }

    fn execute(&mut self, command: &str) {
        lazy_static! {
            static ref RECT: Regex = Regex::new("rect ([0-9]+)x([0-9]+)").unwrap();
            static ref ROTROW: Regex = Regex::new("rotate row y=([0-9]+) by ([0-9]+)").unwrap();
            static ref ROTCOL: Regex = Regex::new("rotate column x=([0-9]+) by ([0-9]+)").unwrap();
        }

        match RECT.captures(command) {
            Some(c) => {
                let x = c.at(1).unwrap().parse::<usize>().unwrap();
                let y = c.at(2).unwrap().parse::<usize>().unwrap();
                self.rect(x, y);
            }
            None => {}
        }

        match ROTROW.captures(command) {
            Some(c) => {
                let row = c.at(1).unwrap().parse::<usize>().unwrap();
                let pixels = c.at(2).unwrap().parse::<usize>().unwrap();
                self.rotate_row(row, pixels);
            }
            None => {}
        }

        match ROTCOL.captures(command) {
            Some(c) => {
                let col = c.at(1).unwrap().parse::<usize>().unwrap();
                let pixels = c.at(2).unwrap().parse::<usize>().unwrap();
                self.rotate_col(col, pixels);
            }
            None => {}
        }
    }

    fn rect(&mut self, x: usize, y: usize) {
        println!("rect {}x{}", x, y);
        for j in 0..y {
            for i in 0..x {
                let offset = (i + j * WIDTH) as usize;
                self.buffer[offset] = '#';
            }
        }
    }

    fn rotate_row(&mut self, row: usize, pixels: usize) {
        println!("rotate row y={} by {}", row, pixels);
        let mut temp : [char;WIDTH] = ['.';WIDTH];
        let start = (row * WIDTH) as usize;
        let stop = start + WIDTH;

        let mut data = &mut self.buffer[start..stop];

        temp.clone_from_slice(&data);

        for x in 0..WIDTH {
            data[x] = temp[(x + (WIDTH - pixels)) % WIDTH];
        }
    }

    fn rotate_col(&mut self, col: usize, pixels: usize) {
        println!("rotate column x={} by {}", col, pixels);
        let temp = (0..HEIGHT)
            .map(|y| self.buffer[col + y * WIDTH])
            .collect::<Vec<char>>();

        for y in 0..HEIGHT {
            self.buffer[col + y * WIDTH] = temp[(y + (HEIGHT - pixels)) % HEIGHT];
        }
    }

    fn display(&self) {
        for y in 0..HEIGHT {
            let start = (y * WIDTH) as usize;
            let stop = start + WIDTH;

            let row = self.buffer[start..stop]
                .iter()
                .cloned()
                .collect::<String>();

            println!("{}", row);
        }
    }

    fn lit(&self) -> usize {
        self.buffer
            .iter()
            .filter(|&x| *x == '#')
            .count()
    }
}

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
    let commands = Parser::load("input.txt").unwrap_or(Vec::new());
    let mut screen = Screen::new();

    for command in commands.iter() {
        screen.execute(command);
        screen.display();
    }

    println!("{}", screen.lit());
}
