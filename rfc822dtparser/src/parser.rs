//
// Copyright 2017 (c) Partha Susarla
//


use std::str;

#[derive(Clone)]
pub struct InputStr<'i> {
    chars: str::Chars<'i>,
}

impl<'i> InputStr<'i> {
    pub fn new(inputstr: &'i str) -> Self {
        let s = inputstr.trim();
        InputStr { chars: s.chars() }
    }
}
