use super::model::Asset;

pub fn new_asset(id: u64, name: impl Into<String>) -> Asset {
    Asset { id, name: name.into() }
}
