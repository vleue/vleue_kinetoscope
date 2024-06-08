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
#[derive(Component, Clone, Default)]
pub struct AnimatedImageController {
    pub(crate) timer: Timer,
    pub(crate) play_count: usize,
    pub(crate) current_frame: usize,
    pub(crate) frame_count: usize,
}

impl AnimatedImageController {
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
}

/// A bundle of components to create an animated image.
#[derive(Bundle, Clone, Default)]
pub struct AnimatedImageBundle {
    /// Handle to the [`AnimatedImage`] asset to be drawn.
    pub animated_image: Handle<AnimatedImage>,
    /// Controller for animation playback, given informations about current progress.
    pub controller: AnimatedImageController,
    /// Specifies the rendering properties of the sprite, such as color tint and flip.
    pub sprite: Sprite,
    /// The local transform of the sprite, relative to its parent.
    pub transform: Transform,
    /// The absolute transform of the sprite. This should generally not be written to directly.
    pub global_transform: GlobalTransform,
    /// A reference-counted handle to the image asset to be drawn.
    pub texture: Handle<Image>,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
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
