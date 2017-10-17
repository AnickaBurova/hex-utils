/**
 * File: src/lib.rs
 * Author: Anicka Burova <anicka.burova@gmail.com>
 * Date: 12.10.2017
 * Last Modified Date: 16.10.2017
 * Last Modified By: Anicka Burova <anicka.burova@gmail.com>
 */
use std::io::{Read,Bytes};
use std::fmt::Write;
#[cfg(test)]
use std::fs::File;
use std::iter::Iterator;

mod format;
mod xhex;


pub use format::Format;

/// Iterator over lines returned by xxd.
///
/// Iterators returns (usize, String, Option<String>)
/// where ther first is the offset of the data,
/// the second is data formatted to hex output
/// and the third is Option to return data formatted as normal text.
///
pub struct XxdLines<T: Read> {
    iter: Bytes<T>,
    format: Format,
    offset: usize,
}

impl<T: Read> Iterator for XxdLines<T> {
    type Item = (usize, String, Option<String>);
    fn next(&mut self) -> Option<Self::Item> {
        let hex_size = self.format.get_hex_size();
        match self.iter {
            ref mut iter => {
                match iter.take(self.format.size) {
                    ref mut iter => {
                        let mut any = false;
                        let mut hexed = String::with_capacity(hex_size);
                        let mut ascii = if self.format.ascii { Some(String::with_capacity(self.format.size)) } else { None };
                        let mut pack = vec![0;self.format.pack.len()];
                        for c in iter {
                            any = true;
                            let c = c.unwrap();
                            let _ = write!(&mut hexed, "{:02x}", c);
                            for i in 0..pack.len() {
                                pack[i] += 1;
                                if pack[i] == self.format.pack[i] {
                                    let _ = write!(&mut hexed, " ");
                                    pack[i] = 0;
                                }
                            }
                            match ascii {
                                Some(ref mut ascii) => {
                                    if 32<= c && c < 127 {
                                        let _ = write!(ascii, "{}", c as char);
                                    } else {
                                        let _ = write!(ascii, "{}", self.format.ascii_none);
                                    }
                                }
                                None => (),
                            }
                        }
                        if any {
                            let offset = self.offset;
                            self.offset += self.format.size;
                            Some((offset,hexed,ascii))
                        } else {
                            None
                        }
                    }
                }
            }
        }
    }
}


/// Returns xxd iterator over the data configured by format.
/// Passing None to format will use defaults.
///
/// # Examples
/// ```
/// extern crate hex_utils;
///
///
/// let text = "The quick brown fox jumps over the lazy dog";
///
/// for (offset, hex, txt) in hex_utils::xxd(text.as_bytes(), None) {
///     println!("offset = {:03x} hex = {:60} txt = {}", offset, hex, txt.unwrap());
/// }
///
/// ```
/// ```
/// extern crate hex_utils;
///
///
/// let text = "The quick brown fox jumps over the lazy dog";
/// let format = hex_utils::Format {
///     size: 18,
///     pack: vec![3,6],
///     ascii_none: '-',
///     ascii: true,
///     gaps:(4,2),
/// };
///
/// let fmt = format.formatter();
///
/// for line in hex_utils::xxd(text.as_bytes(), Some(format)) {
///     println!("{}", fmt(line));
/// }
/// ```
pub fn xxd<T: Read>( data: T, format: Option<Format>) -> XxdLines<T> {
    XxdLines {
        iter: data.bytes(),
        format: Format::or_default(format),
        offset: 0
    }
}

/// Returns the whole xxd output as one string.
///
/// # Examples
///
/// ```
/// extern crate hex_utils;
///
///
/// let text = "The quick brown fox jumps over the lazy dog";
///
/// println!("{}", hex_utils::xxd_str(text.as_bytes(), None));
///
/// ```
pub fn xxd_str<T: Read>( data: T, format: Option<Format>) -> String {
    let format = Format::or_default(format);
    let fmt = format.formatter();
    xxd(data, Some(format)).map(|line| fmt(line)).fold(String::new(), |mut res, line| {let _ = writeln!(&mut res, "{}", line);res})
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
    let format = Format {
        size: 16,
        pack: vec![2,4],
        ascii_none: '.',
        ascii: true,
        gaps:(1,2),
    };
    let xxd_output = xxd_str(text.as_bytes(), Some(format));
    assert_eq!(output.to_string(), xxd_output);
}
#[test]
fn print_test() {
    let mut file = File::open("Cargo.toml").unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    let format = Format {
        size: 16,
        pack: vec![2,4],
        ascii_none: '.',
        ascii: true,
        gaps: (1,4)
    };
    //let fmt = format.formatter();

    //let res = xxd(content.as_bytes(), Some(format)).map(|line| fmt(line)).fold(String::new(), |mut res, line| { let _ = writeln!(&mut res, "{}", line);res});
    //let res = xxd_str(content.as_bytes(), Some(format));
    let res = xxd_str(content.as_bytes(), Some(format));
    println!("{}", res);

    //for line in xxd(content.as_bytes(), Some(format)).map(|line| fmt(line)) {
        //println!("{}", line);
    //}
}
