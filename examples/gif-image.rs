use bevy::prelude::*;
use vleue_kinetoscope::{AnimatedGifImageBundle, AnimatedGifPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AnimatedGifPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(AnimatedGifImageBundle {
        animated_gif: asset_server.load("Geneva_mechanism_6spoke_animation.gif"),
        // animated_gif: asset_server.load("intro.gif"),
        ..default()
    });
}
