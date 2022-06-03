mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    // All properties of pub struct are private by default
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        // Need a constructor otherwise cant set values of private properties
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // All cases of pub enum are pub by default
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn cook_order() {}

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
}

use crate::front_of_house::hosting;

// front of house isn't public but can be accessed here because they are siblings
// hosting and add_to_waitlist need to be public to be accessed here
pub fn eat_at_resaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // From use above
    hosting::add_to_waitlist();

    pub fn eat_at_restaurant() {
        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");

        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
}
