use std::fs::{self, DirEntry, File};
use std::path::{Path};
use std::io::Read;
use world::{World, WorldData};
use room::Room;
use item::*;
use rustc_serialize::json::{self, Json};
use std::io;
use std::fmt;
use std::error::{self, Error};
use std::convert::From;
use WORLD_DATA;

#[derive(Debug)]
pub enum ResourceLoadError {
    Io(io::Error),
    JsonParser(json::ParserError),
    InvalidValue{key: String, expected: String, got: String},
    InvalidItem{item: String},
}

impl fmt::Display for ResourceLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResourceLoadError::Io(ref err) => err.fmt(f),
            ResourceLoadError::JsonParser(ref err) => err.fmt(f),
            ResourceLoadError::InvalidValue{ref key, ref expected, ref got} => {
                write!(f, "Invalid resource, for key {} expected {} but got {}.", key, expected, got)
            },
            ResourceLoadError::InvalidItem{ref item} => {
                write!(f, "Invalid Item, there is no item defined by the name of {}", item)
            }
        }
    }
}

impl Error for ResourceLoadError {
    fn description(&self) -> &str {
        match *self {
            ResourceLoadError::Io(ref err) => err.description(),
            ResourceLoadError::JsonParser(ref err) => err.description(),
            ResourceLoadError::InvalidValue{..} => {
                "Invalid type found for a resource key."
            },
            ResourceLoadError::InvalidItem{..} => {
                "No item found for the given name."
            }
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ResourceLoadError::Io(ref err) => err.cause(),
            ResourceLoadError::JsonParser(ref err) => err.cause(),
            _ => None,
        }
    }
}

impl From<io::Error> for ResourceLoadError {
    fn from(err: io::Error) -> ResourceLoadError {
        ResourceLoadError::Io(err)
    }
}

impl From<json::ParserError> for ResourceLoadError {
    fn from(err: json::ParserError) -> ResourceLoadError {
        ResourceLoadError::JsonParser(err)
    }
}

fn to_type(json: &Json) -> &str {
    match json {
        &Json::I64(_) => "i64",
        &Json::U64(_) => "u64",
        &Json::F64(_) => "f64",
        &Json::String(_) => "String",
        &Json::Boolean(_) => "bool",
        &Json::Array(_) => "Array",
        &Json::Object(_) => "Object",
        &Json::Null => "Null",
    }
}

fn get_string<'a>(json: &'a Json, key: &str) -> Result<&'a str, ResourceLoadError> {
    if let Some(value) = json.find(key) {
        value.as_string().ok_or(ResourceLoadError::InvalidValue {
            key: key.to_string(),
            expected: "String".to_string(),
            got: to_type(value).to_string(),
        })
    } else {
        Err(ResourceLoadError::InvalidValue {
            key: key.to_string(),
            expected: "String".to_string(),
            got: "Nothing".to_string(),
        })
    }
}

fn get_bool(json: &Json, key: &str) -> Result<bool, ResourceLoadError> {
    if let Some(value) = json.find(key) {
        value.as_boolean().ok_or(ResourceLoadError::InvalidValue {
            key: key.to_string(),
            expected: "Bool".to_string(),
            got: to_type(value).to_string(),
        })
    } else {
        Err(ResourceLoadError::InvalidValue {
            key: key.to_string(),
            expected: "Bool".to_string(),
            got: "Nothing".to_string(),
        })
    }
}

fn get_u64(json: &Json, key: &str) -> Result<u64, ResourceLoadError> {
    if let Some(value) = json.find(key) {
        value.as_u64().ok_or(ResourceLoadError::InvalidValue {
            key: key.to_string(),
            expected: "U64".to_string(),
            got: to_type(value).to_string(),
        })
    } else {
        Err(ResourceLoadError::InvalidValue {
            key: key.to_string(),
            expected: "U64".to_string(),
            got: "Nothing".to_string(),
        })
    }
}

fn get_vec<'a>(json: &'a Json, key: &str) -> Result<&'a Vec<Json>, ResourceLoadError> {
    if let Some(value) = json.find(key) {
        value.as_array().ok_or(ResourceLoadError::InvalidValue {
            key: key.to_string(),
            expected: "Array".to_string(),
            got: to_type(value).to_string(),
        })
    } else {
        Err(ResourceLoadError::InvalidValue {
            key: key.to_string(),
            expected: "Array".to_string(),
            got: "Nothing".to_string(),
        })
    }
}

fn parse_json_from_resource<P: AsRef<Path>>(path: P) -> Result<Json, ResourceLoadError> {
    let mut data = String::new();
    let _ = try!(File::open(path).map(|mut file| file.read_to_string(&mut data)));
    Ok(try!(Json::from_str(&data)))
}

pub fn parse_room_from_resource(json: Json) -> Result<Room, ResourceLoadError> {
    // parse the data into the room
    let mut room = Room::new(try!(get_string(&json, "id")).to_string());
    room.name = try!(get_string(&json, "name")).to_string();
    room.description = try!(get_string(&json, "description")).to_string();

    // parse the exits
    for exit in try!(get_vec(&json, "connections")) {
        room.add_exit(try!(get_string(&exit, "direction")).to_string(), try!(get_string(&exit, "room")).to_string())
    }

    for spawn in try!(get_vec(&json, "items")) {
        // first check if the item does really exist
        let id = try!(get_string(&spawn, "id")).to_string();

        if !WORLD_DATA.items.has(&id) {
            return Err(ResourceLoadError::InvalidItem{item: id});
        }

        room.add_item_spawn(ItemSpawn::new(
            id,
            try!(get_u64(&spawn, "count")) as u32,
            try!(get_u64(&spawn, "max")) as u32,
            try!(get_u64(&spawn, "respawn")) as u32
        ));
    }

    Ok(room)
}

pub fn parse_item_definition_from_resource(json: Json) -> Result<ItemDefinition, ResourceLoadError> {
    // parse the data into the room
    let item = ItemDefinition::new(
        try!(get_string(&json, "id")).to_string(),
        try!(get_string(&json, "name")).to_string(),
        try!(get_string(&json, "description")).to_string(),
        try!(get_bool(&json, "stackable")));

    Ok(item)
}

fn parse_resources_folder<Pa: AsRef<Path>, R, F, E, T>(path: Pa, r: R, mut f: F, mut e: E)
    where R: Fn(Json) -> Result<T, ResourceLoadError>,
          F: FnMut(T),
          E: FnMut(ResourceLoadError, DirEntry) {

    if let Ok(dir) = fs::read_dir(path) {
        for path in dir {
                let _ = path.map(|path| {
                parse_json_from_resource(path.path())
                    .and_then(|json| r(json))
                    .map(|v| f(v))
                    .map_err(|err| e(err, path))
            });
        }
    }

}

pub fn parse_world_data_from_resources() -> WorldData {
    let mut data = WorldData::new();

    // parse all item definitions
    parse_resources_folder("./res/items", parse_item_definition_from_resource,
        |item| data.items.add(item),
        |err, path| println!("Failuring parsing item definition {}: {}", path.path().to_string_lossy(), err));


    data
}


pub fn parse_world_from_resources() -> World {
    let mut world = World::new();

    // parse the world
    parse_resources_folder("./res/rooms", parse_room_from_resource,
        |room| world.add_room(room),
        |err, path| println!("Failuring parsing room {}: {}", path.path().to_string_lossy(), err));

    world
}
