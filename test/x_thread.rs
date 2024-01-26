fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn stuff(thing: &mut String) {
    let _a: &mut _ = thing;
    let _b: &String = thing;
    let _c: &mut _ = thing;
    print_type_of(&_c);
}

fn stuff2(thing: &mut String) {
    print_type_of(&thing);
    let _a: &String = thing;
    print_type_of(&_a);
    let _b = thing;
    // let _c = thing;    // error
    print_type_of(&_b);
}

fn main() {
    stuff(&mut "einmien".to_string());
    stuff2(&mut "gmenie".to_string());
}
