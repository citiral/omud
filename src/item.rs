use entity::{self, Describable, Id};

pub struct Item {
    id: usize,
    type_id: usize,
    name: String,
}

impl Item {
    pub fn new(type_id: usize, name: String) -> Item {
        Item {
            id: entity::generate_id(),
            type_id: type_id,
            name: name
        }
    }
}

impl Id for Item {
    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id
    }
}

impl Describable for Item {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_description(&self) -> String {
        "a ".to_string() + &self.name
    }
}
