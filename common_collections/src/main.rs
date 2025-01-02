fn main() {
    vectors();
}

fn vectors() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let first = &v[0];

    println!("First element is {first}");
}
