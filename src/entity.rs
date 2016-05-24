pub struct Entity {
    pub location: String,
    pub randomstat: i32
}

impl Entity {
    pub fn new(location: String) -> Entity {
        Entity {
            location: location,
            randomstat: 20
        }
    }
}
