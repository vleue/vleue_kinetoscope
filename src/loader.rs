use std::io::Cursor;
use std::path::Path;

use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::{prelude::*, render::render_asset::RenderAssetUsages};

use image::{AnimationDecoder, DynamicImage};
use thiserror::Error;

use super::{AnimatedImage, Frame};

trait SubAssetLoader<A: Asset> {
    fn add_asset(&mut self, label: String, asset: A) -> Handle<A>;
}

impl<A: Asset> SubAssetLoader<A> for &mut Assets<A> {
    fn add_asset(&mut self, _label: String, asset: A) -> Handle<A> {
        self.add(asset)
    }
}

impl<'a, A: Asset> SubAssetLoader<A> for &mut LoadContext<'a> {
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
        let frames_from_file = match path.extension() {
            Some(ext) if ext == "gif" => {
                #[cfg(feature = "gif")]
                {
                    let decoder = image::codecs::gif::GifDecoder::new(Cursor::new(bytes))
                        .map_err(AnimatedImageLoaderError::DecodingError)?;
                    let frames = decoder.into_frames();
                    frames
                        .collect_frames()
                        .map_err(AnimatedImageLoaderError::DecodingError)?
                }
                #[cfg(not(feature = "gif"))]
                {
                    return Err(AnimatedImageLoaderError::UnsupportedExtension(
                        "GIF".to_string(),
                    ));
                }
            }
            Some(ext) if ext == "webp" => {
                #[cfg(feature = "webp")]
                {
                    let decoder = image::codecs::webp::WebPDecoder::new(Cursor::new(bytes))
                        .map_err(AnimatedImageLoaderError::DecodingError)?;
                    let frames = decoder.into_frames();
                    frames
                        .collect_frames()
                        .map_err(AnimatedImageLoaderError::DecodingError)?
                }
                #[cfg(not(feature = "webp"))]
                {
                    return Err(AnimatedImageLoaderError::UnsupportedExtension(
                        "WebP".to_string(),
                    ));
                }
            }
            _ => unreachable!("unsupported extension for {}", path.display()),
        };

        let mut frames = vec![];
        for frame in frames_from_file.iter() {
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
