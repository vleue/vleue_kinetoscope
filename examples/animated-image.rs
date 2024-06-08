use bevy::prelude::*;

use vleue_kinetoscope::{AnimatedImageBundle, AnimatedImageController, AnimatedImagePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AnimatedImagePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (log_updates, reset))
        .run();
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum Image {
    Gif,
    Webp,
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Image::Gif => write!(f, "GIF"),
            Image::Webp => write!(f, "WebP"),
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window: Query<&Window>) {
    commands.spawn(Camera2dBundle::default());

    let window_width = window.single().width();

    for (i, (kind, file)) in [(Image::Gif, "cube.gif"), (Image::Webp, "cube.webp")]
        .into_iter()
        .enumerate()
    {
        commands.spawn((
            AnimatedImageBundle {
                animated_image: asset_server.load(file),
                transform: Transform::from_xyz(
                    -window_width * ((-1.0 as f32).powi(i as i32)) / 2.0,
                    -75.0,
                    0.0,
                ),
                ..default()
            },
            kind,
        ));

        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(50.0),
                    height: Val::Percent(100.0),
                    top: Val::Percent(10.0),
                    left: Val::Percent(50.0 * (i as f32)),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_sections(vec![
                        TextSection {
                            value: format!("{}\n", kind),
                            style: TextStyle {
                                font_size: 60.0,
                                ..default()
                            },
                        },
                        TextSection {
                            value: "play count: ".to_string(),
                            style: TextStyle {
                                font_size: 50.0,
                                ..default()
                            },
                        },
                        TextSection {
                            value: "0".to_string(),
                            style: TextStyle {
                                font_size: 50.0,
                                ..default()
                            },
                        },
                        TextSection {
                            value: "\ncurrent frame: ".to_string(),
                            style: TextStyle {
                                font_size: 30.0,
                                ..default()
                            },
                        },
                        TextSection {
                            value: "0".to_string(),
                            style: TextStyle {
                                font_size: 30.0,
                                ..default()
                            },
                        },
                        TextSection {
                            value: " / ".to_string(),
                            style: TextStyle {
                                font_size: 30.0,
                                ..default()
                            },
                        },
                        TextSection {
                            value: "0".to_string(),
                            style: TextStyle {
                                font_size: 30.0,
                                ..default()
                            },
                        },
                    ]),
                    kind,
                ));
            });
    }
}

fn log_updates(
    mut texts: Query<(&mut Text, &Image)>,
    playing_images: Query<(Ref<AnimatedImageController>, &Image)>,
) {
    for (animated_image, image_kind) in &playing_images {
        if animated_image.is_changed() {
            for (mut text, text_kind) in &mut texts {
                if image_kind != text_kind {
                    continue;
                }
                text.sections[2].value = format!("{}", animated_image.play_count());
                text.sections[4].value = format!("{:>4}", animated_image.current_frame());
                text.sections[6].value = format!("{}", animated_image.frame_count() as i32 - 1);
            }
        }
    }
    // let gif = gif.single();
}

fn reset(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut playing_images: Query<&mut AnimatedImageController>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for mut animated_image in &mut playing_images {
            animated_image.reset();
        }
    }
}
