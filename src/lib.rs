use std::marker::PhantomData;

use bevy::prelude::*;

use gif::driver::image_driver;

mod gif;
pub use gif::{
    loader::AnimatedGifLoader, AnimatedGif, AnimatedGifController, AnimatedGifImageBundle,
};

pub struct AnimatedGifPlugin {
    phantom: PhantomData<fn() -> AnimatedGif>,
}

impl Default for AnimatedGifPlugin {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl Plugin for AnimatedGifPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AnimatedGif>()
            .init_asset_loader::<AnimatedGifLoader>()
            .add_systems(Update, image_driver);
    }
}
