pub struct PlayerChar {
    name: String,
    class: String,
    level: u8,
    hit_points: u16,
    fatigue: u16,
    mood: String,
    mental_state: String,
    exp: u16,
}

const MAX_LEVEL: u8 = 20;

// Methods.
impl PlayerChar {
    // Getters.
    pub fn get_name(&self) -> &String {
        return &self.name
    }

    pub fn get_class(&self) -> &String {
        return &self.class
    }

    pub fn get_level(&self) -> &u8 {
        return &self.level
    }

    pub fn get_hit_points(&self) -> &u16 {
        return &self.hit_points
    }

    pub fn get_fatigue(&self) -> &u16 {
        return &self.fatigue
    }

    pub fn get_mood(&self) -> &String {
        return &self.mood
    }

    pub fn get_mental_state(&self) -> &String {
        return &self.mental_state
    }

    pub fn get_exp(&self) -> &u16 {
        return &self.exp
    }

    // Setters
    pub fn level_up(&mut self) -> u8 {
        if self.level == MAX_LEVEL {
            return self.level
        }

        self.level += 1;

        return self.level
    }
}
// Associated functions.
impl PlayerChar {
    pub fn new(name: String, class: String) -> PlayerChar {
        PlayerChar {
            name: name,
            class: class,
            level: 1,
            hit_points: 10,
            fatigue: 10,
            mood: String::from("Content"),
            mental_state: String::from("Healthy"),
            exp: 0
        }
    }
}