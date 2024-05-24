use bevy::prelude::*;

use vleue_kinetoscope::{AnimatedGifController, AnimatedGifImageBundle, AnimatedGifPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AnimatedGifPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, log_updates)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(AnimatedGifImageBundle {
        animated_gif: asset_server.load("Geneva_mechanism_6spoke_animation.gif"),
        ..default()
    });

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_sections(vec![
                TextSection {
                    value: "play count: ".to_string(),
                    style: TextStyle {
                        font_size: 100.0,
                        ..default()
                    },
                },
                TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font_size: 100.0,
                        ..default()
                    },
                },
                TextSection {
                    value: "\ncurrent frame: ".to_string(),
                    style: TextStyle {
                        font_size: 60.0,
                        ..default()
                    },
                },
                TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font_size: 60.0,
                        ..default()
                    },
                },
                TextSection {
                    value: " / ".to_string(),
                    style: TextStyle {
                        font_size: 60.0,
                        ..default()
                    },
                },
                TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font_size: 60.0,
                        ..default()
                    },
                },
            ]));
        });
}

fn log_updates(mut text: Query<&mut Text>, gif: Query<Ref<AnimatedGifController>>) {
    let gif = gif.single();
    if gif.is_changed() {
        text.single_mut().sections[1].value = format!("{}", gif.play_count());
        text.single_mut().sections[3].value = format!("{:>4}", gif.current_frame());
        text.single_mut().sections[5].value = format!("{}", gif.frame_count());
    }
}
