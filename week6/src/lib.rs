/* A library that returns back random flowers */

use rand::Rng;

//create an const array of 30 flowers

const FLOWERS: [&str; 30] = [
    "African Daisy",
    "African Violet",
    "Agapanthus",
    "Ageratum",
    "Alstroemeria",
    "Amaryllis",
    "Anemone",
    "Anthurium",
    "Aster",
    "Azalea",
    "Baby's Breath",
    "Bachelor's Button",
    "Begonia",
    "Bellflower",
    "Bird of Paradise",
    "Bleeding Heart",
    "Bougainvillea",
    "Bromeliad",
    "Buttercup",
    "Butterfly Bush",
    "Calendula",
    "Calla Lily",
    "Camellia",
    "Carnation",
    "Cattleya Orchid",
    "Ceriman",
    "Chrysanthemum",
    "Clematis",
    "Columbine",
    "Cyclamen",
];

//create a function that returns a random fruit
pub fn random_flower() -> String {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..FLOWERS.len());
    FLOWERS[random_index].to_string()
}
