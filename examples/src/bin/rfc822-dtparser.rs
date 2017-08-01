extern crate rfc822dtparser as dtparser;

use std::env;
use dtparser::*;

fn main() {
    dtparser::init().unwrap();

    let args: Vec<_> = env::args().collect();
    let dtstr: &str = if args.len() == 2 {
        args[1].as_ref()
    } else {
        panic!("Usage: {} <date-time-string>", args[0]);
    };
}
