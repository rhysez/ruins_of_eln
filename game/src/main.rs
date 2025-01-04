use lib::char::{self, pc::PlayerChar};
use colored::Colorize;

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

    println!("--------------------------------------------------------");
    println!("{}", "Ruins of Eln v0.1.0".bright_green().on_blue());
    println!("--------------------------------------------------------");

    println!("Player constructed as {}, {}, level {}", new_pc.name, new_pc.class, new_pc.level);
}
