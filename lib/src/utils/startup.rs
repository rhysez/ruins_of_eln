use crate::char::class::Class;

pub fn generate_classes() -> [Class; 3] {
    let bruiser = Class::new(String::from("Bruiser"));
    let skulker: Class = Class::new(String::from("Skulker"));
    let apothecary: Class = Class::new(String::from("Apothecary"));

    [bruiser, skulker, apothecary]
}
