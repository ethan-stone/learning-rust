fn main() {
    let ac_power_type = PowerType::AC(32);

    if let PowerType::DC(amperage) = ac_power_type {
        println!("PowerType is DC and amperage is {amperage}");
    } else {
        println!("PowerType is not DC");
    }

    let dc_power_type = PowerType::DC(32);

    if let PowerType::AC(amperage) = dc_power_type {
        println!("PowerType is AC and amperage is {amperage}");
    } else {
        println!("PowerType is not AC");
    }
}

enum PowerType {
    AC(u32),
    DC(u32),
}
