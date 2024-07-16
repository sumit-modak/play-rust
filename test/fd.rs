// use std::io::Write;
use std::process::{Command, Stdio};

// fn main() {
//     let output = Command::new("rev")
//         .stdin(Stdio::inherit())
//         .stdout(Stdio::piped())
//         .output()
//         .expect("Failed to execute command");

//     print!("You piped in the reverse of: ");
//     std::io::stdout().write_all(&output.stdout).unwrap();
// }

fn main() {
    println!("PID: {}", std::process::id());
    let mut sleep = Command::new("sleep")
        .arg("60")
        // child inherits parent stdin file descriptor
        .stdin(Stdio::inherit())
        // creates a new file descriptor in parent process
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn sleep");

    println!("{:?}", sleep.id());
    println!("{:?}", sleep.wait());
}
