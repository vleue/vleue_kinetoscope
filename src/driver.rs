use std::time::Duration;

use bevy::prelude::*;

use super::{AnimatedImage, AnimatedImageController};

pub(crate) fn image_driver(
    images: Query<(Entity, &Handle<AnimatedImage>)>,
    mut playing_images: Query<(
        &Handle<AnimatedImage>,
        &mut AnimatedImageController,
        &mut Handle<Image>,
    )>,
    animated_images: Res<Assets<AnimatedImage>>,
    time: Res<Time>,
) {
    // don't rely on changed or added filter as the asset can be not yet loaded at the time the component is added
    for (entity, new_animated_image) in images.iter() {
        let Some(animated_image) = animated_images.get(new_animated_image) else {
            continue;
        };

        if let Ok((_, mut controller, mut image)) = playing_images.get_mut(entity) {
            if controller.current_frame == usize::MAX
                || !animated_image.frames.iter().any(|f| f.image == *image)
            {
                *controller = AnimatedImageController {
                    timer: Timer::new(
                        Duration::from_millis(animated_image.frames[0].delay.0 as u64),
                        TimerMode::Repeating,
                    ),
                    current_frame: 0,
                    play_count: 0,
                    frame_count: animated_image.frames.len(),
                };
                *image = animated_image.frames[0].image.clone_weak();
            }
        }
    }

    for (animated_image, mut controller, mut image) in playing_images.iter_mut() {
        if controller.timer.tick(time.delta()).just_finished() {
            let Some(animated_image) = animated_images.get(animated_image) else {
                continue;
            };
            let remaining = controller.timer.elapsed();
            let new_index = (controller.current_frame + 1) % animated_image.frames.len();
            controller.timer = Timer::new(
                Duration::from_millis(animated_image.frames[new_index].delay.0 as u64),
                TimerMode::Repeating,
            );
            controller.timer.set_elapsed(remaining);
            *image = animated_image.frames[new_index].image.clone_weak();
            controller.current_frame = new_index;
            if new_index == 0 {
                controller.play_count += 1;
            }
        }
    }
}
