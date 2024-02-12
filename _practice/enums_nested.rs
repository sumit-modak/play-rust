#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska, 
    Arizona, 
    Arkansas,
    California
}

enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5, 
        Coin::Dime => 10, 
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let val = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", val);
}