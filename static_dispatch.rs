trait Animal {
    fn say(self) -> String;
}

struct Cow {}
struct Pig {}

impl Cow {
    // this method will get overridden
    fn say(self) -> String {
        "moo".to_string()
    }
}

impl Pig {
    // this method will get overridden
    fn say(self) -> String {
        "oink".to_string()
    }
}

impl Animal for Cow {
    fn say(self) -> String {
        "Cow moos".to_string()
    }
}

impl Animal for Pig {
    fn say(self) -> String {
        "Pig oinks".to_string()
    }
}

fn main() {
    let cow = Cow {};
    let pig = Pig {};

    println!("{}", pig.say());
    println!("{}", cow.say());
}