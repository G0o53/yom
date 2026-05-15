use std::process::Command;
use std::process::Stdio;
use std::process::Child;

/// This function executes the path given
/// and waits for the child created to
/// complete it's task, it takes the 
/// entire line.
pub fn waitfor(path: &str) {
    let split = shell_words::split(path).expect("Invalid command syntax");
    let program = &split[0];
    let args = &split[1..];

    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start");

    child.wait().unwrap();
}


/// This function takes the entire line,
/// it does not wait for the child
/// created to finish; in the main 
/// function, it ignores the returned
/// Child, use it for ephermal scripts
/// mainly instead of using it for 
/// long running scripts.
#[inline]
pub fn background(command: &str) -> Child {
    let path = command.trim_start_matches("& ");

    let split = shell_words::split(path).expect("Invalid command syntax");
    let program = &split[0];
    let args = &split[1..];

    Command::new(program)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start")
}

/// This function is the standard
/// shell exec, it replaces the 
/// process (yom) with the path 
/// given. It takes the whole line.
#[inline]
pub fn exec(command: &str){
    let path = command.trim_start_matches("exec ");
    
    use std::process::Command;
    use std::os::unix::process::CommandExt;

    let split = shell_words::split(path).expect("Invalid command syntax");
    let program = &split[0];
    let args = &split[1..];

    let _ = Command::new(program)
        .args(args)
        .exec();
}
