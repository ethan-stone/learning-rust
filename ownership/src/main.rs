fn main() {
    // The string made in gives_ownership is moved into s1
    let s1 = gives_ownership();

    // s2 comes into scope
    let s2 = String::from("hello");

    // s2 is passed into takes_and_gives_back, so within takes_and_gives_gack
    // it has ownership. But then it returns the same variable
    // so ownership is returned to this scope in s3.
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    return a_string;
}
