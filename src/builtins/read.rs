use std::io::{self};

/// ██████╗ ███████╗ █████╗ ██████╗ 
/// ██╔══██╗██╔════╝██╔══██╗██╔══██╗
/// ██████╔╝█████╗  ███████║██║  ██║
/// ██╔══██╗██╔══╝  ██╔══██║██║  ██║
/// ██║  ██║███████╗██║  ██║██████╔╝
/// ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═════╝ 
/// Standard shell `read`
#[inline]
pub fn read(var: &str) -> String {
    let var = var.strip_prefix("$").expect("No $ before the variable");
    let mut var: String = std::env::var(var).unwrap_or_else(|_| String::new());

    io::stdin()
        .read_line(&mut var)
        .expect("Failed to read line from stdin");
    var = var.strip_suffix("\n").unwrap_or(&var).to_string();
    var
}
