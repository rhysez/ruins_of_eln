pub struct PlayerChar {
    name: String,
    class: String,
    level: u8,
    hit_points: u16,
    fatigue: u16,
    mood: String,
    mental_state: String,
    exp: u16,
    required_exp: u32,
}

const MAX_LEVEL: u8 = 20;

// Methods.
impl PlayerChar {
    // Getters.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_class(&self) -> &String {
        &self.class
    }

    pub fn get_level(&self) -> &u8 {
        &self.level
    }

    pub fn get_hit_points(&self) -> &u16 {
        &self.hit_points
    }

    pub fn get_fatigue(&self) -> &u16 {
        &self.fatigue
    }

    pub fn get_mood(&self) -> &String {
        &self.mood
    }

    pub fn get_mental_state(&self) -> &String {
        &self.mental_state
    }

    pub fn get_exp(&self) -> &u16 {
        &self.exp
    }

    pub fn get_required_exp(&self) -> &u32 {
        &self.required_exp
    }

    // Setters
    pub fn level_up(&mut self) -> u8 {
        if self.level == MAX_LEVEL {
            return self.level;
        }

        self.level += 1;

        self.level
    }
}
// Associated functions.
impl PlayerChar {
    pub fn new(name: String, class: String) -> PlayerChar {
        PlayerChar {
            name,
            class,
            level: 1,
            hit_points: 10,
            fatigue: 10,
            mood: String::from("Content"),
            mental_state: String::from("Healthy"),
            exp: 0,
            required_exp: 100
        }
    }
}
