fn main() {
    // datatypes 
    let st1: &str = "hello";
    let st2: String = String::from("world");
    let var: i64 = 38749;
    let mut marks: f64 = 23.35;
    let c: char = 'A';
    println!("{} {} {} {} {}", st1, st2, var, marks, c);

    marks = 58.237879;                              // mutation 
    const PI: f32 = 3.14;                           // constant declaration
    let st1: &str = "wanna get naughty with you";   // redefining immutable datatype with samename
    println!("{} {} \n {} ", marks, PI, st1);

    // non decimal systems
    let hex: i32 = 0x738AB;
    let oct: i32 = 0o32771;
    let bin: i32 = 0b1111_0000;
    println!("{} {} {} ", hex, bin, oct);

    // tuples
    let tup: (&str, i32, bool) = ("hi", 8, true);
    println!("{} {} {} ", tup.0, tup.1, tup.2);

    // arrays
    let arr: [i32; 3] = [256, 300, 404];
    let byte: [i32; 8] = [0; 8];
    println!("{} {} {}", arr[0], arr[1], arr[2]);
    for i in byte.iter() {
        print!("{}", i);
    }

    // conditions
    let int: i32 = if tup.2 {6} else {5};
    if c == 'A' {
        println!(" {}\n how did you know that", int);
    } else {
        println!(" {}\n dont try to be oversmart", int);
    }

    // loop 
    let mut counter: i32 = 0;
    let res: i32 = loop {
        counter += 1;
        if counter == 4 {
            break counter;
        }
    };
    println!("{} {}", counter, res);

    // range based loop
    for i in 0..8 {
        print!("{} ", i);
    } println!();

    // while loop
    let mut a: i8 = 15;
    while a < 16 && a > 0 {
        a -= 2;
        print!("{} ", a);
    } println!();

    let _ttt: (&str, i64) = my_fuckin_fn(var);
    println!("{} {} {}", _ttt.0, _ttt.1, "oaienstaoesonta");
}

fn my_fuckin_fn(bro_val: i64) -> (&'static str, i64) {
    ("aaaaahhh!!!", bro_val * 3)
}