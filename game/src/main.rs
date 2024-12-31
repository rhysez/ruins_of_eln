use lib::char::{self, pc::PlayerChar};

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

    println!("Ruins of Eln alpha: Player constructed as {}", new_pc.name);
}
