struct Color(u8, u8, u8);

#[allow(dead_code, non_snake_case)]
impl Color {
    fn isRed(&self) {
        if self.0 == 255 && self.1 == 0 && self.2 == 0 {
            println!("the color is red")
        } else {
            println!("the color is not red")
        }
    }
    fn isGreen(&self) {
        if self.0 == 0 && self.1 == 255 && self.2 == 0 {
            println!("the color is green")
        } else {
            println!("the color is not green")
        }
    }
    fn isBlue(&self) {
        if self.0 == 0 && self.1 == 0 && self.2 == 255 {
            println!("the color is blue")
        } else {
            println!("the color is not blue")
        }
    }
}

fn main() {
    let c1 = Color(255, 0, 0);
    let c2 = Color(73, 28, 9);
    c1.isRed();
    c2.isGreen();
}