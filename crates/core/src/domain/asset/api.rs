use super::model::Asset;

pub trait AssetApi {
    fn get(&self, id: u64) -> Option<Asset>;
}
