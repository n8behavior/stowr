use super::model::Location;

/// Persistence abstraction for [`Location`] data.
///
/// Similar to `AssetRepository`, this trait defines the operations required by
/// the domain logic without committing to any specific database layer.
pub trait LocationRepository {
    /// Retrieve a location by id.
    fn get(&self, id: u64) -> Option<Location>;

    /// Save a new or updated location.
    fn store(&self, location: &Location);
}

pub fn new_location(id: u64, name: impl Into<String>) -> Location {
    Location { id, name: name.into() }
}
