#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
    class: u8
}

#[derive(Debug)]
struct Teacher {
    name: String,
    subject: String
}

fn main() {
    let students: [Student; 4] = [
        Student {
            name: String::from("Posto"),
            age: 17,
            class: 12,
        },
        Student {
            name: String::from("Bisu"),
            age: 17,
            class: 12,
        },
        Student {
            name: String::from("Sabu"),
            age: 17,
            class: 12,
        },
        Student {
            name: String::from("Bala"),
            age: 17,
            class: 12,
        },
    ];
    let teachers: [Teacher; 4] = [
        Teacher {
            name: String::from("Arup"),
            subject: String::from("Computer"),
        },
        Teacher {
            name: String::from("Sukla"),
            subject: String::from("Maths")
        },
        Teacher {
            name: String::from("Prasenjit"),
            subject: String::from("English")
        },
        Teacher {
            name: String::from("Swarnali"),
            subject: String::from("Physics"),
        }
        Teacher {
            name: String::from("Bharati"),
            subject: String::from("Biology"),
        }
    ];
    println!("{:?} \n {:?}", students, teachers);    
}
