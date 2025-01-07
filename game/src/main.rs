use lib::char::{pc::PlayerChar, class::Class};
use lib::utils::messages::{print_title, print_game_desc};

fn main() {
    let new_pc: PlayerChar = PlayerChar::new_char(String::from("Rhys"), String::from("Skulker"));
    
    print_title();
    print_game_desc();

    let new_name: &String = new_pc.get_name();

    println!("{}", new_name)
}
