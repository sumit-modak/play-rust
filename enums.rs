enum Color {
    Red, 
    Blue, 
    Green, 
    Yellow
}

impl Color {
    fn green_part(&self) -> bool {
        match self {
            Color::Blue => true,
            Color::Yellow => true,
            _ => false
        }
    }
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("red"), 
        Color::Blue => println!("blue"), 
        Color::Yellow => println!("yellow"),
        Color::Green => println!("green")
    }
}

fn main() {
    let col = Color::Red;
    println!(" {} ", col.green_part());
}