// A smoke test for #[domain_impl] + #[command]
extern crate stowr_macro;
use serde::{Deserialize, Serialize};
use stowr_macro::{command, domain, domain_impl};

// Stub the Repository trait so expansion resolves
pub trait Repository {
    type Entity;
    type Id;
}

// Stub common traits and types
pub trait Aggregate {
    type Command;
    type Event;
    type Error;
    fn handle_command(&self, cmd: Self::Command) -> Result<Vec<Self::Event>, Self::Error>;
    fn apply_event(&mut self, evt: &Self::Event);
}

#[derive(Debug)]
pub enum AggregateError {}

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
pub struct Baz {
    val: u8,
}

#[domain_impl]
impl Baz {
    #[command]
    pub fn create(_id: BazId, _val: u8) {
        // initial state
    }

    #[command]
    pub fn increment(&mut self, by: u8) {
        self.val = self.val.wrapping_add(by);
    }
}

fn main() {
    // Ensure enums and methods exist
    let _cmd: BazCommand = BazCommand::Increment { by: 5 };
}
