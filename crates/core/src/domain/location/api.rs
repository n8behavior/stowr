use super::model::Location;

pub trait LocationApi {
    fn get(&self, id: u64) -> Option<Location>;
}
