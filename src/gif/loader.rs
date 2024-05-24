use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext};
use bevy::utils::BoxedFuture;
use bevy::{prelude::*, render::render_asset::RenderAssetUsages};

use image::{codecs::gif::GifDecoder, AnimationDecoder, DynamicImage};

use super::{AnimatedGif, Frame};

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

/// Loader for animated gifs.
#[derive(Default, Clone, Copy)]
pub struct AnimatedGifLoader;

impl AnimatedGifLoader {
    fn internal_load(bytes: Vec<u8>, mut images: impl SubAssetLoader<Image>) -> AnimatedGif {
        let decoder = GifDecoder::new(bytes.as_slice()).unwrap();
        let frames = decoder.into_frames();
        let frames_from_gif = frames.collect_frames().expect("error decoding gif");

        let mut frames = vec![];
        for frame in frames_from_gif.iter() {
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
        AnimatedGif { frames }
    }

    /// For gif that need to be loaded immediately, during app setup.
    /// This can be useful if the gif is meant to be played as a loading screen.
    pub fn load_now(path: String, app: &mut App) -> Handle<AnimatedGif> {
        let mut images = app.world.resource_mut::<Assets<Image>>();
        let bytes = std::fs::read(path).unwrap();
        let gif = Self::internal_load(bytes, &mut *images);
        app.world.resource_mut::<Assets<AnimatedGif>>().add(gif)
    }
}

impl AssetLoader for AnimatedGifLoader {
    type Settings = ();
    type Asset = AnimatedGif;
    type Error = std::io::Error;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let gif = Self::internal_load(bytes, load_context);
            Ok(gif)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["gif"]
    }
}
