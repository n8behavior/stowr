use super::model::Location;

pub fn new_location(id: u64, name: impl Into<String>) -> Location {
    Location { id, name: name.into() }
}
