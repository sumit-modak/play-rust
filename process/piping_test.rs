use std::process::Command;
use std::process::Stdio;

pub fn main() {
    println!("PID: {}", std::process::id());
    test_piping();
    hello();
}

pub fn hello() {
    let mut sleep = Command::new("sleep")
        .arg("60")
        // .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn sleep");

    println!("{:?}", sleep.id());
    println!("{:?}", sleep.wait());
}

// fd[0] = read end
// fd[1] = write end
pub fn test_piping() {
    let mut echo = Command::new("echo")
        .arg("hello world!")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn echo");

    let tr_stdin = Stdio::from(echo.stdout.take().unwrap());

    let tr = Command::new("tr")
        .args(["a-z", "A-Z"])
        .stdin(tr_stdin)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn tr");

    let (echo_result, tr_output) = (echo.wait(), tr.wait_with_output());

    println!("{:?}", echo_result.unwrap().success());

    let tr_output = tr_output.expect("failed to await tr");
    println!("{:?}", tr_output.status.success());

    println!("{:?}", String::from_utf8(tr_output.stdout).unwrap());
}
