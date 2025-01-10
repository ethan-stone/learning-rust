pub trait Vehicle {
    fn drive(&self);
}

pub struct Truck {
    next_truck: Option<Box<Truck>>,
}

impl Vehicle for Truck {
    fn drive(&self) {
        println!("Truck is driving")
    }
}

/*
This will not compile.
The reason is because there could be many implementations of Vehicle,
and each implementation may require a different amount of memory. So,
at compilation time, the compiler does not know how much memory to allocate.
The solution here is to wrap it in a Box, which stores the data on the heap

Futhermore, Box allows for recursive types, like we see for next_truck.
*/
// fn box_example_bad() {
//     let t: dyn Vehicle;
//     t = Truck;
//     t.drive();
// }
pub fn box_example() {
    let t: Box<dyn Vehicle>;
    t = Box::new(Truck { next_truck: None });
    t.drive();
}
