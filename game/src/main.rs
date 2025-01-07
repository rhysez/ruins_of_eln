use lib::char::{pc::PlayerChar};
use lib::utils::messages::{*};

fn main() {
    let new_pc: PlayerChar = PlayerChar::new(String::from("Rhys"), String::from("Skulker"));

    print_title();
    
    println!("{}", new_pc.get_name());
    println!("{}", new_pc.get_class());
    println!("{}", new_pc.get_level());
}
