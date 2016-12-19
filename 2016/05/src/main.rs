extern crate crypto;
extern crate itertools;

use std::env;
use crypto::md5::Md5;
use crypto::digest::Digest;
use std::collections::HashMap;

fn main() {
    let raw_input = env::args().nth(1).unwrap_or("".to_string());
    let input = raw_input.as_bytes();
    let mut index : i64 = -1;
    let mut hash = Md5::new();

    let mut output = String::new();
    let mut output2 = HashMap::new();

    while {output.len() < 8 || output2.len() < 8} {
        let mut temp = [0; 16];

        while {
            index += 1;
            hash.reset();

            hash.input(input);
            hash.input(index.to_string().as_bytes());

            hash.result(&mut temp);

            (temp[0] as i32 + temp[1] as i32 + (temp[2] >> 4) as i32) != 0
        } {}

        let out = format!("{:x}", temp[2] & 0xf);
        let out2 = format!("{:x}", temp[3] >> 4);
        let pos = (temp[2] & 0x0f) as i32;

        if output.len() < 8 {
            output.push_str(&out);
        }

        if pos < 8 && !output2.contains_key(&pos) {
            output2.insert(pos, out2);
        }
    }

    println!("P1 {}", output);

    let empty = " ".to_string();
    let temp = (0..8)
        .map(|i| output2.get(&(i as i32)).unwrap_or(&empty))
        .cloned()
        .collect::<Vec<_>>()
        .concat();

    println!("P2 {}", temp);

}
