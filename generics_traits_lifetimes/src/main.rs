// 'a is a generic lifetime
struct ImportantExcerpt<'a> {
    part: &'a str, // the part field will have the lifetime 'a
}

// Since one of the lifetimes &self, the life time of the return reference is the same as the
// the struct itself.
// In other words, the referenced returned by announce_and_return_part can only live as long
// as the struct is alive.
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        return self.part;
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    // this particular usage of ImportantExcerpt means that
    // i can't outlive first_sentence.
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
