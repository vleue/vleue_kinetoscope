use bevy::prelude::*;

use vleue_kinetoscope::{AnimatedImagePlugin, StreamingAnimatedImageController};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AnimatedImagePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(StreamingAnimatedImageController::play(
        asset_server.load("big-buck-bunny.webp"),
    ));
}
