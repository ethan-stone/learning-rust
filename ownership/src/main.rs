fn main() {
    let mut s = String::from("hello");

    let length = calculate_length(&s);

    println!("The length of {s} is {length}");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

    change(&mut s);
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn change(s: &mut String) {
    s.push_str(", world!");
}
