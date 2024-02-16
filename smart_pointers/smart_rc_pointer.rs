use std::rc::Rc;

#[allow(dead_code, unused)]
#[derive(Debug)]
struct Truck {
    capacity: i32,
}

fn main() {
    let truck = (Rc::new(Truck{capacity: 1}), 
                 Rc::new(Truck{capacity: 2}), 
                 Rc::new(Truck{capacity: 3}));

    let facility_1 = vec![Rc::clone(&truck.0), Rc::clone(&truck.1)];
    let facility_2 = vec![Rc::clone(&truck.1), Rc::clone(&truck.2)];

    println!("Facility one {:?}", facility_1);
    println!("Facility two {:?}", facility_2);
    println!("Ref count {:?}", Rc::strong_count(&truck.1));

    std::mem::drop(truck);

    println!("Facility one {:?}", facility_1);
    println!("Facility two {:?}", facility_2);
}