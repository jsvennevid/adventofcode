use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::io;

struct Keypad {
    keys: Vec<char>,
    w: i32,
    h: i32,
    x: i32,
    y: i32
}

impl Keypad {
    fn new (keys: Vec<char>, w: i32, h: i32, x: i32, y: i32) -> Keypad {
        Keypad {keys: keys, w: w, h: h, x: x, y: y}
    }

    fn find(&self, path: &str) -> char {
        let mut x = self.x;
        let mut y = self.y;

        for i in path.chars() {
            match i {
                'L' => if self.test(x - 1, y) { x -= 1 },
                'R' => if self.test(x + 1, y) { x += 1 },
                'U' => if self.test(x, y - 1) { y -= 1 },
                'D' => if self.test(x, y + 1) { y += 1 },
                _ => ()
            }
        }

        self.keys[(x + y * self.w) as usize]
    }

    fn test(&self, x: i32, y: i32) -> bool {
        if x >= 0 {
            if x < self.w {
                if y >= 0 {
                    if y < self.h {
                        return self.keys[(x + y * self.w) as usize] != '\0'
                    }
                }
            }
        }
        false
    }
}

fn load(src: &str) -> Result<Vec<String>, io::Error> {
    let path = Path::new(src);
    let file = (File::open(path))?;
    let reader = BufReader::new(&file);

    reader.lines().collect()
}

fn main() {
    let input = load("input.txt").unwrap();

    let keypad1 = Keypad::new(vec![
        '1', '2', '3',
        '4', '5', '6',
        '7', '8', '9'
    ], 3, 3, 1, 1);

    let keypad2 = Keypad::new(vec![
        '\0','\0', '1','\0','\0',
        '\0', '2', '3', '4','\0',
         '5', '6', '7', '8', '9',
        '\0', 'A', 'B', 'C','\0',
        '\0','\0', 'D','\0','\0'
    ], 5, 5, 0, 2);

    let code1 : String = input.iter().map(|x| keypad1.find(x)).collect();
    println!("code 1: {}", code1);

    let code2 : String = input.iter().map(|x| keypad2.find(x)).collect();
    println!("code 2: {}", code2);
}
