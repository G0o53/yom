/// ▄█████ ████▄  
/// ██     ██  ██ 
/// ▀█████ ████▀  
/// Standerd shell `cd`
#[inline]
pub fn cd(dir: &str) -> std::io::Result<()> {
    std::env::set_current_dir(&dir)?;
    Ok(())
}

