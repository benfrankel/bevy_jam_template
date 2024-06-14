# Getting started

## Create a new project

1. Clone this template: `git clone --depth 1 https://benfrankel/bevy_jam_template.git`
2. (Linux) Create a new project: `bevy_jam_template/create.sh PROJECT_PATH`
3. (Non-Linux) Create a new project:
    1. Copy `bevy_jam_template` to a new project path (with a valid Rust package name).
    2. Navigate into the new project path.
    3. Remove template files: `.git`, `create.sh`, `Cargo.lock`, `LICENSE-0BSD`, `LICENSE-CC0-1.0`, `README.md`.
    4. Search and replace instances of `bevy_jam_template` with your project name.
    5. Initialize git repo: `git init && git add . && git commit -m 'Initial commit'`

## Enable CI / CD

1. Create a Github repo for your project.
2. Link your local repo to the Github repo.
3. Configure the Github repo:
    1. Settings > General > Default branch = `main`
    2. Settings > Actions > General > Workflow permissions = `Read and write permissions`
    3. Settings > Secrets and variables > Actions > New repository secret
        - Name = `BUTLER_CREDENTIALS`
        - Secret = `<itch.io API keys>`
4. Create an itch.io page for your project.
5. Point the `ITCH_TARGET` value in `.github/workflows/release.yaml` to your itch.io page.
6. To trigger CI, push a commit to `main`.
7. To trigger CD, push a release tag in the format `vX.Y.Z`. Consider releasing daily during a game jam!

## Build your project

- Use `cargo build` for native dev builds (or `cargo run`, etc.).
- (Linux) Use `./build.sh` to cross-compile release builds and package for itch.io (can't build for Mac; trigger CD for that instead).
- (Non-Linux) Replicate the steps in `build.sh` by hand.
- For VS Code users, `.vscode/tasks.json` provides IDE integration.

# Features

- [Web release (WASM)](https://pyrious.itch.io/bevy-jam-template) support
    - [CSS spinner](web/style.css) before the game starts
- [Github CI / CD workflows](.github/workflows/), including itch.io upload
- [VS Code tasks](.vscode/tasks.json)
- [Game logic system ordering](src/core.rs) (via `UpdateSet` system set)
- [Main screen sequence](src/screen.rs) (via `Screen` state)
    - Screen fade in / out animations on transition
    - Restart game on R press
- [Config file](assets/default.config.ron) with [hot-reloading](src/core/config.rs) (via `apply_config` system)
    - [Window settings](src/core/window.rs)
    - [Color palette](src/core/theme.rs) (via `ThemeSpriteColor`, ... components)
- [UI utilities](src/util/ui.rs)
    - [Built-in pixel fonts](assets/font/)
    - [Dynamic font size](src/util/ui/font.rs) (via `FontSize` component)
    - [Basic "rich text" parsing](src/util/ui/font.rs) (via `parse_rich`)
    - [Interactive buttons](src/util/ui/interaction.rs) (via [`bevy_mod_picking`](https://github.com/aevyrie/bevy_mod_picking) and `InteractionPalette` component)
    - [Tooltip on hover](src/util/ui/tooltip.rs) (via `Tooltip` component)
- [Debug mode](src/core/debug.rs) for dev builds
    - Live inspector window (via [`bevy_editor_pls`](https://github.com/jakobhellermann/bevy_editor_pls))
    - Physics wireframes and picking tooltips (F3 to toggle)
    - Some helpful logging

## Planned

- [ ] Persistence on native + web via `bevy-persistent`
- [ ] 9-slice UI via `bevy_nice_slice_ui`?
- [ ] Audio settings, keybindings, and an in-game settings menu
- [ ] In-game pause menu with restart and quit (to title screen) buttons
- [ ] Debug mode cheats (e.g. type `/cmd`)

# Credit

- The [splash screen image](https://github.com/bevyengine/bevy/blob/main/assets/branding/bevy_logo_dark.png) belongs to Bevy and is not covered by any of the licenses in this repository.
- The [CSS background pattern](https://github.com/Afif13/CSS-Pattern) is MIT-licensed.
- The [CSS spinner](https://github.com/vineethtrv/css-loader) is MIT-licensed.
- The [pixel fonts](https://pyrious.itch.io/pypx-fonts) are CC0-licensed.
- The remainder of this template is made available under [CC0](LICENSE-CC0-1.0) or [0BSD](LICENSE-0BSD) at your choice.
