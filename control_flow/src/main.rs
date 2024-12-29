fn main() {
    let number = 3;

    if number < 1 {
        println!("number is less than 1")
    } else if number >= 1 && number < 3 {
        println!("number is greater than or equal to 1 and less than 3")
    } else {
        println!("number is greater than or equal to 3")
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The number is: {element}");
    }
}
