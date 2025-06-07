/// Abstract persistence operations for [`Asset`].
///
/// This trait represents the "port" for saving and retrieving assets without
/// exposing any database implementation details to the rest of the domain code.
pub trait AssetRepository {
    /// Fetch an asset by id.
    fn fetch(&self, id: u64) -> Option<Asset>;

    /// Persist a new asset or update an existing one.
    fn store(&self, asset: &Asset);
}

pub fn new_asset(id: u64, name: impl Into<String>) -> Asset {
    Asset {
        id,
        name: name.into(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asset {
    pub id: u64,
    pub name: String,
}

#[test]
fn creates_asset() {
    let asset = new_asset(1, "test");
    assert_eq!(asset.id, 1);
    assert_eq!(asset.name, "test");
}
