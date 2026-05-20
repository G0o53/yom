use std::io::Write;

/// ██████╗ ██╗    ██╗██████╗ 
/// ██╔══██╗██║    ██║██╔══██╗
/// ██████╔╝██║ █╗ ██║██║  ██║
/// ██╔═══╝ ██║███╗██║██║  ██║
/// ██║     ╚███╔███╔╝██████╔╝
/// ╚═╝      ╚══╝╚══╝ ╚═════╝ 
/// Standard shell `pwd`
#[inline]
pub fn pwd<W: Write>(out: &mut W) -> std::io::Result<()> {
    let pwd = std::env::current_dir()?;
    let _ = write!(out, "{}\n", pwd.display())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(())
}
