#![allow(dead_code, unused_variables)]

use rand::Rng;
mod front_of_the_house;
pub use front_of_the_house::hosting::hosting;

pub fn eat_at_restaurant() {
    let r_num = rand::thread_rng().gen_range(0..10);
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    let mut meal = crate::back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    let order1 = crate::back_of_house::Appetizer::Salad;
    let order2 = crate::back_of_house::Appetizer::Soup;
}

fn serve_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Mango"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
