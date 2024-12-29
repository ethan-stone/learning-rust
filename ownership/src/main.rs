fn main() {
    let mut s = String::from("hello");

    // r1 are immutable references to s

    let r1 = &s;

    let r2 = &s;

    println!("{r1} and {r2}");

    // r3 is a mutable reference to s
    let r3 = &mut s;

    println!("{r3}");

    // This program is okay because r1 or r2 aren't used after line 10
    // If we tried to add the line below, the program wouldn't compile because we are using an immutable reference and a mutable references at the same time,
    // println!("{r2}")
    // We can't do that because the immutable reference is expecting the data to not change, but we're changing it with the mutable reference just before.

    // We can create new immutable reference after the mutation though.

    let r4 = &s;

    println!("{r4}");

    slicing();
}

fn slicing() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // this sets word to an empty string
    s.clear();

    // this means that the first word being at index five isn't true anymore
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}
