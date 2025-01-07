pub struct PlayerChar {
    pub name: String,
    pub class: String,
    pub level: u8,
    pub hit_points: u16,
    pub fatigue: u16,
    pub mood: String,
    pub mental_state: String,
}
// Methods.
impl PlayerChar {
    pub fn get_name(&self) -> &String {
        return &self.name
    }

    pub fn get_class(&self) -> &String {
        return &self.class
    }

    pub fn get_level(&self) -> &u8 {
        return &self.level
    }
}
// Associated functions.
impl PlayerChar {
    pub fn new_char(name: String, class: String) -> PlayerChar {
        PlayerChar {
            name: name,
            class: class,
            level: 1,
            hit_points: 10,
            fatigue: 10,
            mood: String::from("Content"),
            mental_state: String::from("Healthy")
        }
    }
}