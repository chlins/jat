extern crate serde_json;
extern crate colored_json;
use serde_json::{from_str, Value};
use std::io::stdout;
use std::io::Write;
use std::fs;
use std::env;
use std::io::{self, BufReader, BufRead};

pub fn main() -> ::std::result::Result<(), Box<dyn (::std::error::Error)>> {
    let input = env::args().nth(1);
    let mut reader: Box<dyn BufRead> = match input {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap()))
    };
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let value: Value = from_str(&contents)?;
    let out = stdout();
    {
        let mut out = out.lock();
        colored_json::write_colored_json(&value, &mut out)?;
        out.flush()?;
    }

    Ok(())
}
