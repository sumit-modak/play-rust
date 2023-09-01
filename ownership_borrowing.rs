fn main() {
    // ----- Owenrship Rules -----
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 2. When the owner goes out of scope, the value will be dropped.
    {
        let ss: &str = "hello";
        let sh: String = String::from("hello");
    } // the value of s is dropped 
}