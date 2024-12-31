// Private fields should be modified via methods or associated functions.
// Public fields are accessible but cannot change, such as name or class.
pub struct PlayerChar {
    pub name: String,
    pub class: String,
    pub level: u8,
    pub hit_points: u16,
    pub fatigue: u16,
    pub mood: String,
    pub mental_state: String,
}