use std::{io::Cursor, path::Path};

use bevy_app::App;
use bevy_asset::{Asset, AssetLoader, Assets, Handle, LoadContext, RenderAssetUsages, io::Reader};
use bevy_image::Image;

use image::{AnimationDecoder, DynamicImage, Frames};
use thiserror::Error;

use crate::FrameChannel;

use super::{AnimatedImage, Frame, StreamingAnimatedImage};

trait SubAssetLoader<A: Asset> {
    fn add_asset(&mut self, label: String, asset: A) -> Handle<A>;
}

impl<A: Asset> SubAssetLoader<A> for &mut Assets<A> {
    fn add_asset(&mut self, _label: String, asset: A) -> Handle<A> {
        self.add(asset)
    }
}

impl<A: Asset> SubAssetLoader<A> for &mut LoadContext<'_> {
    fn add_asset(&mut self, label: String, asset: A) -> Handle<A> {
        self.add_labeled_asset(label, asset)
    }
}

/// Loader for animated images (GIF and WebP).
#[derive(Default, Clone, Copy)]
pub struct AnimatedImageLoader;

impl AnimatedImageLoader {
    fn internal_load(
        bytes: Vec<u8>,
        mut images: impl SubAssetLoader<Image>,
        path: &Path,
    ) -> Result<AnimatedImage, AnimatedImageLoaderError> {
        let frames_from_file: Frames<'_> = match path.extension() {
            Some(ext) if ext == "gif" => {
                #[cfg(feature = "gif")]
                {
                    let decoder = image::codecs::gif::GifDecoder::new(Cursor::new(bytes))
                        .map_err(AnimatedImageLoaderError::DecodingError)?;
                    decoder.into_frames()
                }
                #[cfg(not(feature = "gif"))]
                {
                    return Err(AnimatedImageLoaderError::UnsupportedImageFormat(
                        "GIF".to_string(),
                    ));
                }
            }
            Some(ext) if ext == "webp" => {
                #[cfg(feature = "webp")]
                {
                    let decoder = image::codecs::webp::WebPDecoder::new(Cursor::new(bytes))
                        .map_err(AnimatedImageLoaderError::DecodingError)?;
                    decoder.into_frames()
                }
                #[cfg(not(feature = "webp"))]
                {
                    return Err(AnimatedImageLoaderError::UnsupportedImageFormat(
                        "WebP".to_string(),
                    ));
                }
            }
            _ => unreachable!("unsupported extension for {}", path.display()),
        };

        let mut frames = vec![];
        for frame in frames_from_file {
            let frame = frame.map_err(AnimatedImageLoaderError::DecodingError)?;
            let image = Image::from_dynamic(
                DynamicImage::ImageRgba8(frame.buffer().clone()),
                true,
                RenderAssetUsages::RENDER_WORLD,
            );
            let handle = images.add_asset(format!("frame-{}", frames.len()), image);
            frames.push(Frame {
                delay: frame.delay().numer_denom_ms(),
                image: handle.clone(),
            });
        }
        Ok(AnimatedImage { frames })
    }

    /// For gif that need to be loaded immediately, during app setup.
    /// This can be useful if the gif is meant to be played as a loading screen.
    pub fn load_now(
        path: String,
        app: &mut App,
    ) -> Result<Handle<AnimatedImage>, AnimatedImageLoaderError> {
        let mut images = app.world_mut().resource_mut::<Assets<Image>>();
        let bytes = std::fs::read(&path).map_err(AnimatedImageLoaderError::IoError)?;
        let gif = Self::internal_load(bytes, &mut *images, Path::new(&path))?;
        Ok(app
            .world_mut()
            .resource_mut::<Assets<AnimatedImage>>()
            .add(gif))
    }

    /// For animated image that need to be loaded immediately, during app setup.
    /// This can be useful if the video is meant to be played as a loading screen.
    /// Use `included_bytes!` macro to load the bytes.
    /// # Example
    /// ```
    /// # use vleue_kinetoscope::AnimatedImageLoader;
    /// # use bevy::prelude::AssetApp;
    /// # let mut app = bevy::prelude::App::new();
    /// # app.add_plugins(bevy::prelude::AssetPlugin::default());
    /// # app.init_asset::<bevy::prelude::Image>();
    /// app.add_plugins(vleue_kinetoscope::AnimatedImagePlugin);
    /// let bytes = include_bytes!("../assets/cube.gif");
    /// let handle = AnimatedImageLoader::load_now_from_bytes(bytes, "cube.gif", &mut app).unwrap();
    /// ```
    pub fn load_now_from_bytes(
        bytes: &[u8],
        extension: &str,
        app: &mut App,
    ) -> Result<Handle<AnimatedImage>, AnimatedImageLoaderError> {
        let mut images = app.world_mut().resource_mut::<Assets<Image>>();
        let bytes = bytes.to_vec();
        let gif = Self::internal_load(bytes, &mut *images, Path::new(&format!("foo.{extension}")))?;
        Ok(app
            .world_mut()
            .resource_mut::<Assets<AnimatedImage>>()
            .add(gif))
    }
}

