# Simple dependency injection and command execution for Rust

- Compatible with WebAssembly via feature flags
- Designed for use in [Dioxus](https://dioxuslabs.com/) full stack applications
- Initially developed for [Alnwick](https://github.com/StudioLE/Alnwick)

## Packages

### `studiole_command`

A simple command execution framework with event subscription and progress tracking.

Designed for use in [Dioxus](https://dioxuslabs.com/) full stack applications.

1. WebAssembly UI adds commands to the `CommandRunner` using server functions.
2. Server executes the commands and publishes events.
3. WebAssembly UI subscribes to events and displays progress.

### `studiole_di`

A simple dependency injection framework for Rust.

## Usage

### Feature Flags

`studiole_command` has a `server` feature (enabled by default) that provides:
- `CommandRunner` worker pool
- Progress tracking with `indicatif`
- DI integration via `studiole_di`

Disable for WASM environments:

```toml
[dependencies]
studiole_command = { version = "0.1", default-features = false }
```

### Command Execution with `studiole_command`

Define a request type implementing `Executable`:

```rust
use studiole_command::prelude::*;

#[derive(Clone)]
pub struct DownloadRequest {
    pub url: String,
}

impl Display for DownloadRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Download {}", self.url)
    }
}

impl Executable for DownloadRequest {
    type Response = DownloadResponse;
    type ExecutionError = DownloadError;
}
```

Define a handler implementing `Execute`:

```rust
#[derive(Service)]
pub struct DownloadHandler {
    http: Arc<HttpClient>,
}

#[async_trait]
impl Execute<DownloadRequest, DownloadResponse, DownloadError> for DownloadHandler {
    async fn execute(&self, request: &DownloadRequest) -> Result<DownloadResponse, DownloadError> {
        let bytes = self.http.get(&request.url).await?;
        Ok(DownloadResponse { bytes })
    }
}
```

Queue and execute commands with `CommandRunner`:

```rust
let services = ServiceProvider::new()
    .with_commands()
    .await?;

let runner: Arc<CommandRunner<CommandInfo>> = services.get_service().await?;

// Start worker pool
runner.start(4).await;

// Queue requests
runner.queue_request(DownloadRequest { url: "https://example.com".to_string() }).await?;

// Wait for completion
runner.drain().await;
```

### Dependency Injection with `studiole_di`

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
