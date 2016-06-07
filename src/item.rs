use entity::{self, Describable, Id};
use datastore::Unique;
use WORLD_DATA;

pub struct ItemDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub stackable: bool,
}

pub struct ItemSpawn {
    pub id: String,
    pub count: u32,
    pub max: u32,
    pub respawn_rate: u32,
}

pub struct Item {
    id: usize,
    count: u32,
    item_id: String,
}

impl Unique for ItemDefinition {
    type Identifier = String;

    fn get_unique_identifier(&self) -> Self::Identifier {
        self.id.clone()
    }
}

impl ItemDefinition {
    pub fn new(id: String, name: String, description: String, stackable: bool) -> ItemDefinition {
        ItemDefinition {
            id: id,
            name: name,
            description: description,
            stackable: stackable,
        }
    }

    pub fn spawn(&self, count: u32) -> Item {
        Item {
            id: entity::generate_id(),
            count: count,
            item_id: self.id.clone(),
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn is_stackable(&self) -> bool {
        self.stackable
    }
}

impl ItemSpawn {
    pub fn new(id: String, count: u32, max: u32, respawn_rate: u32) -> ItemSpawn {
        ItemSpawn {
            id: id,
            count: count,
            max: max,
            respawn_rate: respawn_rate,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }

    pub fn get_max(&self) -> u32 {
        self.max
    }

    pub fn get_respawn_rate(&self) -> u32 {
        self.respawn_rate
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
        let def = self.get_item_definition();
        let name = def.map(|def| def.get_name()).unwrap_or("INVALID_ITEM").to_string();

        if self.count > 1 {
            name + " (" + &self.count.to_string() + ")"
        } else {
            name
        }
    }

    fn get_description(&self) -> String {
        let def = self.get_item_definition();
        def.map(|def| def.get_description()).unwrap_or("INVALID_ITEM").to_string()
    }
}

impl Item {
    pub fn get_count(&self) -> u32 {
        self.count
    }

    pub fn set_count(&mut self, count: u32) {
        self.count = count
    }

    pub fn get_item_id(&self) -> &str {
        &self.item_id
    }

    pub fn get_item_definition(&self) -> Option<&ItemDefinition> {
        // we unwrap here because having no item definition would mean the world data is malformed anyway
        WORLD_DATA.items.get(&self.item_id)
    }
}
