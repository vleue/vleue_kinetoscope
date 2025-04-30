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
        .add_plugins(DefaultPlugins)
        .add_plugins(AnimatedImagePlugin);
}
```

### Play an animated gif

Spawn an entity with the component `AnimatedImageController`:

```rust
use bevy::prelude::*;
use vleue_kinetoscope::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(AnimatedImageController::play(asset_server.load("cube.gif")));
}
```

### Play an animated WebP

Spawn an entity with the component `AnimatedImageController`:

```rust
use bevy::prelude::*;
use vleue_kinetoscope::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(AnimatedImageController::play(asset_server.load("cube.webp")));
}
```

### Streaming animations

It is possible to stream an animated GIF or WebP, so that it's not loaded completely into Memory:

```rust
use bevy::prelude::*;
use vleue_kinetoscope::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(StreamingAnimatedImageController::play(asset_server.load("big-buck-bunny.webp")));
}
```

Streaming animated images are only played once, and can't loop.

Wasm is not yet supported for streaming animations.

## Bevy Support

| Bevy | vleue_kinetoscope |
| ---- | ----------------- |
| main | main              |
| 0.16 | 0.4               |
| 0.15 | 0.3               |
| 0.14 | 0.2               |
| 0.13 | 0.1               |
