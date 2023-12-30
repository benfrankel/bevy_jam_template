# Usage

TODO:
- How to replace project name
- How to build/run the project (cargo commands, `.vscode/tasks.json`)
- How to use `build.sh`
- How to use CD workflow (set up itch.io upload, push tag)

# Features

- WASM support
    - CSS loading screen before the game starts
- Github CI / CD workflows with itch.io upload
- VSCode tasks
- System ordering (`enum AppSet { Start, Update, RecordIntents, ApplyIntents, HandleEvents, QueueCommands, FlushCommands, UpdateUi, End }`)
- State sequence (`enum AppState { Boot, SplashScreen, TitleScreen, LoadingScreen, Game, EndScreen }`)
    - Screen fade in / out transition animations
    - Restart on R press in Game state
- Config file with hot-reloading (`config.rs`, with values from `assets/main.config.ron`)
    - Window settings (`WindowConfig`)
    - Color palette (`ThemeConfig`)
- UI utilities (`ui.rs`)
    - Built-in pixel fonts (`assets/font`)
    - Dynamic font size (`ui/font.rs`)
    - Basic "rich text" parsing (`ui/font.rs`)
    - Interactive buttons (`ui/interaction.rs`)
    - Tooltip on hover (`ui/tooltip.rs`)
- Debug mode for dev builds (`debug.rs`)
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

- The fonts are CC0 by [Pyrious](https://github.com/benfrankel).
- The splash screen image belongs to Bevy and is not covered by any of the licenses in this repository.
- The CSS loader / spinner is MIT (https://github.com/vineethtrv/css-loader).
- The CSS background pattern is MIT (https://github.com/Afif13/CSS-Pattern).
- The remaining code in this template is licensed under any of CC0, MIT, or Apache 2.0 at your choice.