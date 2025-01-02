mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        season_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            return Breakfast {
                toast: String::from(toast),
                season_fruit: String::from("peaches"),
            };
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

use back_of_house::Breakfast;
use front_of_house::hosting;

pub fn eat_at_restaurant() {
    let mut meal = Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please.", meal.toast);

    // this next line won't compile because season_fruit is private
    // meal.seasonal_fruit = String::from("blueberries");

    // all values of an enum are public if the enum is public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
}
