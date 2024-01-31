struct Human;

trait Wizard {
    fn fly(&self);
    fn swim();
}

trait Pilot {
    fn fly(&self);
    fn swim();
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
    fn swim() {
        println!("*almost a fish*");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("whoosh!!");
    }
    fn swim() {
        println!("*swimming is magic*");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("Your captain is sneaking up");
    }
    fn swim() {
        println!("I dont swim I just drive");
    }
}

fn main() {
    let person = Human;

    person.fly();
    Wizard::fly(&person);
    Pilot::fly(&person);

    Human::swim();
    <Human as Wizard>::swim();
    <Human as Pilot>::swim();
}
