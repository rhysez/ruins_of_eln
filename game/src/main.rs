use lib::char::{self, pc::PlayerChar};
use lib::utils::messages::print_title;
use std::io;

fn main() {
    let new_pc: PlayerChar = PlayerChar {
        name: String::from("Rhys"),
        class: String::from("Skulker"),
        level: 1,
        hit_points: 10,
        fatigue: 10,
        mood: String::from("Content"),
        mental_state: String::from("Healthy")
    };

    print_title();
}
