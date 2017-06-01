use std::io::{Read,Bytes};
use std::fmt::Write;
#[cfg(test)]
use std::fs::File;
use std::iter::Iterator;

/// Format configuration of xxd output
pub struct Format {
    /// How many bytes per line, the default is 16.
    pub size: usize,
    /// How to pack xx bytes next to each other. For each multiple of the value, space will be
    /// inserted.
    /// The default is [2,4,8]. (Every two bytes one space, every 4 bytes a space and every 8 bytes
    /// a space).
    pub pack: Vec<usize>,
    /// A character to print in case of unprintable characters for ASCII output, the default is '.'.
    pub ascii_none: char,
    /// True to output ASCII. The default is true.
    pub ascii: bool,
    /// Gaps in formatting: offset{gaps.0}hex{gaps.1}ascii. The default is 2 spaces for both.
    pub gaps: (usize, usize),
}

impl Format {
    /// Create default format
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate hex_utils;
    ///
    /// let format = hex_utils::Format::default().unwrap();
    ///
    /// assert_eq!(16, format.size);
    ///
    /// ```
    pub fn default() -> Option<Format> {
        Some(
            Format {
                size: 16,
                pack: vec![2,4,8],
                ascii_none: '.',
                ascii: true,
                gaps: (2,2),
            }
            )
    }

    /// Get the format out of Option or get the default one.
    ///
    /// # Examples
    ///
    /// ```
    /// let format = Some(hex_utils::Format {
    ///                                 size: 9,
    ///                                 pack: vec![3,5],
    ///                                 ascii_none: '#',
    ///                                 ascii: true,
    ///                                 gaps: (2,3),
    ///                                 });
    ///
    /// let opt = hex_utils::Format::or_default(format);
    /// assert_eq!(9, opt.size);
    /// assert_eq!(vec![3,5], opt.pack);
    /// assert_eq!('#', opt.ascii_none);
    ///
    /// let opt = hex_utils::Format::or_default(None);
    /// assert_eq!(16, opt.size);
    /// assert_eq!(vec![2,4,8], opt.pack);
    /// assert_eq!('.', opt.ascii_none);
    ///
    /// ```
    pub fn or_default(format: Option<Format>) -> Format {
        match format {
            Some(o) => o,
            None    => Format::default().unwrap(),
        }
    }

    /// Create formatter function from this formatting configuration.
    pub fn formatter(&self) -> Box<Fn( (usize, String, Option<String>)) -> String> {
            let hex_size =
                    self.size * 2
                    + self.pack.iter().fold(0, | sum, val| sum + if *val == 0 { 0 } else { (self.size - 1) / val } );
            let gap0 = self.gaps.0;
            let gap1 = self.gaps.1;
            Box::new(move |line| {
                let (i,h,t) = line;
                let mut result = String::new();
                match t {
                    Some(t) => {
                        let _ = write!(&mut result, "{:0index$x}:{:gap0$}{:hex$}{:gap1$}{}", i,"", h.trim(),"", t, index=6, hex = hex_size, gap0 = gap0, gap1 = gap1);
                    }
                    None => {
                        let _ = write!(&mut result, "{:0index$x}:{:gap0$}{:hex$}", i, "", h.trim(), index=6, hex = hex_size, gap0 = gap0);
                    }
                }
                result
            })
    }
}

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
        match self.iter {
            ref mut iter => {
                match iter.take(self.format.size) {
                    ref mut iter => {
                        let mut any = false;
                        let mut hexed = String::new();
                        let mut ascii = if self.format.ascii { Some(String::new()) } else { None };
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
                        let offset = self.offset;
                        self.offset += self.format.size;
                        if any {
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
