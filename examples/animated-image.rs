use bevy::prelude::*;

use vleue_kinetoscope::{AnimatedImageController, AnimatedImagePlugin};

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
    commands.spawn(Camera2d);

    let window_width = window.single().unwrap().width() / 2.0;

    for (i, (kind, file)) in [(Image::Gif, "cube.gif"), (Image::Webp, "cube.webp")]
        .into_iter()
        .enumerate()
    {
        commands.spawn((
            AnimatedImageController::play(asset_server.load(file)),
            Transform::from_xyz(
                -window_width * ((-1.0 as f32).powi(i as i32)) / 2.0,
                -75.0,
                0.0,
            ),
            kind,
        ));

        commands
            .spawn(Node {
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                top: Val::Percent(10.0),
                left: Val::Percent(50.0 * (i as f32)),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((Text::default(), kind)).with_children(|text| {
                    text.spawn((
                        TextSpan(format!("{}\n", kind)),
                        TextFont {
                            font_size: 60.0,
                            ..default()
                        },
                    ));
                    text.spawn((
                        TextSpan("Play Count: ".to_string()),
                        TextFont {
                            font_size: 50.0,
                            ..default()
                        },
                    ));
                    text.spawn((
                        TextSpan("0".to_string()),
                        TextFont {
                            font_size: 50.0,
                            ..default()
                        },
                    ));
                    text.spawn((
                        TextSpan("\ncurrent frame: ".to_string()),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                    ));
                    text.spawn((
                        TextSpan("0".to_string()),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                    ));
                    text.spawn((
                        TextSpan("/".to_string()),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                    ));
                    text.spawn((
                        TextSpan("0".to_string()),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                    ));
                });
            });
    }
}

fn log_updates(
    texts: Query<(Entity, &Image), With<Text>>,
    playing_images: Query<(Ref<AnimatedImageController>, &Image)>,
    mut text_writer: TextUiWriter,
) {
    for (animated_image, image_kind) in &playing_images {
        if animated_image.is_changed() {
            for (text, text_kind) in &texts {
                if image_kind != text_kind {
                    continue;
                }
                *text_writer.text(text, 3) = format!("{}", animated_image.play_count());
                *text_writer.text(text, 5) = format!("{:>4}", animated_image.current_frame());
                *text_writer.text(text, 7) = format!("{}", animated_image.frame_count() as i32 - 1);
            }
        }
    }
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
