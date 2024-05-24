# vleue_kinetoscope

![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
[![Doc](https://docs.rs/vleue_kinetoscope/badge.svg)](https://docs.rs/vleue_kinetoscope)
[![Crate](https://img.shields.io/crates/v/vleue_kinetoscope.svg)](https://crates.io/crates/vleue_kinetoscope)
[![Bevy Tracking](https://img.shields.io/badge/Bevy%20tracking-main-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![CI](https://github.com/vleue/vleue_kinetoscope/actions/workflows/ci.yml/badge.svg)](https://github.com/vleue/vleue_kinetoscope/actions/workflows/ci.yml)

Animated GIF player for Bevy.

![animated-gif](https://raw.githubusercontent.com/vleue/vleue_kinetoscope/main/animated-gif.webp)


## Usage

### System setup

Add the plugin to your app:

```rust
use bevy::prelude::*;
use vleue_kinetoscope::AnimatedGifPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AnimatedGifPlugin);
}
```

### Play an animated gif

Spawn an entity with the bundle `AnimatedGifImageBundle`

```rust
use bevy::prelude::*;
use vleue_kinetoscope::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AnimatedGifImageBundle {
        animated_gif: asset_server.load("Geneva_mechanism_6spoke_animation.gif"),
        ..default()
    });
}
```
