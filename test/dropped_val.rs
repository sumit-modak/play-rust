fn main() {
    let x: i32;
    {
        x = 4;
        println!("{}", x);
    }
    println!("{}", x); // Not getting dropped OMG!!
}