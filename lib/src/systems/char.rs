use crate::char::class::Class;
use crate::char::pc::PlayerChar;

// When initialising a new char, the PlayerChar is moved here.
// The Class is borrowed, because other entities may require use of it.
pub fn init_char(char: PlayerChar, class: &Class) -> (PlayerChar, &Class) {
    (char, class)
}