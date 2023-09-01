use ndarray::arr2;
use io::stdio;

fn main() {
    let x: [[i32; 1000]; 1000];
    let y: [[i32; 1000]; 1000];
    let mut input: String = String::new();
    match io::stdin().readline(&mut input) {
        Ok(val) => {
            
        },
        Err(val) => {
            panic!("");
        }
    }
    let sum = &x + &y;
    let dif = &x - &y;
}

fn input(arr: &[i32]) {
    for i in arr.iter() {
        
    }
}