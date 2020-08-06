pub struct MiscParameters {
    age: i32,
}

impl MiscParameters {
    pub fn new() -> MiscParameters {
        MiscParameters {
            age: 30,
        }
    }
}

impl Default for MiscParameters {
    fn default() -> Self {
        MiscParameters::new()
    }
}