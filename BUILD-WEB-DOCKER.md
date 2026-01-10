# Build the web app with Docker

## `Dockerfile`

The [Dockerfile](docker/dioxus/Dockerfile) is configured to do the following:

1. Fetch the `bulma` CSS framework and `font-awesome` icons from `npm`.

2. Install [cargo-chef](https://github.com/LukeMathWalker/cargo-chef) and prepare a build recipe for the cargo dependencies.
This caches the dependencies to speed up subsequent builds.

3. Install the [dependencies](https://dioxuslabs.com/learn/0.7/getting_started/#linux) for running the `dx` CLI tool

4. Install the [`dx`](https://dioxuslabs.com/learn/0.7/tutorial/tooling#all-the-commands) CLI tool

5. Build the cargo dependencies with [cargo-chef](https://github.com/LukeMathWalker/cargo-chef).

6. Copy the `bulma` CSS framework and `font-awesome` icons from `npm`

The Dockerfile then provides two build targets:

- **dev**: Serve the web app using `dx serve` with hot reloading
- **release**: Build the web app using `dx build --release` and copy to a minimal Debian image

## `docker-compose.yml`

The [docker-compose.yml](docker/dioxus/docker-compose.yml) provides two services:

### `dev`

Use the root of the repo as the build context.

Use `docker/dioxus/target` as a volume for caching the cargo build artifacts.

Include `Cargo.toml` and the entire `packages` directory as volumes so `dx serve` detects changes and hot reloads.

Use your local `~/.cache/alnwick` and `~/.local/share/alnwick` as volumes for config and state

### `release`

Use the root of the repo as the build context.

Use your local `~/.cache/alnwick` and `~/.local/share/alnwick` as volumes for config and state

## Getting Started

1. Change to the `docker/dioxus` directory

```bash
cd docker/dioxus
```

2. Build the docker image

The first build of the docker image can take a while but subsequent builds are not required.

```shell
docker compose build dev
```

3. Serve the development environment.

Any changes to the source code will be hot reloaded.

The web app will be available at `http://localhost:8080`.

```shell
docker compose run --rm dev
```

4. Build the release image

```shell
docker compose build release
```

5. Run the release container

```shell
docker compose up -d release
```

The web app will be available at `http://localhost:8080`.
