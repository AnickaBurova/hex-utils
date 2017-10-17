/**
 * File: src/xhex.rs
 * Author: Anicka Burova <anicka.burova@gmail.com>
 * Date: 12.10.2017
 * Last Modified Date: 16.10.2017
 * Last Modified By: Anicka Burova <anicka.burova@gmail.com>
 */

use format::Format;
use std::io::{Read,Bytes};
use std::fmt::Write;


/// Tokens
pub enum Token {
    BlockStart,
    Next,
    BlockEnd,
}

pub trait Tokenizer {
    fn get_token(&mut self, token: Token) -> Option<(Option<String>, Option<u8>)>;

}

use std::cell::RefCell;
use std::rc::Rc;

pub struct ExXxdLines {
    format: Format,
    offset: usize,
    tokens: Rc<RefCell<Tokenizer>>,
    end: bool,
}

macro_rules! write_prefix {
    ($prefix: ident, $hexed: ident, $ascii: ident) => {
        match $prefix {
            Some(prefix) => {
                $hexed.push_str(&prefix);
                $ascii = $ascii.and_then(|mut ascii| { ascii.push_str(&prefix); Some(ascii)});
            }
            None         => (),
        }
    }
}

impl Iterator for ExXxdLines {
    type Item = (usize, String, Option<String>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            return None;
        }

        let hex_size = self.format.get_hex_size();
        match self.tokens.borrow_mut() {
            ref mut tokens => {
                let mut any = false;
                let mut hexed = String::with_capacity(hex_size);
                let mut ascii = if self.format.ascii { Some(String::with_capacity(self.format.size)) } else { None };
                // line per line
                match tokens.get_token(Token::BlockStart) {
                    Some((prefix, _)) => {
                        write_prefix!(prefix, hexed, ascii);
                    }
                    _ => (),
                }

                for i in 0..self.format.size {
                    match tokens.get_token(Token::Next) {
                        Some((prefix, Some(ch))) => {
                            any = true;
                            write_prefix!(prefix, hexed, ascii);
                            let _ = write!(&mut hexed, "{:02x}", ch);
                            match ascii {
                                Some(ref mut ascii) => {
                                    if 32<= ch && ch < 127 {
                                        let _ = write!(ascii, "{}", ch as char);
                                    } else {
                                        let _ = write!(ascii, "{}", self.format.ascii_none);
                                    }
                                }
                                None => (),
                            }
                        }
                        _             => {
                            self.end = true;
                            break;
                        }
                    }
                }

                match tokens.get_token(Token::BlockEnd) {
                    Some((prefix, _)) => {
                            write_prefix!(prefix, hexed, ascii);
                    }
                    _ => (),
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


#[cfg(test)]
mod testing {
    use xhex::*;
    struct SimpleTokenizer {
        text: Vec<u8>,
        index: usize,
    }

    impl Tokenizer for SimpleTokenizer {
        fn get_token(&mut self, token: Token) -> Option<(Option<String>, Option<u8>)> {
            match token {
                Token::BlockStart => None,
                Token::BlockEnd => None,
                Token::Next => {
                    let index = self.index;
                    if index < self.text.len() {
                        self.index += 1;
                        Some((None, Some(self.text[index])))
                    } else {
                        None
                    }
                }
            }
        }
    }
    #[test]
    fn test_xhex() {
    
    }
}
