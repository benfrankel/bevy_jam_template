# Pyri New Jam

This template is optimized for game jams, with 3rd-party dependencies and some more opinionated patterns.

You can [try this template in your browser](https://pyrious.itch.io/pyri-new-jam)!

## Comparison to Bevy New 2D

This template builds on top of [Bevy New 2D](https://github.com/TheBevyFlock/bevy_new_2d) with a few additions:

<details>
  <summary><ins>Features</ins></summary>

  - Hot-reloaded [config files](./assets/config)
  - Screen transition animations (fade in / out)
  - Fancy buttons (drop shadow, hover offset)
  - Press P or Escape to pause
  - Restart game from pause menu
  - Basic rich text parsing
  - Dynamic font size
</details>

<details>
  <summary><ins>3rd-party crates</ins></summary>

  - [`avian2d`](https://github.com/Jondolf/avian) for 2D physics
  - [`bevy_asset_loader`](https://github.com/NiklasEi/bevy_asset_loader) for asset loading ergonomics
  - [`bevy_common_assets`](https://github.com/NiklasEi/bevy_common_assets) for loading `.ron` assets
  - [`bevy_editor_pls`](https://github.com/jakobhellermann/bevy_editor_pls) for a live inspector window
  - [`bevy_mod_debugdump`](https://github.com/jakobhellermann/bevy_mod_debugdump) for inspecting the system graph
  - [`bevy_simple_prefs`](https://github.com/rparrett/bevy_simple_prefs) for remembering settings
  - [`iyes_progress`](https://github.com/IyesGames/iyes_progress) for loading bar progress tracking
  - [`leafwing-input-manager`](https://github.com/Leafwing-Studios/leafwing-input-manager) for input-to-action mapping
  - [`pyri_state`](https://github.com/benfrankel/pyri_state) for more flexible game states
  - [`pyri_tooltip`](https://github.com/benfrankel/pyri_tooltip) for tooltips
  - [`tiny_bail`](https://github.com/benfrankel/tiny_bail) for error handling ergonomics
</details>

<details>
  <summary><ins>Assets</ins></summary>
  
  - [`pypx`](https://pyrious.itch.io/pypx-fonts) pixel fonts
  - Different [click](https://freesound.org/people/suntemple/sounds/253168/) and [hover](https://freesound.org/people/deadsillyrabbit/sounds/251390/) sound effects
  - Different music for [title](https://freesound.org/people/edtijo/sounds/240376/) and [gameplay](https://freesound.org/people/Bertsz/sounds/545458/) screens
</details>

## Getting started

Use [Bevy CLI](https://github.com/TheBevyFlock/bevy_cli) to create a new game:

```shell
bevy new game -t benfrankel/pyri_new_jam
cd game
git commit -am 'Initial commit'
```

Next, create a new GitHub repository and upload your game to it. Set up [GitHub workflows](https://docs.github.com/en/actions/writing-workflows) by following the steps described in [Bevy New 2D's documentation](https://github.com/TheBevyFlock/bevy_new_2d/blob/main/docs/workflows.md).

## Run your game

Use any of the following commands to run your game:

```shell
bevy run
bevy run web
bevy run --release
bevy run --release web
```

For VS Code users, [`.vscode/tasks.json`](./.vscode/tasks.json) provides IDE integration.

## Release your game

To trigger a full release, navigate to `Actions > Release > Run workflow` in your GitHub repository.

> [!NOTE]
> A web release runs automatically on every commit to `main`.

# License

The source code in this repository is licensed under either of the following at your option:

- [CC0-1.0 License](./LICENSE-CC0-1.0.txt)
- [0BSD License](./LICENSE-0BSD.txt)

# Credits

- The [splash screen image](https://github.com/bevyengine/bevy/blob/main/assets/branding/bevy_logo_dark.png) belongs to the [Bevy Foundation](https://bevyengine.org/foundation/).
- The [pixel fonts](https://pyrious.itch.io/pypx-fonts) are CC0.
- The [click](https://freesound.org/people/suntemple/sounds/253168/) and [hover](https://freesound.org/people/deadsillyrabbit/sounds/251390/) sound effects are CC0.
- The [title](https://freesound.org/people/edtijo/sounds/240376/) and [gameplay](https://freesound.org/people/Bertsz/sounds/545458/) music tracks are CC0.
