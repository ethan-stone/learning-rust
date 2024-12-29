fn main() {
    let s = String::from("hello");

    let length = calculate_length(&s);

    println!("The length of {s} is {length}");
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}
