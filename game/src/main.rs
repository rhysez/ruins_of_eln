use lib::char::pc::PlayerChar;
use lib::char::class::Class;
use lib::utils::messages::*;
use lib::utils::startup::*;
use lib::systems::char::init_char;

fn main() {
    let classes: [Class; 3] = generate_classes();

    // Initialising a new char
    let new_pc: PlayerChar = PlayerChar::new(String::from("Rhys"));
    let char: (PlayerChar, &Class) = init_char(new_pc, classes[2].get());

    print_title();

    println!("{}", char.0.get_name());
    println!("{}", char.1.get_name());
}
