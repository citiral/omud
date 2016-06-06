use std::fs::{self, DirEntry, File};
use std::path::Path;
use std::io::Read;
use world::World;
use room::Room;
use rustc_serialize::json::{self, Json};
use std::io;
use std::fmt;
use std::error::{self, Error};
use std::convert::From;

#[derive(Debug)]
pub enum ResourceLoadError {
    Io(io::Error),
    JsonParser(json::ParserError),

    InvalidValue{key: String, expected: String, got: String},
}

impl fmt::Display for ResourceLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResourceLoadError::Io(ref err) => err.fmt(f),
            ResourceLoadError::JsonParser(ref err) => err.fmt(f),
            ResourceLoadError::InvalidValue{ref key, ref expected, ref got} => {
                write!(f, "Invalid resource, for key {} expected {} but got {}.", key, expected, got)
            }
        }
    }
}

impl Error for ResourceLoadError {
    fn description(&self) -> &str {
        match *self {
            ResourceLoadError::Io(ref err) => err.description(),
            ResourceLoadError::JsonParser(ref err) => err.description(),
            ResourceLoadError::InvalidValue{ref key, ref expected, ref got} => {
                "Invalid type found for a resource key."
            }
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ResourceLoadError::Io(ref err) => err.cause(),
            ResourceLoadError::JsonParser(ref err) => err.cause(),
            ResourceLoadError::InvalidValue{..} => None,
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

pub fn parse_room_from_resource<P: AsRef<Path>>(path: P) -> Result<Room, ResourceLoadError> {
    // read the file
    let mut data = String::new();
    let _ = try!(File::open(path).map(|mut file| file.read_to_string(&mut data)));
    let json: Json = try!(Json::from_str(&data));

    // parse the data into the room
    let mut room = Room::new(try!(get_string(&json, "id")).to_string());
    room.name = try!(get_string(&json, "name")).to_string();
    room.description = try!(get_string(&json, "description")).to_string();

    // parse the exits
    for exit in try!(get_vec(&json, "connections")) {
        room.add_exit(try!(get_string(&exit, "direction")).to_string(), try!(get_string(&exit, "room")).to_string())
    }

    Ok(room)

}

pub fn parse_world_from_resources() -> Result<World, ResourceLoadError> {
    let mut world = World::new();

    // first parse all events

    // then parse all rooms
    if let Ok(dir) = fs::read_dir("./res/rooms") {
        for path in dir {
            let path = try!(path);
            world.add_room(try!(parse_room_from_resource(path.path())));
        }
    }

    Ok(world)
}
