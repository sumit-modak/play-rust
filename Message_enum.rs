enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let messages: Vec<Message> = vec![
        Message::Quit,
        Message::Move {
            x: 7,
            y: 8,
        },
        Message::Write (
            String::from("hello")
        ),
        Message::ChangeColor(
            32, 64, 128
        )
    ];

    for i in messages.iter() {
        match i {
            Message::Quit => println!("Bro just quit!"),
            Message::Move {x: x, y: y @ (7 | 8 | 9)} => {
                println!("Move to {}, {}", x, y);
            },
            Message::Write (msg) => println!("{}", msg),
            Message::ChangeColor (r, g, b) => {
                println!("Your color is {}, {}, {}", r, g, b);
            },
            _ => println!("other movements are not allowed")
        }
    }
}