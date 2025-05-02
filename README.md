# Bevy Jam Template

This template is optimized for game jams, with 3rd-party dependencies and some more opinionated patterns.

You can [try this template in your browser](https://pyrious.itch.io/bevy-jam-template)!

## Comparison to Bevy New 2D

This template builds on top of Bevy New 2D with a few additions.

- **Features:**
    - Hot-reloaded [config files](./assets/config/)
    - Screen transition animations (fade in / out)
    - Press P or Escape to pause
    - Press R to restart
    - Basic rich text parsing
    - Dynamic font size
- **3rd-party crates:**
    - [`avian2d`](https://github.com/Jondolf/avian/) for physics
    - [`bevy_editor_pls`](https://github.com/jakobhellermann/bevy_editor_pls) for a live inspector window
    - [`leafwing-input-manager`](https://github.com/Leafwing-Studios/leafwing-input-manager/) for input to action mapping
    - [`pyri_tooltip`](https://github.com/benfrankel/pyri_tooltip/) for tooltips
    - [`pyri_state`](https://github.com/benfrankel/pyri_state) for flexible game states
    - [`tiny_bail`](https://github.com/benfrankel/tiny_bail) for error handling ergonomics
    - And a few more...
- **Assets:**
    - [`pypx`](https://pyrious.itch.io/pypx-fonts) pixel fonts

## Getting started

Use [Bevy CLI](https://github.com/TheBevyFlock/bevy_cli) to create a new game:

```shell
bevy new game -t benfrankel/bevy_jam_template`
cd game
git commit -am 'Initial commit'
```

Next, create a new GitHub repository and upload your game to it. Set up [GitHub workflows](https://docs.github.com/en/actions/writing-workflows) by following the steps described in [Bevy New 2D's documentation](https://github.com/TheBevyFlock/bevy_new_2d/blob/main/docs/workflows.md).

## Run your game

Use the following commands to run your game:

```shell
bevy run
bevy run web
bevy run --release
bevy run --release web
```

For VS Code users, [`.vscode/tasks.json`](./.vscode/tasks.json) provides IDE integration.

## Release your game

The easiest way to release your game is by triggering the [release workflow](./.github/workflows/release.yaml).

Alternatively, you can cross-compile locally from Linux with [`./build.sh`](./build.sh). If you're not on Linux, you can manually follow the steps in the shell script.
  
Note: `./build.sh mac` is currently not implemented.

# License

The source code in this repository is licensed under any of the following at your option:

- [CC0-1.0 License](./LICENSE-CC0-1.0.txt)
- [0BSD License](./LICENSE-0BSD.txt)

# Credits

- The [splash screen image](https://github.com/bevyengine/bevy/blob/main/assets/branding/bevy_logo_dark.png) belongs to the Bevy Foundation and is not covered by any of the licenses in this repository.
- The [pixel fonts](https://pyrious.itch.io/pypx-fonts) are CC0.
- The sound effects are CC0:
    - [Button click](https://freesound.org/people/suntemple/sounds/253168/)
    - [Button hover](https://freesound.org/people/deadsillyrabbit/sounds/251390/)
