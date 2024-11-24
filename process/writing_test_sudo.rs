use std::{io::Write, process::Stdio};

fn main() {
    dbg!(std::process::id());

    const PASSWORD: &str = "/sprime/";
    let contents = "hello this is me\n";

    let mut child = std::process::Command::new("sudo")
        .args(["-k", "tee", "/usr/games/hello"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    println!("process spawned {}", child.id());
    dbg!(&child);
    std::thread::sleep(std::time::Duration::from_secs(60));

    // passing password as input
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(&PASSWORD.as_bytes())
        .unwrap();

    println!("password passed");
    dbg!(&child);
    std::thread::sleep(std::time::Duration::from_secs(30));

    // passing contents as input
    // child
    //     .stdin
    //     .as_mut()
    //     .unwrap()
    //     .write_all(&contents.as_bytes())
    //     .unwrap();

    child.wait_with_output().unwrap();

    // let mut f = File::create("/usr/games/bye").unwrap();
    // f.write_all(&contents.as_bytes()).unwrap();

    // this doesn't work!!
    // std::fs::write("/usr/games/hello.txt", contents).unwrap();
}
