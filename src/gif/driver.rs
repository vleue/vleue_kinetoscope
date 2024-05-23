use std::time::Duration;

use bevy::prelude::*;
// use bevy_asset::{Assets, Handle, HandleId};
// use bevy_ecs::{
//     entity::Entity,
//     system::Query,
//     system::{Local, Res, ResMut},
// };
// use bevy_pbr::StandardMaterial;
// use bevy_render::texture::Image;
// use bevy_sprite::{TextureAtlas, TextureAtlasSprite};
// use bevy_time::{Time, Timer, TimerMode};
// use bevy_utils::HashMap;

use super::{AnimatedGif, AnimatedGifTimer};

// pub(crate) fn texture_atlas_driver(
//     gifs: Query<(Entity, &Handle<AnimatedGif>)>,
//     mut playing_gifs: Query<(
//         &Handle<AnimatedGif>,
//         &mut AnimatedGifTimer,
//         &mut TextureAtlas,
//     )>,
//     mut atlases: Query<&mut Handle<TextureAtlasLayout>>,
//     animated_gifs: Res<Assets<AnimatedGif>>,
//     time: Res<Time>,
// ) {
//     for (entity, new_gif) in gifs.iter() {
//         let Some(gif) = animated_gifs.get(new_gif) else {
//             continue;
//         };
//         if let Ok(mut atlas_handle) = atlases.get_mut(entity) {
//             if *atlas_handle == gif.atlas {
//                 continue;
//             }
//             *atlas_handle = gif.atlas.clone_weak();
//         }
//         if let Ok((_, mut timer, mut sprite)) = playing_gifs.get_mut(entity) {
//             *timer = AnimatedGifTimer(
//                 Timer::new(
//                     Duration::from_millis(gif.frames[0].delay.0 as u64),
//                     TimerMode::Repeating,
//                 ),
//                 0,
//             );
//             *sprite = TextureAtlas::new(0);
//         }
//     }
//     for (gif, mut timer, mut sprite) in playing_gifs.iter_mut() {
//         if timer.0.tick(time.delta()).just_finished() {
//             let Some(gif) = animated_gifs.get(gif) else {
//                 continue;
//             };
//             let remaining = timer.0.elapsed();
//             let index = (sprite.index + 1) % gif.frames.len();
//             timer.0 = Timer::new(
//                 Duration::from_millis(gif.frames[index].delay.0 as u64),
//                 TimerMode::Repeating,
//             );
//             timer.0.set_elapsed(remaining);
//             sprite.index = index;
//         }
//     }
// }

// pub(crate) fn magic_driver(
//     gifs: Res<Assets<AnimatedGif>>,
//     mut images: ResMut<Assets<Image>>,
//     time: Res<Time>,
//     mut timers: Local<
//         std::collections::HashMap<Handle<AnimatedGif>, (AnimatedGifTimer, Handle<Image>)>,
//     >,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     strong_handles: Query<&Handle<Image>>,
// ) {
//     for gif in gifs.iter() {
//         let handle_image = &Handle::weak(gif.0);
//         if images.contains(&handle_image) {
//             let timer = timers.get_mut(&gif.0).unwrap();
//             if timer.0 .0.tick(time.delta()).just_finished() {
//                 let index = (timer.0 .1 + 1) % gif.1.frames.len();
//                 let remaining = timer.0 .0.elapsed();
//                 timer.0 .0 = Timer::new(
//                     Duration::from_millis(gif.1.frames[index].delay.0 as u64),
//                     TimerMode::Repeating,
//                 );
//                 timer.0 .0.set_elapsed(remaining);
//                 timer.0 .1 = index;
//                 let image = images.get(&gif.1.frames[index].image).unwrap().clone();
//                 // let _ = images.set(handle_image, image);
//                 *images.get_mut(handle_image).unwrap() = image;
//             }
//         } else {
//             if strong_handles
//                 .iter()
//                 .any(|h| h.is_strong() && h.id() == gif.0)
//             {
//                 let image = images.get(&gif.1.frames[0].image).unwrap().clone();
//                 let handle = images.set(handle_image, image);
//                 timers.insert(
//                     gif.0,
//                     (
//                         AnimatedGifTimer(
//                             Timer::new(
//                                 Duration::from_millis(gif.1.frames[0].delay.0 as u64),
//                                 TimerMode::Repeating,
//                             ),
//                             0,
//                         ),
//                         handle_image.clone_weak(),
//                     ),
//                 );
//             }
//         }
//     }
//     // for material in materials.iter_mut() {
//     //     println!("coucou");
//     // }
// }

pub(crate) fn image_driver(
    gifs: Query<(Entity, &Handle<AnimatedGif>)>,
    mut playing_gifs: Query<(
        &Handle<AnimatedGif>,
        &mut AnimatedGifTimer,
        &mut Handle<Image>,
    )>,
    animated_gifs: Res<Assets<AnimatedGif>>,
    images: Res<Assets<Image>>,
    time: Res<Time>,
) {
    for (entity, new_gif) in gifs.iter() {
        let Some(gif) = animated_gifs.get(new_gif) else {
            continue;
        };
        // println!("got one");

        if let Ok((_, mut timer, mut image)) = playing_gifs.get_mut(entity) {
            if !gif.frames.iter().any(|f| f.image == *image) {
                *timer = AnimatedGifTimer(
                    Timer::new(
                        Duration::from_millis(gif.frames[0].delay.0 as u64),
                        TimerMode::Repeating,
                    ),
                    0,
                );
                *image = gif.frames[0].image.clone_weak();
            }
        }
    }
    for (gif, mut timer, mut image) in playing_gifs.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            let Some(gif) = animated_gifs.get(gif) else {
                continue;
            };
            let remaining = timer.0.elapsed();
            let index = (gif
                .frames
                .iter()
                .enumerate()
                .find(|(_, f)| f.image == *image)
                .unwrap()
                .0
                + 1)
                % gif.frames.len();
            timer.0 = Timer::new(
                Duration::from_millis(gif.frames[index].delay.0 as u64),
                TimerMode::Repeating,
            );
            timer.0.set_elapsed(remaining);
            *image = gif.frames[index].image.clone_weak();
        }
    }
}
