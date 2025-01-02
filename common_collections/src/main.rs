fn main() {
    vectors();
}

fn vectors() {
    let v: Vec<i32> = vec![100, 32, 57];

    for i in &v {
        println!("{i}");
    }
}
