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

    println!("The result is: {result}")
}
