use std::io::Write;
use std::process::Stdio;

fn main() {
    const PASSWD: &str = "HELLO";

    let mut child = std::process::Command::new("sudo")
        .args(["-S", "ls", "-al", "/root"])
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    writeln!(child.stdin.as_mut().unwrap(), "{}", PASSWD);

    let output = child.wait_with_output().unwrap();

    println!("{}", output.status);
    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(output.stderr).unwrap());
}
