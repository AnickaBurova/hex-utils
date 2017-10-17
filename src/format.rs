/**
 * File: src/format.rs
 * Author: Anicka Burova <anicka.burova@gmail.com>
 * Date: 12.10.2017
 * Last Modified Date: 16.10.2017
 * Last Modified By: Anicka Burova <anicka.burova@gmail.com>
 */

use std::fmt::Write;

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

    pub fn get_hex_size(&self) -> usize {
        self.size * 2
        + self.pack.iter().fold(0, | sum, val| sum + if *val == 0 { 0 } else { (self.size - 1) / val } )
    }

    /// Create formatter function from this formatting configuration.
    pub fn formatter(&self) -> Box<Fn( (usize, String, Option<String>)) -> String> {
            let hex_size = self.get_hex_size();
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
