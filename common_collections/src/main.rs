fn main() {
    vectors();
    strings()
}

fn vectors() {
    let v: Vec<i32> = vec![100, 32, 57];

    for i in &v {
        println!("{i}");
    }
}

fn strings() {
    let mut s1 = String::from("hello, ");
    let s2 = "world!";

    s1.push_str(s2);

    println!("{s1}");
    println!("{s2}");

    for b in s1.bytes() {
        println!("{b}");
    }

    for c in s1.chars() {
        println!("{c}")
    }
}
