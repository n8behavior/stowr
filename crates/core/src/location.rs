use serde::{Deserialize, Serialize};

use crate::common::{Repository, RepositoryId};

/// Persistence abstraction for [`Location`] data.
///
/// this trait defines the operations required by
/// the domain logic without committing to any specific database layer.
pub trait LocationRepository: Repository<Entity = Location, Id = LocationId> {}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LocationTag {}

pub type LocationId = RepositoryId<LocationTag>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: LocationId,
    pub name: String,
}

pub fn new(id: LocationId, name: impl Into<String>) -> Location {
    Location {
        id,
        name: name.into(),
    }
}

#[test]
fn creates_location() {
    let id = LocationId::new();
    let name = "warehouse";
    let loc = new(id.clone(), name);
    assert_eq!(loc.id, id);
    assert_eq!(loc.name, "warehouse");
}
