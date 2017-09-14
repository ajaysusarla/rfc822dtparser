extern crate rfc822dtparser as dtparser;

use std::env;
use dtparser::*;

fn help(prgname: &str) {
    println!("RFC822 Date Time Parser");
    println!("Usage: {} <date-time-string>", prgname);
}

fn main() {
    dtparser::init().unwrap();

    let args: Vec<_> = env::args().collect();
    match args.len() {
        2 => {
            let dtstr: &str = args[1].as_ref();
            let t = dtparser::dtparser::RFC822DT::parse(dtstr);
        },
        _ => {
            help(args[0].as_ref());
        }
    }
}
