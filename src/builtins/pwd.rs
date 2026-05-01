use std::io::Write;

pub fn main<W: Write>(out: &mut W) -> std::io::Result<()> {
    let pwd = std::env::current_dir()?;
    let _ = write!(out, "{}\n", pwd.display())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(())
}
