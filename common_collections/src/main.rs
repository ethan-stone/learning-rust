fn main() {
    vectors();
}

fn vectors() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // This is an immutable references
    let first = &v[0];

    // Here we mutate the array, thus borrowing a mutable references
    v.push(6);

    // so we can't use "first" here by rules of borrowing
    println!("The first elemetn is: {first}");
}
