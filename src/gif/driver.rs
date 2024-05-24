use std::time::Duration;

use bevy::prelude::*;

use super::{AnimatedGif, AnimatedGifController};

pub(crate) fn image_driver(
    gifs: Query<(Entity, &Handle<AnimatedGif>)>,
    mut playing_gifs: Query<(
        &Handle<AnimatedGif>,
        &mut AnimatedGifController,
        &mut Handle<Image>,
    )>,
    animated_gifs: Res<Assets<AnimatedGif>>,
    time: Res<Time>,
) {
    // don't rely on changed or added filter as the asset can be not yet loaded at the time the component is added
    for (entity, new_gif) in gifs.iter() {
        let Some(gif) = animated_gifs.get(new_gif) else {
            continue;
        };

        if let Ok((_, mut controller, mut image)) = playing_gifs.get_mut(entity) {
            if controller.current_frame == usize::MAX
                || !gif.frames.iter().any(|f| f.image == *image)
            {
                *controller = AnimatedGifController {
                    timer: Timer::new(
                        Duration::from_millis(gif.frames[0].delay.0 as u64),
                        TimerMode::Repeating,
                    ),
                    current_frame: 0,
                    play_count: 0,
                    frame_count: gif.frames.len(),
                };
                *image = gif.frames[0].image.clone_weak();
            }
        }
    }

    for (gif, mut controller, mut image) in playing_gifs.iter_mut() {
        if controller.timer.tick(time.delta()).just_finished() {
            let Some(gif) = animated_gifs.get(gif) else {
                continue;
            };
            let remaining = controller.timer.elapsed();
            let new_index = (controller.current_frame + 1) % gif.frames.len();
            controller.timer = Timer::new(
                Duration::from_millis(gif.frames[new_index].delay.0 as u64),
                TimerMode::Repeating,
            );
            controller.timer.set_elapsed(remaining);
            *image = gif.frames[new_index].image.clone_weak();
            controller.current_frame = new_index;
            if new_index == 0 {
                controller.play_count += 1;
            }
        }
    }
}
