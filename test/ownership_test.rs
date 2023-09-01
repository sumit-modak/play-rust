fn print(s: String) {
    println!("{}", s);
    // s getting dropped
}

fn main() {
    let st: String = String::from("hello");
    print(st);  // ownership transferred
    let a: &str = &st;  // creating reference of st which is a fucking error
    println!("{}", a);  
}