use std::io::{self, Write};

/// Standard shell `read`
#[inline]
pub fn read<W: Write>(prompt: &str, out: &mut W) -> String {
    let _ = write!(out, "{prompt}");
    let _ = out.flush();

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line from stdin");
    buffer = buffer.strip_suffix("\n").unwrap_or(&buffer).to_string();
    buffer
}
