fn main() {
    vectors();
}

fn vectors() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let first = v.get(0);

    match first {
        Some(value) => {
            println!("The first element is {value}");
        }
        None => {
            println!("There is no first element")
        }
    }
}
