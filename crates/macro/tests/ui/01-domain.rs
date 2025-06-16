// A smoke test for #[domain]
extern crate stowr_macro;
use serde::{Deserialize, Serialize};
use stowr_macro::domain;

// Stub the Repository trait so expansion resolves
pub trait Repository {
    type Entity;
    type Id;
}

// Stub the RepositoryId so expansion resolves
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RepositoryId<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> Default for RepositoryId<T> {
    fn default() -> Self {
        RepositoryId {
            _marker: std::marker::PhantomData,
        }
    }
}

#[domain]
pub struct Bar {
    a: String,
    b: i32,
}

fn main() {
    let id: BarId = Default::default();
    let bar = Bar::new(id.clone(), "hello", 42);
    assert_eq!(bar.a, "hello");
    assert_eq!(bar.b, 42);
}
