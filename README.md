# Dependency Injection for Rust

## Highlights

- Sync and Async constructors
- Resolve by type or trait
- Singleton or transient services

## Usage

### Define services

Implement `FromProvider` to describe how a type is constructed from the container:

```rust
use studiole_di::prelude::*;

struct Config {
    port: u16,
}

struct Database {
    config: Arc<Config>,
}

impl FromProvider for Database {
    type Error = ResolveError;

    fn from_provider(services: &ServiceProvider) -> Result<Self, Report<ResolveError>> {
        let config = services.get::<Config>()?;
        Ok(Self { config })
    }
}
```

### Register and resolve

```rust
let services = ServiceBuilder::new()
    .with_instance(Config { port: 8080 })
    .with_type::<Database>()
    .build();

let db = services.get::<Database>().expect("should resolve");
assert_eq!(db.config.port, 8080);
```

### Transient services

By default, services are singletons. Use the `_transient` variants for a fresh instance on every resolution:

```rust
let services = ServiceBuilder::new()
    .with_type_transient::<Database>()
    .build();
```

### Trait objects

*Requires nightly + `traits` feature*

Register a concrete type and resolve it as one or more trait objects:

```rust
let services = ServiceBuilder::new()
    .with_trait::<dyn Get, MemoryCache>()
    .with_trait::<dyn Set, MemoryCache>()
    .build();

let cache = services.get_trait::<dyn Get>().expect("should resolve");
```

Both trait registrations share the same concrete singleton.

### Async services

*Requires `async` feature*

Implement `FromProviderAsync` for services that need async construction:

```rust
struct AsyncDatabase {
    config: Arc<Config>,
}

impl FromProviderAsync for AsyncDatabase {
    type Error = ResolveError;

    async fn from_provider_async(
        services: &ServiceProvider,
    ) -> Result<Self, Report<ResolveError>> {
        let config = services.get::<Config>()?;
        Ok(Self { config })
    }
}
```

```rust
let services = ServiceBuilder::new()
    .with_instance(Config { port: 8080 })
    .with_type_async::<AsyncDatabase>()
    .build();

let db = services.get_async::<AsyncDatabase>().await.expect("should resolve");
```

Async trait objects work the same way with `with_trait_async` and `get_trait_async`.

## Migration

- [0.2 to 0.3](docs/migration-guides/0.2-to-0.3.md)

## License

This repository and its libraries are provided open source with the [AGPL-3.0](https://www.gnu.org/licenses/agpl-3.0.en.html) license that requires you must disclose your source code when you distribute, publish, or provide access to modified or derivative software.

Developers who wish to keep modified or derivative software proprietary or closed source can [get in touch for a commercial license agreements](https://studiole.uk/contact/)

> Copyright © Laurence Elsdon 2025-2026
>
> This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
>
> This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
>
> You should have received a copy of the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

→ [GNU Affero General Public License](LICENSE.md)
