use bevy::prelude::*;

pub(crate) mod driver;
pub(crate) mod loader;

/// An animated gif asset.
#[derive(Clone, Asset, TypePath)]
pub struct AnimatedGif {
    /// List of frames of the animated gif.
    pub frames: Vec<Frame>,
}

#[derive(Clone, Debug)]
pub struct Frame {
    /// Delay of this frame
    pub delay: (u32, u32),
    /// Handle to the image of this frame.
    pub image: Handle<Image>,
}

/// Component to help control the animation of an [`AnimatedGif`].
#[derive(Component, Clone, Default)]
pub struct AnimatedGifController {
    pub(crate) timer: Timer,
    pub(crate) play_count: usize,
    pub(crate) current_frame: usize,
    pub(crate) frame_count: usize,
}

impl AnimatedGifController {
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
}

/// A bundle of components to create an animated gif image.
#[derive(Bundle, Clone, Default)]
pub struct AnimatedGifImageBundle {
    /// Handle to the [`AnimatedGif`] asset to be drawn.
    pub animated_gif: Handle<AnimatedGif>,
    /// Controller for animation playback, given informations about current progress.
    pub controller: AnimatedGifController,
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
