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
#![allow(clippy::needless_doctest_main)]
#![doc = include_str!("../README.md")]

use bevy::prelude::*;

mod driver;
mod loader;

use driver::image_driver;
pub use loader::AnimatedImageLoader;

/// An animated image asset.
#[derive(Clone, Asset, TypePath)]
pub struct AnimatedImage {
    /// List of frames of the animated image.
    pub frames: Vec<Frame>,
}

/// Frame of an animated image.
#[derive(Clone, Debug)]
pub struct Frame {
    /// Delay of this frame
    pub delay: (u32, u32),
    /// Handle to the image of this frame.
    pub image: Handle<Image>,
}

/// Component to help control the animation of an [`AnimatedImage`].
#[derive(Component, Clone)]
#[require(Sprite)]
pub struct AnimatedImageController {
    pub(crate) animated_image: Handle<AnimatedImage>,
    pub(crate) timer: Timer,
    pub(crate) play_count: usize,
    pub(crate) current_frame: usize,
    pub(crate) frame_count: usize,
}

impl AnimatedImageController {
    /// Create a new controller for an animated image and starts playing it.
    pub fn play(animated_image: Handle<AnimatedImage>) -> Self {
        Self {
            animated_image,
            timer: Timer::default(),
            play_count: 0,
            current_frame: usize::MAX,
            frame_count: 0,
        }
    }

    /// Number of times the animation has looped.
    pub fn play_count(&self) -> usize {
        self.play_count
    }
    /// Current frame of the animation.
    pub fn current_frame(&self) -> usize {
        self.current_frame
    }

    /// How many frames the animation has.
    pub fn frame_count(&self) -> usize {
        self.frame_count
    }

    /// Reset the animation to the first frame.
    pub fn reset(&mut self) {
        self.current_frame = usize::MAX;
        self.play_count = 0;
    }

    /// Pause the animation.
    pub fn pause(&mut self) {
        self.timer.pause();
    }

    /// Unpause the animation.
    pub fn unpause(&mut self) {
        self.timer.unpause();
    }

    /// Returns true if the animation is paused.
    pub fn paused(&mut self) -> bool {
        self.timer.paused()
    }
}

/// A plugin for loading and displaying animated images (GIF or WebP).
#[derive(Copy, Clone)]
pub struct AnimatedImagePlugin;

impl Plugin for AnimatedImagePlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AnimatedImage>()
            .init_asset_loader::<AnimatedImageLoader>()
            .add_systems(Update, image_driver);
    }
}
