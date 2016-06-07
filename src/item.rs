use entity::{self, Describable, Id};

/*pub struct Thing {
    id: usize,
    item_type: String,
    name: String,
    count: u64,
}

impl Describable for Thing {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_description(&self) -> String {
        "a ".to_string() + &self.name
    }
}

impl Stackable for Thing {
    fn get_stack_count(&self) -> u64 {
        self.count
    }
}

impl Id for Thing {
    fn get_id(&self) -> usize {
        self.id
    }
    fn set_id(&mut self, id: usize) {
        self.id = id
    }
}

impl Item for Thing {
    fn get_item_type(&self) -> &str {
        &self.item_type
    }
}

impl Thing {
    pub fn new(type_id: String, name: String, count: u64) -> Thing {
        Thing {
            id: entity::generate_id(),
            item_type: type_id,
            name: name,
            count: count,
        }
    }
}
*/

pub struct ItemDefinition {
    pub id: String,
    pub name: String,
    pub stackable: bool,
}

impl ItemDefinition {
    pub fn spawn(&self, count: u32) -> Item {
        Item {
            id: entity::generate_id(),
            count: count,
            itemId: self.id.clone(),
        }
    }
}

pub struct ItemSpawnDefinition {
    pub id: String,
    pub count: u32,
    pub max: u32,
    pub respawn: u32,
}

pub struct Item {
    pub id: usize,
    pub count: u32,
    pub itemId: String,
}