# Getting started

## Create a new project

1. Clone this template: `git clone --depth 1 https://github.com/benfrankel/bevy_jam_template.git`
2. Create a new project:
    1. Copy `bevy_jam_template` to a new project path (with a valid Rust package name).
    2. Navigate into the new project path.
    3. Remove template files: `.git`, `create.sh`, `Cargo.lock`, `LICENSE-0BSD`, `LICENSE-CC0-1.0`, `README.md`.
    4. Search and replace instances of `bevy_jam_template` with your project name.
    5. Initialize git repo: `git init && git add . && git commit -m 'Initial commit'`

## Enable CI / CD

1. Create a GitHub repo for your project.
2. Link your local repo to the GitHub repo.
3. Configure the GitHub repo:
    1. Settings > General > Default branch = `main`
    2. Settings > Secrets and variables > Actions > New repository secret
        - Name = `BUTLER_CREDENTIALS`
        - Secret = `<itch.io API key>`
4. Create an itch.io page for your project.
5. Point the `ITCH_TARGET` value in `.github/workflows/release.yaml` to your itch.io page.
6. To trigger CI, push a commit to `main`.
7. To trigger CD, push a release tag in the format `v1.2.3`. Consider releasing daily during a game jam!

## Build your project

- Use `cargo build` for native dev builds (or `cargo run`, etc.).
- (Linux) Use `./build.sh` to cross-compile release builds and package for itch.io (can't build for Mac; trigger CD for that instead).
- (Non-Linux) Replicate the steps in `build.sh` by hand.
- For VS Code users, `.vscode/tasks.json` provides IDE integration.

# Features

- [Web release (WASM)](https://pyrious.itch.io/bevy-jam-template) support
    - [CSS spinner](./web/style.css) before the game starts
- [Github CI / CD workflows](./.github/workflows/), including itch.io upload
- [VS Code tasks](./.vscode/tasks.json)
- [Game logic system ordering](./src/core.rs) (via `UpdateSet` system set)
- [Main screen sequence](./src/screen.rs) (via `Screen` state)
    - Screen fade in / out animations on transition
    - Restart game on R press
- [Config files](./assets/config/) with [hot-reloading](./src/util/config.rs) (via `apply_config<C>` system)
    - [Window settings](./src/core/window.rs)
    - [Color palette](./src/core/theme.rs) (via `ThemeColorFor<C>` components)
- [UI utilities](./src/util/theme.rs)
    - [Built-in pixel fonts](./assets/font/)
    - [Dynamic font size](./src/theme/text.rs) (via `DynamicFontSize` component)
    - [Basic "rich text" parsing](./src/theme/text.rs) (via `parse_rich`)
    - [Interactive buttons](./src/theme/interaction.rs) (via `bevy_picking` and the `InteractionTable` component)
    - [Tooltip on hover](./src/theme/tooltip.rs) (via `Tooltip` component)
- [Debug mode](./src/core/debug.rs) for dev builds
    - Live inspector window (via [`bevy_editor_pls`](https://github.com/jakobhellermann/bevy_editor_pls))
    - Physics wireframes and picking tooltips (F3 to toggle)
    - Some helpful logging

# Credit

- The [splash screen image](https://github.com/bevyengine/bevy/blob/main/assets/branding/bevy_logo_dark.png) belongs to the Bevy Foundation and is not covered by any of the licenses in this repository.
- The [pixel fonts](https://pyrious.itch.io/pypx-fonts) are CC0-licensed.
- The remainder of this template is made available under [CC0](./LICENSE-CC0-1.0.txt) or [0BSD](./LICENSE-0BSD.txt) at your choice.
