use std::pin::Pin;

fn main() {
    let mut s = String::from("hello");
    let x = &s;
    manipulates_str(Pin::new(&mut s));
}

fn manipulates_str(mut s: Pin<&mut String>) {
    s.push('!');
    println!("{s}");
}

// fn manipulates_str(mut s: String) -> String {
//     s.push('!');
//     s
// }
