use rand::seq::{IndexedRandom};
use rand::rng;

pub fn motd() -> String {
    let potentials = vec![
        "First we Ball, then we Atro. We Balatro!",
        "Someone call Bean!",
        "Surely you'll pull a baron this time.",
        "Also try the SealSeal mod!",
        "You are valid!",
        "Don't buff ceremonial dagger.",
        "Praise Zech!",
        "Only cool people can see this message :3",
        ":3",
        "Also try Cloverpit!",
        "You can add your own MOTD by opening a report at https://github.com/colonthreeing/balatro-tui!",
        "Welcome to the balatro tui!"
    ];

    let random = potentials.choose(&mut rng());
    random.unwrap().to_string()
}