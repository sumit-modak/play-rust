fn main() {
    let mut s = "hello";
    let refs = &s;
    println!("{}", s);
    s = "wtf";      // Error! s cannot be mutated because it is borrowed
    println!("{} {}", s, refs);
}
