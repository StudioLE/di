# Simple dependency injection for Rust

- Initially developed for [Alnwick](https://github.com/StudioLE/Alnwick)

## Usage

### Dependency Injection with `studiole-di`

Define services that implement the `Service` trait:

```rust
use studiole_di::prelude::*;

// Manual implementation
pub struct Database {
    connection_string: String,
}

impl Service for Database {
    type Error = DatabaseError;

    async fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self {
            connection_string: "postgres://localhost/mydb".to_string(),
        })
    }
}

// Or use the derive macro for structs with injectable fields
#[derive(Service)]
pub struct UserRepository {
    db: Arc<Database>,
}
```

Create a `ServiceProvider` and resolve services:

```rust
let services = ServiceProvider::new()
    .with_instance(Database { connection_string: "...".to_string() });

// Resolve a service (creates if not exists)
let repo: Arc<UserRepository> = services.get_service().await?;
```

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
