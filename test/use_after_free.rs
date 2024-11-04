#![allow(unused_assignments)]

fn main() {
    let x = 5;
    let y = 5;
    let mut x_ptr: *const i32 = &x;
    let mut y_ptr: *const i32 = &y;
    {
        let x = 10;
        x_ptr = &x;
    }
    {
        let y = 10;
        y_ptr = &y;
    }
    println!("{}/{}", &x == &y, x_ptr == y_ptr);
}
