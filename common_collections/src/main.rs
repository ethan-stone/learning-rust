fn main() {
    vectors();
    strings();
    hash_maps();
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

use std::collections::HashMap;

fn hash_maps() {
    let text = String::from("Hello, world. My beautiful world.");

    let mut char_map: HashMap<char, u32> = HashMap::new();

    for char in text.chars() {
        let count = char_map.entry(char).or_insert(0);
        *count += 1;
    }

    println!("{char_map:?}");
}
