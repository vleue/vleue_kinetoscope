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

mod driver;
mod loader;

use bevy_app::{App, Plugin, Update};
use bevy_asset::{Asset, AssetApp, Handle};
#[cfg(feature = "streaming")]
use bevy_asset::{Assets, RenderAssetUsages};
use bevy_ecs::{component::Component, event::Event};
use bevy_image::Image;
use bevy_reflect::TypePath;
use bevy_sprite::Sprite;
use bevy_time::Timer;

#[cfg(feature = "streaming")]
use crossbeam_channel::Receiver;
use driver::image_driver;
#[cfg(feature = "streaming")]
use driver::streaming_image_driver;
pub use loader::AnimatedImageLoader;
#[cfg(feature = "streaming")]
pub use loader::StreamingAnimatedImageLoader;

#[cfg(feature = "streaming")]
use image::DynamicImage;
#[cfg(feature = "streaming")]
use smallvec::SmallVec;

/// An animated image asset.
#[derive(Clone, Asset, TypePath)]
pub struct AnimatedImage {
    /// List of frames of the animated image.
    pub frames: Vec<Frame>,
}

#[cfg(feature = "streaming")]
const FRAME_BUFFER_SIZE: usize = 5;

/// An animated image asset.
#[derive(Asset, TypePath)]
#[cfg(feature = "streaming")]
pub struct StreamingAnimatedImage {
    // frames: Arc<Mutex<Frames<'static>>>,
    frame_receiver: Receiver<FrameChannel>,
    // frame_command: Sender<FrameCommand>,
    buffered: SmallVec<[Option<Frame>; FRAME_BUFFER_SIZE]>,
}

#[cfg(feature = "streaming")]
enum FrameChannel {
    Frame(image::Frame),
    Finished,
}

#[cfg(feature = "streaming")]
impl StreamingAnimatedImage {
    /// Get the next frame of the animated image.
    pub fn next(&mut self, images: &mut Assets<Image>) -> StreamingFrame {
        let next = self.buffered.pop();

        if next.is_some() && next.clone().unwrap().is_none() {
            return StreamingFrame::Finished;
        }

        let Some(frame) = self.frame_receiver.try_recv().ok() else {
            return match next {
                Some(Some(next)) => StreamingFrame::Frame {
                    delay: next.delay,
                    image: next.image,
                },
                _ => StreamingFrame::Waiting,
            };
        };

        let to_buffer = match frame {
            FrameChannel::Frame(frame) => {
                let image = Image::from_dynamic(
                    DynamicImage::ImageRgba8(frame.buffer().clone()),
                    true,
                    RenderAssetUsages::RENDER_WORLD,
                );
                let handle = images.add(image);
                Some(Frame {
                    delay: frame.delay().numer_denom_ms(),
                    image: handle,
                })
            }
            FrameChannel::Finished => None,
        };

        self.buffered.insert(0, to_buffer);

        match next {
            Some(Some(next)) => StreamingFrame::Frame {
                delay: next.delay,
                image: next.image,
            },
            _ => StreamingFrame::Waiting,
        }
    }
}

/// Frame of an animated image.
#[derive(Clone, Debug)]
pub struct Frame {
    /// Delay of this frame
    pub delay: (u32, u32),
    /// Handle to the image of this frame.
    pub image: Handle<Image>,
}

/// Frame of an animated image.
#[derive(Clone, Debug)]
#[cfg(feature = "streaming")]
pub enum StreamingFrame {
    /// Stream is finished
    Finished,
    /// Stream is waiting for more frames
    Waiting,
    /// Next frame in the stream
    Frame {
        /// Delay of this frame
        delay: (u32, u32),
        /// Handle to the image of this frame.
        image: Handle<Image>,
    },
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

/// Component to help control the animation of an [`StreamingAnimatedImage`].
#[derive(Component, Clone)]
#[require(Sprite)]
#[cfg(feature = "streaming")]
pub struct StreamingAnimatedImageController {
    pub(crate) animated_image: Handle<StreamingAnimatedImage>,
    pub(crate) timer: Timer,
    pub(crate) current_frame: usize,
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

#[cfg(feature = "streaming")]
impl StreamingAnimatedImageController {
    /// Create a new controller for an animated image and starts playing it.
    pub fn play(animated_image: Handle<StreamingAnimatedImage>) -> Self {
        Self {
            animated_image,
            timer: Timer::default(),
            current_frame: usize::MAX,
        }
    }

    /// Current frame of the animation.
    pub fn current_frame(&self) -> usize {
        self.current_frame
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
            .add_event::<AnimationPlayed>()
            .init_asset_loader::<AnimatedImageLoader>()
            .add_systems(Update, image_driver);
        #[cfg(feature = "streaming")]
        app.init_asset::<StreamingAnimatedImage>()
            .init_asset_loader::<StreamingAnimatedImageLoader>()
            .add_systems(Update, streaming_image_driver);
    }
}

/// Event triggered when an animation finishes playing.
#[derive(Event, Debug, Copy, Clone)]
pub struct AnimationPlayed(pub usize);
