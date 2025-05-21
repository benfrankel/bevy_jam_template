# Pyri New Jam

This template is optimized for game jams, with 3rd-party dependencies and some more opinionated patterns.

You can [try this template in your browser](https://pyrious.itch.io/pyri-new-jam)!

## Comparison to Bevy New 2D

This template builds on top of Bevy New 2D with a few additions.

- **Features:**
    - Hot-reloaded [config files](./assets/config/)
    - Screen transition animations (fade in / out)
    - Press P or Escape to pause
    - Restart game from pause menu
    - Basic rich text parsing
    - Dynamic font size
- **3rd-party crates:**
    - [`avian2d`](https://github.com/Jondolf/avian/) for physics
    - [`bevy_editor_pls`](https://github.com/jakobhellermann/bevy_editor_pls) for a live inspector window
    - [`bevy_simple_subsecond_system`](https://github.com/TheBevyFlock/bevy_simple_subsecond_system) for function hot-patching
    - [`leafwing-input-manager`](https://github.com/Leafwing-Studios/leafwing-input-manager/) for input to action mapping
    - [`pyri_tooltip`](https://github.com/benfrankel/pyri_tooltip/) for tooltips
    - [`pyri_state`](https://github.com/benfrankel/pyri_state) for more flexible game states
    - [`tiny_bail`](https://github.com/benfrankel/tiny_bail) for error handling ergonomics
    - And a few more...
- **Assets:**
    - [`pypx`](https://pyrious.itch.io/pypx-fonts) pixel fonts

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

<details>
  <summary>Hot-patching with Dioxus</summary>

  Follow the instructions in [`bevy_simple_subsecond_system`](https://github.com/TheBevyFlock/bevy_simple_subsecond_system/) to
  install `dioxus-cli` and set up your linker.

  Then use the following command to run your game with hot-patching enabled:

  ```shell
  dx serve --hot-patch --features native_dev --profile dev
  ```
  
  Annotate any system you add to enable hot-patching:

  ```rust
  #[cfg_attr(feature = "native_dev", hot)]
  fn my_system() {}
  ```
  
  The functions called from your systems will also be hot-patched; no annotation required!
</details>

## Release your game

To trigger a full release, navigate to `Actions` > `Release` > `Run workflow` in your GitHub repository.

> [!NOTE]
> A web release runs automatically on every commit to `main`.

# License

The source code in this repository is licensed under either of the following at your option:

- [CC0-1.0 License](./LICENSE-CC0-1.0.txt)
- [0BSD License](./LICENSE-0BSD.txt)

# Credits

- The [splash screen image](https://github.com/bevyengine/bevy/blob/main/assets/branding/bevy_logo_dark.png) belongs to the [Bevy Foundation](https://bevyengine.org/foundation/).
- The [pixel fonts](https://pyrious.itch.io/pypx-fonts) are CC0.
- The sound effects are CC0: [click](https://freesound.org/people/suntemple/sounds/253168/), [hover](https://freesound.org/people/deadsillyrabbit/sounds/251390/).
