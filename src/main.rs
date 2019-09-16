extern crate clap;
use clap::{App, Arg};
use convert::Convert;

mod convert;

fn main() {
    let matches = App::new("cha")
        .version("0.1")
        .about("cha: try to find files")
        .author("betta")
        .arg(Arg::with_name("INPUT")
            .help("Please input some string")
            .index(1)
            .required(true))
        .get_matches();

    let st: String = matches.value_of("INPUT").unwrap().to_string();
    Convert::new(st).format_and_print();
}
