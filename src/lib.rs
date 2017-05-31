extern crate itertools;

use itertools::Itertools;
use std::io::Read;
use std::fmt::Write;
use std::fs::File;
use std::io::prelude::*;

pub struct Options {
    size: usize,
    pack: Vec<usize>,
    dot: char,
}

impl Options {
    pub fn default() -> Option<Options> {
        Some(
            Options {
                size: 16,
                pack: vec![2,4],
                dot: '.',
            }
            )
    }
}

pub fn xxd_str<T: Read>(data: T, options: Option<Options>) -> String {
    let hex_size = match &options {
        &Some(ref o) =>
            o.size * 2
            + o.pack.iter().fold(0, | sum, val| if *val == 0 { 0 } else { o.size / val } ),
            //+ if o.pack.0 > 0 {(o.size / o.pack.0)} else {0}
            //+ if o.pack.1 > 0 {(o.size / o.pack.1)} else {0},
        &None        => 52,
    };
    let mut result = String::new();
    for (i,h,t) in xxd(data,options).into_iter() {
        writeln!(&mut result, "{:0index$}: {:hex$}  {:16}", i.unwrap(), h.unwrap(), t.unwrap(), index=6, hex = hex_size);
    }

    result
}


pub fn xxd<T: Read>(data: T, options: Option<Options>) -> Vec<(Option<usize>, Option<String>, Option<String>)> {
    let mut result = Vec::new();
    let options = match options {
        Some(o) => o,
        None    => Options::default().unwrap(),
    };
    let size = options.size;
    let mut index = 0;
    for line in &data.bytes().chunks(size) {
        let mut hexed = String::new();
        let mut texted = String::new();
        let mut pack = vec![0;options.pack.len()];
        for c in line {
            let c = c.unwrap();
            write!(&mut hexed, "{:02x}", c);
            for i in 0..pack.len() {
                pack[i] += 1;
                if pack[i] == options.pack[i] {
                    write!(&mut hexed, " ");
                    pack[i] = 0;
                }
            }
            if 32<= c && c < 127 {
                write!(&mut texted, "{}", c as char);
            } else {
                write!(&mut texted, "{}", options.dot);
            }
        }
        result.push((Some(index), Some(hexed), Some(texted)));
        index += size;
    }
    result
}


#[test]
fn it_works() {
    let mut file = File::open("Cargo.toml").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content);
    let options = Options { size: 16, pack: vec![1,4], dot: '.' };

    println!("{}", xxd_str(content.as_bytes(), Some(options)));
}
