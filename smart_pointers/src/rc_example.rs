use std::rc::Rc;

#[derive(Debug)]
struct Truck {
    capacity: i32,
}

// fn rc_example_bad() {
//     let truck_1 = Truck { capacity: 1 };

//     let truck_2 = Truck { capacity: 1 };

//     let truck_3 = Truck { capacity: 1 };

//     // Here, the facility_1 vector takes ownership of truck_2.
//     let facility_1 = vec![truck_1, truck_2];

//     // So when we use it here we get a compiler error.
//     // We could get around this using a read-only reference,
//     // to the trucks instead.
//     // The problem with this is that the main function will own
//     // the trucks the entire lifetime of the trucks until the main
//     // function ends. But what if the trucks aren't needed that whole
//     // time???
//     let facility_2 = vec![truck_2, truck_3];
// }

fn rc_example() {
    /*
    First we create Rc versions of the Trucks. Rc stands for Reference Counter.
    It maintains a count of all the references to the underlying data, and doesn't
    drop the data until all the references are gone.
     */

    let truck_1 = Rc::new(Truck { capacity: 1 });

    let truck_2 = Rc::new(Truck { capacity: 1 });

    let truck_3 = Rc::new(Truck { capacity: 1 });

    /*
    THen we can use Rc::clone to make a new Rc that points to the same data.
     */
    let facility_1 = vec![Rc::clone(&truck_1), Rc::clone(&truck_2)];

    let facility_2 = vec![Rc::clone(&truck_2), Rc::clone(&truck_3)];

    println!("Facility 1 {:?}", facility_1);
    println!("Facility 2 {:?}", facility_2);

    // We can use strong_count to see the number of references.
    println!(
        "Truck 2 strong count before drop {:?}",
        Rc::strong_count(&truck_2)
    );

    // Even if drop facility_2, facility_1 will still have a valid reference to truck_2
    std::mem::drop(facility_2);

    println!(
        "Truck 2 strong count after drop {:?}",
        Rc::strong_count(&truck_2)
    );
}
