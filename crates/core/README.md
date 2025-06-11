# stowr-core

`stowr-core` provides the shared domain logic and data structures for all stowr
front ends. It is intended to be a small, well-tested library that each user
interface can build upon.

## Purpose

- Define the core data model for assets and collections
- Expose reusable utilities and abstractions that are UI agnostic
- Offer a simple storage layer (planned: embedded SurrealDB) so all frontends
  can operate on the same data

## Planned Features

- Centralized types for items, tags and storage locations
- Import/export helpers for various file formats
- Async task runner for background operations
- Optional local database powered by SurrealDB

This crate is still in its early stages; contributions and ideas are welcome!

## How to Implement a New Domain Entity and Repository

This guide walks you through the idiomatic stowr way to add a new domain entity
(such as `Location`, `Asset`, etc.) and its repository, using the project’s
generic `Repository` trait and type-safe IDs.

> **You’ll create:**
>
> - The [type-safe ID](#define-the-entity-id) (e.g., `LocationId`)
> - The [entity struct](#define-the-entity-struct) (e.g., `Location`)
> - The [repository trait alias](#define-the-repository-trait-alias) (e.g., `LocationRepository`)
> - A simple [repository implementation](#implement-a-repository) (e.g.,
>   `VectorLocationRepo` for in-memory/test)
> - [Tests](#write-a-test) to prove it all works

### Define the Entity ID

Use the generic `RepositoryId<T>` to create a unique, type-safe ID for your entity.

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
enum LocationTag {} // Empty marker type

type LocationId = RepositoryId<LocationTag>; // ergonomic type alias
```

### Define the Entity Struct

Make your entity struct. Include its ID and any other relevant fields.

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Location {
    id: LocationId,
    name: String,
    // Add more fields as needed
}

impl Location {
    fn new(id: LocationId, name: impl Into<String>) -> Self {
        Location { id, name: name.into() }
    }
}
```

### Define the Repository Trait Alias

For clarity, define a trait alias for your new entity’s repository:

```rust
trait LocationRepository: Repository<Entity = Location, Id = LocationId> {}
```

This enables you to refer to “any repository of `Location`” in a type-safe,
concise way.

### Implement a Repository

Here’s a simple in-memory repository using a `Mutex<Vec<Location>>`.
For real use, you’d implement a DB-backed version.

```rust
use std::sync::Mutex;
use async_trait::async_trait;

struct VectorLocationRepo {
    db: Mutex<Vec<Location>>,
}

impl VectorLocationRepo {
    fn new() -> Self {
        Self { db: Mutex::new(Vec::new()) }
    }
}

#[async_trait]
impl Repository for VectorLocationRepo {
    type Entity = Location;
    type Id = LocationId;

    async fn create(&self, entity: Location) -> Result<Location> {
        let mut guard = self.db.lock().unwrap();
        guard.push(entity.clone());
        Ok(entity)
    }

    async fn fetch(&self, id: LocationId) -> Result<Option<Location>> {
        let guard = self.db.lock().unwrap();
        Ok(guard.iter().cloned().find(|l| l.id == id))
    }
}

impl LocationRepository for VectorLocationRepo {}
```

### Write a Test

Here’s a minimal test for the repository:

```rust
#[tokio::test]
async fn vector_location_repo_works() {
    let repo = VectorLocationRepo::new();
    let id = LocationId::new();
    let loc = Location::new(id.clone(), "Lab A");
    let created = repo.create(loc.clone()).await.unwrap();
    assert_eq!(created, loc);

    let fetched = repo.fetch(id.clone()).await.unwrap();
    assert_eq!(fetched, Some(loc));
}
```

### **Pattern Recap**

- **ID**: `RepositoryId<EntityTag>`
- **Entity Struct**: struct with its typed ID
- **Repository Trait Alias**: for ergonomic trait bounds
- **Repository Implementation**: can be memory, DB, etc.
- **Tests**: Prove your repo works as expected

### **Why This Pattern?**

- **Type-safe IDs**: Impossible to mix up `LocationId`, `AssetId`, etc.
- **Repository Abstraction**: Swap implementations (e.g., for tests vs. prod) easily.
- **Consistent, readable, scalable codebase**

### **Template for Copy-Paste**

```rust
// 1. Marker for ID
enum FooTag {}
type FooId = RepositoryId<FooTag>;

// 2. Entity struct
struct Foo { id: FooId, /* fields */ }

// 3. Trait alias
trait FooRepository: Repository<Entity = Foo, Id = FooId> {}

// 4. In-memory repo (or DB)
struct VectorFooRepo { db: Mutex<Vec<Foo>> }
impl VectorFooRepo { /* ... */ }
#[async_trait]
impl Repository for VectorFooRepo { /* ... */ }
impl FooRepository for VectorFooRepo {}

// 5. Tests
#[tokio::test]
async fn foo_repo_works() { /* ... */ }
```

### Questions?

Add a note in this doc or see the `common.rs` test module for a live working example.
