use bevy::prelude::*;

use vleue_kinetoscope::{AnimatedImageController, AnimatedImagePlugin, AnimationPlayed};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AnimatedImagePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands
        .spawn(AnimatedImageController::play(
            asset_server.load("big-buck-bunny.webp"),
        ))
        .observe(|_: Trigger<AnimationPlayed>, mut commands: Commands| {
            commands.send_event(AppExit::Success);
        });
}
