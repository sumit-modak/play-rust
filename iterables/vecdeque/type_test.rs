use std::collections::VecDeque;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let s1 = "hello".to_string();
    let s2 = "world".to_string();
    let mut val: VecDeque<&String> = VecDeque::new();
    val.push_back(&s1);
    val.push_back(&s2);
    println!("{:?}", val);
    print_type_of(&val);
}
