use std::process::Command;
use std::process::Stdio;

/// ██╗    ██╗ █████╗ ██╗████████╗███████╗ ██████╗ ██████╗ 
/// ██║    ██║██╔══██╗██║╚══██╔══╝██╔════╝██╔═══██╗██╔══██╗
/// ██║ █╗ ██║███████║██║   ██║   █████╗  ██║   ██║██████╔╝
/// ██║███╗██║██╔══██║██║   ██║   ██╔══╝  ██║   ██║██╔══██╗
/// ╚███╔███╔╝██║  ██║██║   ██║   ██║     ╚██████╔╝██║  ██║
///  ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝   ╚═╝   ╚═╝      ╚═════╝ ╚═╝  ╚═╝
/// This function executes the path given
/// and waits for the child created to
/// complete it's task, it takes the 
/// entire line.
pub fn waitfor(path: &str) {
    let split = shell_words::split(path).expect("Invalid command syntax");
    let program = &split[0];
    let args = &split[1..];
    if program.starts_with("$") {
        let program = program.strip_prefix("$").unwrap();

        let mut child = Command::new(std::env::var_os(program).unwrap())
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to start");

        child.wait().unwrap();

    } else {
        let mut child = Command::new(program)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to start");

        child.wait().unwrap();
    }
}

/// ██████╗  █████╗  ██████╗██╗  ██╗ ██████╗ ██████╗  ██████╗ ██╗   ██╗███╗   ██╗██████╗ 
/// ██╔══██╗██╔══██╗██╔════╝██║ ██╔╝██╔════╝ ██╔══██╗██╔═══██╗██║   ██║████╗  ██║██╔══██╗
/// ██████╔╝███████║██║     █████╔╝ ██║  ███╗██████╔╝██║   ██║██║   ██║██╔██╗ ██║██║  ██║
/// ██╔══██╗██╔══██║██║     ██╔═██╗ ██║   ██║██╔══██╗██║   ██║██║   ██║██║╚██╗██║██║  ██║
/// ██████╔╝██║  ██║╚██████╗██║  ██╗╚██████╔╝██║  ██║╚██████╔╝╚██████╔╝██║ ╚████║██████╔╝
/// ╚═════╝ ╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═══╝╚═════╝                                                                                  
/// This function needs the end cleaned,
/// it does not wait for the child
/// created to finish; in the main 
/// function, it ignores the returned
/// Child, use it for ephermal scripts
/// mainly instead of using it for 
/// eternal scripts.
#[inline]
pub fn background(command: &str) {

    let split = shell_words::split(command).expect("Invalid command syntax");
    let program = &split[0];
    let args = &split[1..];
    if program.starts_with('$') {
        let program = program.strip_prefix('$').unwrap();

         let _ = Command::new(std::env::var_os(program).unwrap())
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to start");

    } else {
        let _ = Command::new(program)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to start");
 
    }
}

/// ███████╗██╗  ██╗███████╗ ██████╗
/// ██╔════╝╚██╗██╔╝██╔════╝██╔════╝
/// █████╗   ╚███╔╝ █████╗  ██║     
/// ██╔══╝   ██╔██╗ ██╔══╝  ██║     
/// ███████╗██╔╝ ██╗███████╗╚██████╗
/// ╚══════╝╚═╝  ╚═╝╚══════╝ ╚═════╝ 
/// This function is the standard
/// shell exec, it replaces the 
/// process (yom) with the path 
/// given. It takes the whole line.
#[inline]
pub fn exec(command: &str){
    let path = command.strip_prefix("exec ").unwrap();
    
    use std::process::Command;
    use std::os::unix::process::CommandExt;

    let split = shell_words::split(path).expect("Invalid command syntax");
    let program = &split[0];
    if program.starts_with("$") {
        let program = program.strip_prefix("$").unwrap();
        let args = &split[1..];

        let _ = Command::new(std::env::var_os(program).unwrap())
            .args(args)
            .exec();

    } else {
        let args = &split[1..];

        let _ = Command::new(program)
            .args(args)
            .exec();

    }
}
