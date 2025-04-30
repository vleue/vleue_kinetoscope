use std::time::Duration;

use bevy_asset::Assets;
#[cfg(feature = "streaming")]
use bevy_ecs::system::ResMut;
use bevy_ecs::system::{Query, Res};
#[cfg(feature = "streaming")]
use bevy_image::Image;
use bevy_sprite::Sprite;
use bevy_time::{Time, Timer, TimerMode};

#[cfg(feature = "streaming")]
use crate::{Frame, StreamingAnimatedImage, StreamingAnimatedImageController};

use super::{AnimatedImage, AnimatedImageController};

pub(crate) fn image_driver(
    mut playing_images: Query<(&mut AnimatedImageController, &mut Sprite)>,
    animated_images: Res<Assets<AnimatedImage>>,
    time: Res<Time>,
) {
    // don't rely on changed or added filter as the asset can be not yet loaded at the time the component is added
    for (mut controller, mut image) in &mut playing_images {
        let Some(animated_image) = animated_images.get(&controller.animated_image) else {
            continue;
        };

        if controller.current_frame == usize::MAX
            || !animated_image.frames.iter().any(|f| f.image == image.image)
        {
            controller.current_frame = 0;
            controller.play_count = 0;
            controller.timer = Timer::new(
                Duration::from_millis(animated_image.frames[0].delay.0 as u64),
                TimerMode::Repeating,
            );
            controller.frame_count = animated_image.frames.len();
            image.image = animated_image.frames[0].image.clone_weak();
        }
        if controller.timer.tick(time.delta()).just_finished() {
            let remaining = controller.timer.elapsed();
            let new_index = (controller.current_frame + 1) % animated_image.frames.len();
            controller.timer = Timer::new(
                Duration::from_millis(animated_image.frames[new_index].delay.0 as u64),
                TimerMode::Repeating,
            );
            controller.timer.set_elapsed(remaining);
            image.image = animated_image.frames[new_index].image.clone_weak();
            controller.current_frame = new_index;
            if new_index == 0 {
                controller.play_count += 1;
            }
        }
    }
}

#[cfg(feature = "streaming")]
pub(crate) fn streaming_image_driver(
    mut playing_images: Query<(&mut StreamingAnimatedImageController, &mut Sprite)>,
    mut animated_images: ResMut<Assets<StreamingAnimatedImage>>,
    mut images: ResMut<Assets<Image>>,
    time: Res<Time>,
) {
    // don't rely on changed or added filter as the asset can be not yet loaded at the time the component is added
    for (mut controller, mut image) in &mut playing_images {
        if controller.paused() {
            continue;
        }
        let Some(animated_image) = animated_images.get_mut(&controller.animated_image) else {
            continue;
        };

        if controller.current_frame == usize::MAX {
            controller.current_frame = 0;
            let first_frame = match animated_image.next(images.as_mut()) {
                crate::StreamingFrame::Finished => continue,
                crate::StreamingFrame::Waiting => continue,
                crate::StreamingFrame::Frame { delay, image } => Frame { delay, image },
            };
            controller.timer = Timer::new(
                Duration::from_millis(first_frame.delay.0 as u64),
                TimerMode::Repeating,
            );
            image.image = first_frame.image;
        }
        if controller.timer.tick(time.delta()).finished() {
            let remaining = controller.timer.elapsed();
            let next_frame = match animated_image.next(images.as_mut()) {
                crate::StreamingFrame::Finished => {
                    controller.pause();
                    continue;
                }
                crate::StreamingFrame::Waiting => {
                    continue;
                }
                crate::StreamingFrame::Frame { delay, image } => Frame { delay, image },
            };

            controller.timer = Timer::new(
                Duration::from_millis(next_frame.delay.0 as u64),
                TimerMode::Repeating,
            );
            controller.timer.set_elapsed(remaining);
            image.image = next_frame.image;
            controller.current_frame += 1;
        }
    }
}
