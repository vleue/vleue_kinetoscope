use std::time::Duration;

use bevy::prelude::*;

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
