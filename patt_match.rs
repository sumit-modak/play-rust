fn main() {
    let val = [
        4, 13, 0, 32
    ];

    for i in val.iter() {
        match i {
            0 => println!("It is a zero val"),
            1 => println!("It is not prime"),
            2 | 3 | 7 | 11 | 13 => {
                println!("It is a prime no");
            },
            2 | 4 | 6 | 8 | 10 => {
                println!("It is a even no");
            },
            1 | 3 | 5 | 7 | 9 => {
                println!("It is a odd no");
            },
            _ => println!("Out of range")
        }
    }
}
