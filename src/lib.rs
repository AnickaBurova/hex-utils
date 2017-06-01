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
                    + self.pack.iter().fold(0, | sum, val| sum + if *val == 0 { 0 } else { (self.size - 1) / val } );
            Box::new(move |line| {
                let (i,h,t) = line;
                let mut result = String::new();
                let _ = write!(&mut result, "{:0index$x}: {:hex$}{:gap$}{}", i, h.trim(),"", t.unwrap(), index=6, hex = hex_size, gap = 2);
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

pub fn xxd_str<T: Read>( data: T, options: Option<Options>) -> String {
    let options = Options::or_default(options);
    let fmt = options.formatter();
    xxd(data, Some(options)).map(|line| fmt(line)).fold(String::new(), |mut res, line| {let _ = writeln!(&mut res, "{}", line);res})
}


#[test]
fn xxd_test() {
    let text = "[package]
name = \"hex-utils\"
version = \"0.1.5\"
authors = [\"Anicka Burova <anicka.burova@gmail.com>\"]

[dependencies]
itertools = \"*\"";
    let output = "000000: 5b70 6163  6b61 6765  5d0a 6e61  6d65 203d  [package].name =\n\
000010: 2022 6865  782d 7574  696c 7322  0a76 6572   \"hex-utils\".ver\n\
000020: 7369 6f6e  203d 2022  302e 312e  3522 0a61  sion = \"0.1.5\".a\n\
000030: 7574 686f  7273 203d  205b 2241  6e69 636b  uthors = [\"Anick\n\
000040: 6120 4275  726f 7661  203c 616e  6963 6b61  a Burova <anicka\n\
000050: 2e62 7572  6f76 6140  676d 6169  6c2e 636f  .burova@gmail.co\n\
000060: 6d3e 225d  0a0a 5b64  6570 656e  6465 6e63  m>\"]..[dependenc\n\
000070: 6965 735d  0a69 7465  7274 6f6f  6c73 203d  ies].itertools =\n\
000080: 2022 2a22                                    \"*\"\n";
    let options = Options { size: 16, pack: vec![2,4], dot: '.' };
    let xxd_output = xxd_str(text.as_bytes(), Some(options));
    assert_eq!(output.to_string(), xxd_output);
}
#[test]
fn print_test() {
    let mut file = File::open("Cargo.toml").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    let options = Options { size: 16, pack: vec![2,4], dot: '.' };
    //let fmt = options.formatter();

    //let res = xxd(content.as_bytes(), Some(options)).map(|line| fmt(line)).fold(String::new(), |mut res, line| { let _ = writeln!(&mut res, "{}", line);res});
    let res = xxd_str(content.as_bytes(), Some(options));
    println!("{}", res);

    //for line in xxd(content.as_bytes(), Some(options)).map(|line| fmt(line)) {
        //println!("{}", line);
    //}
}
