/* A library that returns back random fruit */

use rand::Rng;

//create an const array of 10 fruits
const FRUITS: [&str; 10] = [
    "Apple",
    "Banana",
    "Orange",
    "Pineapple",
    "Strawberry",
    "Watermelon",
    "Grape",
    "Pear",
    "Peach",
    "Mango",
];

//create a function that returns a random fruit
pub fn random_fruit() -> String {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..FRUITS.len());
    FRUITS[random_index].to_string()
}
