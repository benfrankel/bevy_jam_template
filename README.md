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

- Web release (WASM)
    - CSS spinner before the game starts
- Github CI / CD workflows, including itch.io upload
- VSCode tasks
- Game logic system ordering (`enum UpdateSet`)
- Main screen sequence (`enum Screen`)
    - Screen fade in / out animations on transition
    - Restart on R press during `Screen::Playing`
- Config file with hot-reloading (`common/config.rs`, with values from `assets/main.config.ron`)
    - Window settings (`WindowConfig`)
    - Color palette (`ThemeConfig`)
- UI utilities (`ui.rs`)
    - Built-in pixel fonts (`assets/font`)
    - Dynamic font size (`ui/font.rs`)
    - Basic "rich text" parsing (`ui/font.rs`)
    - Interactive buttons (`ui/interaction.rs`)
    - Tooltip on hover (`ui/tooltip.rs`)
- Debug mode for dev builds (`common/debug.rs`)
    - Editor window (`bevy_editor_pls`)
    - Hitbox wireframes and picking tooltip (press F3 to toggle)
    - Some helpful log messages

## Planned

- [ ] Persistence on native + web via `bevy-persistent`
- [ ] 9-slice UI via `bevy_nice_slice_ui`?
- [ ] Audio settings, keybindings, and an in-game settings menu
- [ ] In-game pause menu with restart and quit (to title screen) buttons
- [ ] Debug mode cheats (e.g. type `/cmd`)

# License

- The fonts are [CC0](LICENSE-CC0-1.0) by [Pyrious](https://github.com/benfrankel).
- The splash screen image belongs to Bevy and is not covered by any of the licenses in this repository.
- The CSS spinner is MIT (https://github.com/vineethtrv/css-loader).
- The CSS background pattern is MIT (https://github.com/Afif13/CSS-Pattern).
- The remaining code in this template is available under either [CC0](LICENSE-CC0-1.0) or [0BSD](LICENSE-0BSD) at your choice.
