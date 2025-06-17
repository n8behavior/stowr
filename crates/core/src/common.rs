use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{fmt, marker::PhantomData, str::FromStr};
use uuid::Uuid;

// ANCHOR: repository
/// Base trait for all domain repositories.
/// - [Entity] is the domain type (e.g. [Asset], [Location]).  
/// - [Id] is the type of its primary‐key (e.g. [AssetId], [LocationId]).
#[async_trait]
pub trait Repository {
    /// The domain object (e.g. [Asset], [Location]).
    type Entity: Send + Sync;
    /// The ID type (e.g. [AssetId]).
    type Id: Send + Sync;

    /// Create a new [Entity] and return it (with its new `Id` set).
    async fn create(&self, entity: Self::Entity) -> Result<Self::Entity>;

    /// Fetch an [Entity] by its ID (or return `None` if not found).
    async fn fetch(&self, id: Self::Id) -> Result<Option<Self::Entity>>;
}
// ANCHOR_END: repository

// ANCHOR: Repository_id
/// The one-and-only underlying ID type, always a v4 UUID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RepositoryId<T> {
    value: Uuid,
    #[serde(skip)]
    _marker: PhantomData<T>,
}
// ANCHOR_END: Repository_id

impl<T> Default for RepositoryId<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> RepositoryId<T> {
    /// Create a brand-new v4 UUID
    pub fn new() -> Self {
        Self {
            value: Uuid::new_v4(),
            _marker: PhantomData,
        }
    }
}

impl<T> fmt::Display for RepositoryId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> FromStr for RepositoryId<T> {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let u = Uuid::from_str(s)?;
        Ok(RepositoryId {
            value: u,
            _marker: PhantomData,
        })
    }
}

/// Allows `let id: Id<Foo> = uuid.into();`
impl<T> From<Uuid> for RepositoryId<T> {
    fn from(value: Uuid) -> Self {
        RepositoryId {
            value,
            _marker: PhantomData,
        }
    }
}

/// Allows `let uuid: Uuid = id.into();`
impl<T> From<RepositoryId<T>> for Uuid {
    fn from(id: RepositoryId<T>) -> Self {
        id.value
    }
}

/// in your `common.rs` (or wherever your macros live)
pub trait Aggregate {
    type Command;
    type Event;
    type Error;
    fn handle_command(&self, cmd: Self::Command) -> Result<Vec<Self::Event>, Self::Error>;
    fn apply_event(&mut self, evt: &Self::Event);
}

pub enum AggregateError {}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::common::Repository;
    use stowr_macro::{command, domain, domain_impl};

    // ANCHOR: foo_domain
    /// The `#[domain]` attribute will expand [Foo] to have a [RepositoryId]
    /// and `new()` impl with struct field declared as `impl Into<T>`.
    /// Additionally it will create a trait object with blanket impl and a
    /// type alias to be used by users of concrete Repository implementations.
    ///
    ///    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    ///    enum FooTag {}
    ///    type FooId = RepositoryId<FooTag>;
    ///
    ///    impl Foo {
    ///        fn new(id: FooId, name: impl Into<String>) -> Foo {
    ///            Foo {
    ///                id,
    ///                name: name.into(),
    ///            }
    ///        }
    ///    }
    ///
    ///    trait FooRepository: Repository<Entity = Foo, Id = FooId> + Send + Sync {}
    ///    impl<T> FooRepository for T
    ///      where T: Repository<Entity = Foo, Id = FooId> + Send + Sync {}
    ///    type FooRepo = Arc<dyn FooRepository>;

    #[domain]
    struct Foo {
        name: String,
    }

    #[domain_impl]
    impl Foo {
        #[command]
        fn rename(&mut self, new_name: String) {
            self.name = new_name;
        }
    }
    // ANCHOR_END: foo_domain

    #[test]
    fn rename_foo() {
        let old_name = "Old Name".to_string();
        let new_name = "New Name".to_string();
        let mut f = Foo::new(FooId::new(), old_name);
        f.rename(new_name.clone());
        assert_eq!(f.name, new_name);
    }

    // ANCHOR: vector_foo_repo
    struct VectorFooRepo {
        db: Mutex<Vec<Foo>>,
    }

    impl VectorFooRepo {
        fn new() -> FooRepo {
            Arc::new(Self {
                db: Default::default(),
            })
        }
    }

    #[async_trait]
    impl Repository for VectorFooRepo {
        type Entity = Foo;
        type Id = FooId;

        async fn create(&self, entity: Foo) -> Result<Foo> {
            let mut guard = self.db.lock().unwrap();
            guard.push(entity.clone());
            Ok(entity)
        }

        async fn fetch(&self, id: FooId) -> Result<Option<Foo>> {
            let guard = self.db.lock().unwrap();
            Ok(guard.iter().cloned().find(|d| d.id == id))
        }
    }
    // ANCHOR_END: vector_foo_repo

    #[tokio::test]
    async fn dummy_repo_can_create_and_fetch() {
        let repo = VectorFooRepo::new();
        let id = FooId::new();
        let item = Foo::new(id.clone(), "warehouse");
        let created = repo.create(item.clone()).await.unwrap();
        assert_eq!(created, item);

        let fetched = repo.fetch(id.clone()).await.unwrap();
        assert_eq!(fetched, Some(item));
    }

    #[test]
    fn dummy_new_create_dummies() {
        let id = FooId::new();
        let name = "warehouse";
        let loc = Foo::new(id.clone(), name);
        assert_eq!(loc.id, id);
        assert_eq!(loc.name, "warehouse");
    }

    #[test]
    fn dummy_id_new_produces_unique_ids() {
        let a = FooId::new();
        let b = FooId::new();
        assert_ne!(a, b, "sequential new() calls should yield different IDs");
    }

    #[test]
    fn roundtrip_uuid_via_into_and_from() {
        let original = FooId::new();
        let uuid: Uuid = original.clone().into();
        let reconstructed: FooId = uuid.into();
        assert_eq!(original, reconstructed);
    }

    #[test]
    fn parse_from_string_roundtrip() {
        let original = FooId::new();
        let s = original.to_string();
        let parsed = FooId::from_str(&s).expect("valid uuid string");
        assert_eq!(original, parsed);
    }
}
