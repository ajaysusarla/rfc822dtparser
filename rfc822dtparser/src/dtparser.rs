// Copyright 2017, Partha Susarla

//use parser::*;

pub struct RFC822DT {
}

impl RFC822DT {
    pub fn parse(source: &str) -> Result<(), ()> {
        let chars:Vec<char> = source.chars().collect();
        for c in chars {
            println!("{}", c);
        }
        Ok(())
    }
}


#[cfg(test)]
mod test {
}
