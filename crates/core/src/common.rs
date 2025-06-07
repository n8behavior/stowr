use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{fmt, marker::PhantomData, str::FromStr};
use uuid::Uuid;

/// A generic CRUD interface.
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

/// The one-and-only underlying ID type, always a v4 UUID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RepositoryId<T> {
    value: Uuid,
    #[serde(skip)]
    _marker: PhantomData<T>,
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
