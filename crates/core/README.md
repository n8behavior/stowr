# stowr-core

`stowr-core` provides the shared domain logic and data structures for all stowr frontends. It is intended to be a small, well-tested library that each user interface can build upon.

## Purpose

- Define the core data model for assets and collections
- Expose reusable utilities and abstractions that are UI agnostic
- Offer a simple storage layer (planned: embedded SurrealDB) so all frontends can operate on the same data

## Planned Features

- Centralized types for items, tags and storage locations
- Import/export helpers for various file formats
- Async task runner for background operations
- Optional local database powered by SurrealDB

This crate is still in its early stages; contributions and ideas are welcome!
