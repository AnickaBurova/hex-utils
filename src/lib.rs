extern crate itertools;

use std::io::{Read,Bytes};
use std::fmt::Write;
#[cfg(test)]
use std::fs::File;
use std::iter::Iterator;

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
    pub fn or_default(options: Option<Options>) -> Options {
        match options {
            Some(o) => o,
            None    => Options::default().unwrap(),
        }
    }

    pub fn formatter(&self) -> Box<Fn( (usize, String, Option<String>)) -> String> {
            let hex_size =
                    self.size * 2
                    + self.pack.iter().fold(0, | sum, val| sum + if *val == 0 { 0 } else { self.size / val } );
            Box::new(move |line| {
                let (i,h,t) = line;
                let mut result = String::new();
                let _ = write!(&mut result, "{:0index$}: {:hex$}{:18}", i, h, t.unwrap(), index=6, hex = hex_size);
                result
            })
    }
}

pub struct XxdLines<T: Read> {
    iter: Bytes<T>,
    options: Options,
    offset: usize,
}

impl<T: Read> XxdLines<T> {
}

impl<T: Read> Iterator for XxdLines<T> {
    type Item = (usize, String, Option<String>);
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter {
            ref mut iter => {
                match iter.take(self.options.size) {
                    ref mut iter => {
                        let mut any = false;
                        let mut hexed = String::new();
                        let mut texted = String::new();
                        let mut pack = vec![0;self.options.pack.len()];
                        for c in iter {
                            any = true;
                            let c = c.unwrap();
                            let _ = write!(&mut hexed, "{:02x}", c);
                            for i in 0..pack.len() {
                                pack[i] += 1;
                                if pack[i] == self.options.pack[i] {
                                    let _ = write!(&mut hexed, " ");
                                    pack[i] = 0;
                                }
                            }
                            if 32<= c && c < 127 {
                                let _ = write!(&mut texted, "{}", c as char);
                            } else {
                                let _ = write!(&mut texted, "{}", self.options.dot);
                            }
                        }
                        let offset = self.offset;
                        self.offset += self.options.size;
                        if any {
                            Some((offset,hexed,Some(texted)))
                        } else {
                            None
                        }
                    }
                }
            }
        }
    }
}

pub fn xxd<T: Read>( data: T, options: Option<Options>) -> XxdLines<T> {
    XxdLines {
        iter: data.bytes(),
        options: Options::or_default(options),
        offset: 0
    }
}


#[test]
fn it_works() {
    let mut file = File::open("Cargo.toml").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    let options = Options { size: 16, pack: vec![2,4], dot: '.' };
    let fmt = options.formatter();

    let res = xxd(content.as_bytes(), Some(options)).map(|line| fmt(line)).fold(String::new(), |mut res, line| { writeln!(&mut res, "{}", line);res});
    println!("{}", res);

    //for line in xxd(content.as_bytes(), Some(options)).map(|line| fmt(line)) {
        //println!("{}", line);
    //}
}
