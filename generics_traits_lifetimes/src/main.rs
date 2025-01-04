fn main() {
    let string1 = String::from("abcd");
    let result;

    /*
     * This does not work.
     * The reason is because the longest function using lifetime generics.
     * So when we call this function, the lifetime that is chosen for 'a that is in the signature for longest is the shorter
     * of string1 and string2. That means the lifetime of the returned reference will only live for as long as string2,
     * which is the inner scope.
     */
    {
        let string2 = String::from("xyz");

        result = longest(string1.as_str(), string2.as_str());
    };

    println!("The longest string is {result}");
}

/**
 * The 'a is a lifetime generic. Basically 'a represents a lifetime (which is how long a variable can last)
 *
 * By using 'a to define x and y, we are saying that the lifetimes of x and y are at least as long as 'a.
 *
 * In practice this means that the lifetime of the returned reference is the same as the smaller of the lifetimes
 * of the values referenced in the function arguments.
 */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}
