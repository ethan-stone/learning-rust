fn main() {
    println!("-------- Mutable Variables --------");
    mutable_variable();
    println!("-------- Shadowing --------");
    shadowing();
    println!("-------- Data Types --------");
    data_types()
}

fn mutable_variable() {
    let mut x = 5; // mut keyword makes variable mutable
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x + 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    // shadowing allows us to reuse the same varaible name with different types
    // you can't do this with the mut keyword
    let spaces = "spaces";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces)
}

fn data_types() {
    // without the type annotation there is a compiler error
    //
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess)
}
