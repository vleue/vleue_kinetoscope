# vleue_kinetoscope

![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
[![Doc](https://docs.rs/vleue_kinetoscope/badge.svg)](https://docs.rs/vleue_kinetoscope)
[![Crate](https://img.shields.io/crates/v/vleue_kinetoscope.svg)](https://crates.io/crates/vleue_kinetoscope)
[![Bevy Tracking](https://img.shields.io/badge/Bevy%20tracking-main-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![CI](https://github.com/vleue/vleue_kinetoscope/actions/workflows/ci.yml/badge.svg)](https://github.com/vleue/vleue_kinetoscope/actions/workflows/ci.yml)

Animated GIF and WebP player for Bevy.

![animated-gif](https://raw.githubusercontent.com/vleue/vleue_kinetoscope/main/animated-gif.webp)


## Usage

### System setup

Add the plugin to your app:

```rust
use bevy::prelude::*;
use vleue_kinetoscope::AnimatedImagePlugin;

fn main() {
    App::new()
        // Usually included with `DefaultPlugins`
        .add_plugins(AssetPlugin::default())
        .add_plugins(AnimatedImagePlugin);
}
```

### Play an animated gif

Spawn an entity with the bundle `AnimatedImageBundle`

```rust
use bevy::prelude::*;
use vleue_kinetoscope::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AnimatedImageBundle {
        animated_image: asset_server.load("Geneva_mechanism_6spoke_animation.gif"),
        ..default()
    });
}
```

## Bevy Support

|Bevy|vleue_kinetoscope|
|---|---|
|main|main|
|0.14|0.2|
|0.13|0.1|