#[derive(Debug, Error)]
pub enum AnimatedImageLoaderError {
    #[error("Error reading data: {0}")]
    IoError(std::io::Error),
    #[error("Unsupported image format: {0} (you should enable the corresponding feature)")]
    UnsupportedImageFormat(String),
    #[error("Error decoding image: {0}")]
    DecodingError(image::ImageError),
}

impl AssetLoader for AnimatedImageLoader {
    type Settings = ();
    type Asset = AnimatedImage;
    type Error = AnimatedImageLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader
            .read_to_end(&mut bytes)
            .await
            .map_err(AnimatedImageLoaderError::IoError)?;
        let path = load_context.path().to_owned();
        let gif = Self::internal_load(bytes, load_context, &path)?;
        Ok(gif)
    }

    fn extensions(&self) -> &[&str] {
        &[
            #[cfg(feature = "gif")]
            "gif",
            #[cfg(feature = "webp")]
            "webp",
        ]
    }
}

/// Loader for animated images (GIF and WebP).
#[derive(Default, Clone, Copy)]
pub struct StreamingAnimatedImageLoader;

impl StreamingAnimatedImageLoader {
    fn internal_load(
        bytes: Vec<u8>,
        mut images: impl SubAssetLoader<Image>,
        path: &Path,
    ) -> Result<StreamingAnimatedImage, AnimatedImageLoaderError> {
        let (sender, receiver) = crossbeam_channel::bounded(2);

        let extension = path.extension().map(|s| s.to_ascii_lowercase());
        std::thread::spawn(move || {
            let mut frames: Frames<'_> = match extension {
                Some(ext) if ext == "gif" => {
                    #[cfg(feature = "gif")]
                    {
                        let decoder = image::codecs::gif::GifDecoder::new(Cursor::new(bytes))
                            .map_err(AnimatedImageLoaderError::DecodingError)
                            .unwrap();
                        decoder.into_frames()
                    }
                    #[cfg(not(feature = "gif"))]
                    {
                        panic!(
                            "{}",
                            AnimatedImageLoaderError::UnsupportedImageFormat("GIF".to_string(),)
                        );
                    }
                }
                Some(ext) if ext == "webp" => {
                    #[cfg(feature = "webp")]
                    {
                        let decoder = image::codecs::webp::WebPDecoder::new(Cursor::new(bytes))
                            .map_err(AnimatedImageLoaderError::DecodingError)
                            .unwrap();
                        decoder.into_frames()
                    }
                    #[cfg(not(feature = "webp"))]
                    {
                        panic!(
                            "{}",
                            AnimatedImageLoaderError::UnsupportedImageFormat("WebP".to_string(),)
                        );
                    }
                }
                _ => unreachable!("unsupported extension for {:?}", extension),
            };
            loop {
                let Some(next_frame) = frames.next() else {
                    let _ = sender.send(FrameChannel::Finished);
                    break;
                };
                let next_frame = next_frame.unwrap();
                let _ = sender.send(FrameChannel::Frame(next_frame));
            }
        });

        let buffered = (0..5)
            .map_while(|i| receiver.recv().ok().map(|f| (i, f)))
            .map_while(|(i, frame)| {
                let frame = match frame {
                    FrameChannel::Frame(frame) => frame,
                    FrameChannel::Finished => return None,
                };
                let image = Image::from_dynamic(
                    DynamicImage::ImageRgba8(frame.buffer().clone()),
                    true,
                    RenderAssetUsages::RENDER_WORLD,
                );
                let handle = images.add_asset(format!("frame-{}", i), image);
                Some(Some(Frame {
                    delay: frame.delay().numer_denom_ms(),
                    image: handle.clone(),
                }))
            })
            .collect();

        Ok(StreamingAnimatedImage {
            // frames: Arc::new(Mutex::new(frames)),
            frame_receiver: receiver,
            buffered,
        })
    }
}

impl AssetLoader for StreamingAnimatedImageLoader {
    type Settings = ();
    type Asset = StreamingAnimatedImage;
    type Error = AnimatedImageLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader
            .read_to_end(&mut bytes)
            .await
            .map_err(AnimatedImageLoaderError::IoError)?;
        let path = load_context.path().to_owned();
        let gif = Self::internal_load(bytes, load_context, &path)?;
        Ok(gif)
    }

    fn extensions(&self) -> &[&str] {
        &[
            #[cfg(feature = "gif")]
            "gif",
            #[cfg(feature = "webp")]
            "webp",
        ]
    }
}
