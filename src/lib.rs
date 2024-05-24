#![warn(
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]
#![doc = include_str!("../README.md")]

use bevy::prelude::*;

use gif::driver::image_driver;

mod gif;
pub use gif::{
    loader::AnimatedGifLoader, AnimatedGif, AnimatedGifController, AnimatedGifImageBundle,
};

/// A plugin for loading and displaying animated gifs.
#[derive(Copy, Clone)]
pub struct AnimatedGifPlugin;

impl Plugin for AnimatedGifPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AnimatedGif>()
            .init_asset_loader::<AnimatedGifLoader>()
            .add_systems(Update, image_driver);
    }
}
