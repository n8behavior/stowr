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
    Location {
        id,
        name: name.into(),
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub id: u64,
    pub name: String,
}

#[test]
fn creates_location() {
    let loc = new_location(1, "warehouse");
    assert_eq!(loc.id, 1);
    assert_eq!(loc.name, "warehouse");
}
