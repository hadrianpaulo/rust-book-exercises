mod front_of_house;

pub mod back_of_house;

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();

    let mut meal = crate::back_of_house::Breakfast::summer("Foccacia");

    meal.toast = String::from("Wheat");

    println!("A {} toast, please.", &meal.toast);

    let order1 = back_of_house::Appetizer::Salad;
}
