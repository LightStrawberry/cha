extern crate crypto;
extern crate urlencoding;
extern crate term;

use std::io;
use crate::App;

use crypto::md5::Md5;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use urlencoding::encode;
use urlencoding::decode;


pub struct Convert {
    pub st: String,
}

impl Convert {
    pub fn new(st: String) -> Convert {
        Convert { st }
    }

    pub fn format_and_print(&self) {
        let mut terminal = term::stdout().unwrap();
        terminal.fg(term::color::BRIGHT_GREEN).unwrap();
        terminal.attr(term::Attr::Bold).unwrap();

        // raw data
        println!("raw string     {}", &self.st);

        terminal.reset().unwrap();
        terminal.fg(term::color::BRIGHT_CYAN).unwrap();

        // md5 encode
        let mut md5 = Md5::new();
        md5.input_str(&self.st);
        let md5_st = md5.result_str();
        println!("md5 encode     {}", md5_st);

        // sha256 encode
        let mut sha256 = Sha256::new();
        sha256.input_str(&self.st);
        let sha256_st = sha256.result_str();
        println!("sha256 encode  {}", sha256_st);

        // url encode
        let url_st = encode(&self.st);
        println!("url encode     {}", url_st);

        // url decode
        let decoded = decode(&self.st);
        println!("url decode     {}", decoded.unwrap());

        // utf 16
        let s = &self.st;
        let uni_st_c: String = s.escape_unicode().collect();
        let uni_st: String = uni_st_c.replace("{", "").replace("}", "");
        println!("unicode decode {}", uni_st);

        terminal.reset().unwrap();
    }
}
