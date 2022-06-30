mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // In contrast, if we make an enum public, all of its variants are then public.
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

// Note that use only creates the shortcut for the particular scope in which the use occurs.
// Bringing the function’s parent module into scope with use means we have to specify the
// parent module when calling the function. Specifying the parent module when calling the function makes
// it clear that the function isn’t locally defined while still minimizing repetition of the full path.
/*
Re-exporting
To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope,
we can combine pub and use. This technique is called re-exporting because we’re bringing an item into scope
but also making that item available for others to bring into their scope.

Re-exporting is useful when the internal structure of your code is different from how programmers calling
your code would think about the domain. For example, in this restaurant metaphor, the people running the
restaurant think about “front of house” and “back of house.” But customers visiting a restaurant probably
won’t think about the parts of the restaurant in those terms. 
*/
pub use crate::front_of_house::hosting;

/*
use std::cmp::Ordering;
use std::io;
*/

/*
use std::{cmp::Ordering, io};
*/

/*
use std::io;
use std::io::Write;
*/

/*
use std::io::{self, Write};
*/

// The glob operator is often used when testing to bring everything under test into the tests module
// The glob operator is also sometimes used as part of the prelude pattern
/*
use std::collections::*;
*/

pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

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