fn main() {
    another_function(5);

    print_labeled_measurement(5, 'h');

    let n = plus_two(3);

    println!("The value of n is: {n}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn plus_two(n: i32) -> i32 {
    n + 2
}
