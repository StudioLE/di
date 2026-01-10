# Build the web app

It is recommended to run the [docker developer environment](BUILD-WEB-DOCKER.md), however the following steps can be used if required

1. Fetch the `bulma` CSS framework and `font-awesome` icons with NPM:

```bash
npm install
```

2. Install `dx`

Follow the steps to install [dx](https://dioxuslabs.com/learn/0.6/getting_started/#)

Ensure you install the [dependencies for your platform](https://dioxuslabs.com/learn/0.6/getting_started/#linux)

Refer to the [tauri docs](https://tauri.app/start/prerequisites/#linux) if your platform aren't included.

3. Serve the dev environment

```bash
dx serve
```
