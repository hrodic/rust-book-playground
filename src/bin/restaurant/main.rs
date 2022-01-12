// https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
// this binary uses a library defined in the same package rust-book
use rust_book::{front_of_house, back_of_house}; // from lib.rs

fn main() {
    eat_at_restaurant();
    use_keyword();
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    dbg!(order1);
    dbg!(order2);
}

pub fn use_keyword() {
    use self::front_of_house::hosting;
    use self::front_of_house::hosting as hosting_alias;
    hosting::add_to_waitlist();
    hosting_alias::add_to_waitlist();
}