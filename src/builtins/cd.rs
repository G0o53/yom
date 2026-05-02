#[inline]
pub fn main(dir: &str) -> std::io::Result<()> {
    std::env::set_current_dir(&dir)?;
    Ok(())
}

