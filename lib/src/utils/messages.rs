use crate::char::pc::PlayerChar;
use colored::Colorize;

pub fn print_title() {
    println!("--------------------------------------------------------");
    println!(
        "Welcome to {}",
        "Ruins of Eln v0.1.0".bright_green().on_blue()
    );
    println!("--------------------------------------------------------");
}

pub fn print_game_desc() {
    const GAME_DESC: &str = "Ruins of Eln is a roguelike RPG set in a science-fantasy, underground world. Create your character and embark into the darkness of Eln, an old labyrinthine civilisation lost to space and time.";
    println!("{}", GAME_DESC)
}

pub fn announce_level_up(pc: &PlayerChar) {
    println!("---You have reached level {}---", pc.get_level());
}
