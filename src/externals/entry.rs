use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::io::copy;

pub fn main<W: Write>(path: &str, out: &mut W) {
    let split = shell_words::split(path).expect("Invalid command syntax");
    let program = &split[0];
    let args = &split[1..];

    let mut child = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start");

    if let Some(mut stdout) = child.stdout.take() {
        copy(&mut stdout, out).unwrap();
    }
    child.wait().unwrap();
}
