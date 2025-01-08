pub struct Class {
    name: String,
}

impl Class {
    pub fn get(&self) -> &Class {
        &self
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

impl Class {
    pub fn new(name: String) -> Class {
        Class { name }
    }
}
