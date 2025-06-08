use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::Repository;

/// Abstract persistence operations for [`Asset`].
///
/// This trait represents the "port" for saving and retrieving assets without
/// exposing any database implementation details to the rest of the domain code.
pub trait AssetRepository: Repository<Entity = Asset, Id = AssetId> {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: AssetId,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetId(pub Uuid);

impl Asset {
    fn new(id: AssetId, name: impl Into<String>) -> Asset {
        Asset {
            id,
            name: name.into(),
        }
    }
}

#[test]
fn creates_asset() {
    let id = AssetId(Uuid::new_v4());
    let name = "test";
    let asset = Asset::new(id.clone(), name);
    assert_eq!(asset.id, id);
    assert_eq!(asset.name, "test");
}
