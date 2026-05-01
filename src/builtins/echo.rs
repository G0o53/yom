use std::io::Write;

pub fn main<W: Write>(str: &str, out: &mut W) {
    let _ = write!(out, "{str}\n");
} 
