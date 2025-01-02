fn main() {
    let dice_roll = 8;

    match dice_roll {
        3 => {
            println!("3");
        }
        other => {
            println!("other: {other}")
        }
    }
}
