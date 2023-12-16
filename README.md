# Usage
TODO:
- How to replace project name
- How to build/run the project (cargo commands, `.vscode/tasks.json`)
- How to use `build.sh`
- How to use CD workflow (set up itch.io upload, push tag)

# Features
- WASM support
- Github CI / CD workflows with itch.io upload
- VS Code tasks
- System ordering (`AppSet { Start, Update, Despawn, ApplyDeferred, End, AnimateSync, Animate }`)
- State transitions (`AppState { SplashScreen, TitleScreen, LoadingScreen, Game, EndScreen }`)
- Config file with hot-reloading (`config.rs`, with values from `assets/main.config.ron`)
    - Window settings (`WindowConfig`)
    - Color palette (`ThemeConfig`)
- UI utilities (`ui.rs`)
    - Interactive buttons (`ui/interaction_palette.rs` + `bevy_mod_picking`)
    - Built-in regular / bold pixel fonts with dynamic font size (`ui/font.rs`)
- Debug utilities in dev build (`debug.rs`)
    - Editor (`bevy_editor_pls`)
    - Hitboxes and hover tooltip (press F3 to toggle)
    - Some helpful log messages

## Planned
- [ ] Audio settings, keybindings, and an in-game settings menu
- [ ] In-game pause menu with restart and quit (to title screen) buttons
- [ ] Persistence on native + web via `bevy-persistent`
- [ ] 9-slice UI via `bevy_nice_slice_ui`

# License
The code in this template is licensed under any of CC0, MIT, or Apache 2.0 at your choice.

The fonts in this template are licensed under CC0 by [Pyrious](https://github.com/benfrankel).

The splash screen in this template belongs to Bevy and is not covered by any of the licenses in this repository.