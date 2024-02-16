#[derive(Debug)]
struct MyStr<'a> {
    val: Option<&'a str>,
}

fn main() {
    let mut mystr = MyStr { val: Some("hello") };
    // if let Some(ref mut value) = mystr.val {
    //     *value = "world";
    // }
    if let Some(value) = &mut mystr.val {
        *value = "world";
    }
    println!("{:?}", mystr);
}
