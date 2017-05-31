extern crate itertools;

use itertools::Itertools;
use itertools::structs::{Chunks,IntoChunks};

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
}

pub fn xxd_str<T: Read>(data: T, options: Option<Options>) -> String {
    let hex_size = match &options {
        &Some(ref o) =>
            o.size * 2
            + o.pack.iter().fold(0, | sum, val| sum + if *val == 0 { 0 } else { o.size / val } ),
        &None        => 52,
    };
    let mut result = String::new();
    for (i,h,t) in xxd(data,options).into_iter() {
        let _ = writeln!(&mut result, "{:0index$}: {:hex$}{:18}", i, h, t.unwrap(), index=6, hex = hex_size);
    }

    result
}

//pub fn xxd_test<'a,T : Read + 'a>(data: &'a T, options: Option<Options>) -> XxdLines<'a,Bytes<T>> {
    //let o = Options::or_default(options);
    //XxdLines {
        //chunks: &data.bytes().chunks(o.size).into_iter(),
        //offset: 0,
        //options: o,
    //}
//}

//pub struct XxdLines <'a,T: Iterator + 'a> {
    //chunks: &'a Chunks<'a, Bytes<T>>,
    //offset: usize,
    //options: Options,
//}

//impl<'a, T: Iterator + 'a> Iterator for XxdLines<'a,T> {
    //type Item = (usize, String, Option<String>);
    //fn next(&mut self) -> Option<Self::Item> {
        //let size = self.options.size;
        //match self.chunks.next() {
            //Some(line) => {
                //for i in &line.iter() {
                        //println!("{:?}", i);
                //}
                //None
                ////let mut hexed = String::new();
                ////let mut texted = String::new();
                ////let mut pack = vec![0;self.options.pack.len()];
                ////for c in line {
                    ////let _ = write!(&mut hexed, "{:02x}", c);
                    ////for i in 0..pack.len() {
                        ////pack[i] += 1;
                        ////if pack[i] == self.options.pack[i] {
                            ////let _ = write!(&mut hexed, " ");
                            ////pack[i] = 0;
                        ////}
                    ////}
                    ////if 32<= c && c < 127 {
                        ////let _ = write!(&mut texted, "{}", c as char);
                    ////} else {
                        ////let _ = write!(&mut texted, "{}", self.options.dot);
                    ////}
                ////}
                ////let offset = self.offset;
                ////self.offset += size;
                ////Some((offset, hexed, Some(texted)))
            //}
            //None        => None
        //}
    //}
//}



pub fn xxd<T: Read>(data: T, options: Option<Options>) -> Vec<(usize, String, Option<String>)> {
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
            let _ = write!(&mut hexed, "{:02x}", c);
            for i in 0..pack.len() {
                pack[i] += 1;
                if pack[i] == options.pack[i] {
                    let _ = write!(&mut hexed, " ");
                    pack[i] = 0;
                }
            }
            if 32<= c && c < 127 {
                let _ = write!(&mut texted, "{}", c as char);
            } else {
                let _ = write!(&mut texted, "{}", options.dot);
            }
        }
        result.push((index, hexed, Some(texted)));
        index += size;
    }
    result
}

pub struct XxdLines<T: Read> {
    iter: IntoChunks<Bytes<T>>
}

impl<T: Read> Iterator for XxdLines<T> {
    type Item = (usize, String, Option<String>);
    fn next(&mut self) -> Option<Self::Item> {

        None
    }
}

pub fn xxd_test<T: Read>( data: T, options: Option<Options>) -> XxdLines<T> {
    for chunk in &data.bytes().chunks(4) {
    }
    //XxdLines{iter: data.bytes().chunks(3)}
}


#[test]
fn it_works() {
    let mut file = File::open("Cargo.toml").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    let options = Options { size: 16, pack: vec![1,4], dot: '.' };

    //println!("{}", xxd_str(content.as_bytes(), Some(options)));

    for (i,h,t) in xxd_test("hello world".as_bytes(), Some(options)) {
        println!("{:06}: {:52}{:18}", i, h, t.unwrap());
    }
}
