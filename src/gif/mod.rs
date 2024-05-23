// use bevy_asset::Handle;
// use bevy_ecs::{
//     bundle::Bundle,
//     component::Component,
//     system::{lifetimeless::SRes, SystemParamItem},
// };
// use bevy_math::Vec2;
// use bevy_reflect::TypeUuid;
// use bevy_render::{
//     render_asset::{PrepareAssetError, RenderAsset},
//     render_resource::TextureViewDescriptor,
//     renderer::{RenderDevice, RenderQueue},
//     texture::{DefaultImageSampler, GpuImage, Image, ImageSampler},
//     view::{ComputedVisibility, Visibility},
// };
// use bevy_sprite::{Sprite, TextureAtlas, TextureAtlasSprite};
// use bevy_time::Timer;
// use bevy_transform::components::{GlobalTransform, Transform};
use bevy::{
    ecs::system::lifetimeless::SRes,
    prelude::*,
    render::{
        render_asset::{PrepareAssetError, RenderAsset, RenderAssetUsages},
        renderer::{RenderDevice, RenderQueue},
        texture::{DefaultImageSampler, GpuImage, ImageSampler},
    },
};

pub(crate) mod driver;
pub(crate) mod loader;

#[derive(Clone, Asset, TypePath)]
pub struct AnimatedGif {
    pub(crate) frames: Vec<Frame>,
    // pub(crate) atlas: Handle<TextureAtlasLayout>,
}

// impl RenderAsset for AnimatedGif {
//     type PreparedAsset = GpuImage;
//     type Param = (
//         SRes<RenderDevice>,
//         SRes<RenderQueue>,
//         SRes<DefaultImageSampler>,
//     );

//     fn asset_usage(&self) -> RenderAssetUsages {
//         // self.asset_usage
//         RenderAssetUsages::all()
//     }

//     // type ExtractedAsset = Image;

//     // type PreparedAsset = GpuImage;
//     // type Param = (
//     //     SRes<RenderDevice>,
//     //     SRes<RenderQueue>,
//     //     SRes<DefaultImageSampler>,
//     // );

//     // /// Clones the Image.
//     // fn extract_asset(&self) -> Self::ExtractedAsset {
//     //     // self.clone()
//     //     todo!()
//     // }

//     // /// Converts the extracted image into a [`GpuImage`].
//     // fn prepare_asset(
//     //     image: Self::ExtractedAsset,
//     //     (render_device, render_queue, default_sampler): &mut SystemParamItem<Self::Param>,
//     // ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
//     //     let texture = render_device.create_texture_with_data(
//     //         render_queue,
//     //         &image.texture_descriptor,
//     //         &image.data,
//     //     );

//     //     let texture_view = texture.create_view(
//     //         image
//     //             .texture_view_descriptor
//     //             .or_else(|| Some(TextureViewDescriptor::default()))
//     //             .as_ref()
//     //             .unwrap(),
//     //     );
//     //     let size = Vec2::new(
//     //         image.texture_descriptor.size.width as f32,
//     //         image.texture_descriptor.size.height as f32,
//     //     );
//     //     let sampler = match image.sampler_descriptor {
//     //         ImageSampler::Default => (***default_sampler).clone(),
//     //         ImageSampler::Descriptor(descriptor) => render_device.create_sampler(&descriptor),
//     //     };

//     //     Ok(GpuImage {
//     //         texture,
//     //         texture_view,
//     //         texture_format: image.texture_descriptor.format,
//     //         sampler,
//     //         size,
//     //         mip_level_count: image.texture_descriptor.mip_level_count,
//     //     })
//     // }

//     /// Converts the extracted image into a [`GpuImage`].
//     fn prepare_asset(
//         self,
//         (render_device, render_queue, default_sampler): &mut SystemParamItem<Self::Param>,
//     ) -> Result<Self::PreparedAsset, PrepareAssetError<Self>> {
//         let texture = render_device.create_texture_with_data(
//             render_queue,
//             &self.texture_descriptor,
//             // TODO: Is this correct? Do we need to use `MipMajor` if it's a ktx2 file?
//             wgpu::util::TextureDataOrder::default(),
//             &self.data,
//         );

//         let texture_view = texture.create_view(
//             self.texture_view_descriptor
//                 .or_else(|| Some(TextureViewDescriptor::default()))
//                 .as_ref()
//                 .unwrap(),
//         );
//         let size = Vec2::new(
//             self.texture_descriptor.size.width as f32,
//             self.texture_descriptor.size.height as f32,
//         );
//         let sampler = match self.sampler {
//             ImageSampler::Default => (***default_sampler).clone(),
//             ImageSampler::Descriptor(descriptor) => {
//                 render_device.create_sampler(&descriptor.as_wgpu())
//             }
//         };

//         Ok(GpuImage {
//             texture,
//             texture_view,
//             texture_format: self.texture_descriptor.format,
//             sampler,
//             size,
//             mip_level_count: self.texture_descriptor.mip_level_count,
//         })
//     }
// }

#[derive(Clone, Debug)]
pub struct Frame {
    pub(crate) delay: (u32, u32),
    pub(crate) image: Handle<Image>,
}

#[derive(Component, Clone, Default)]
pub struct AnimatedGifTimer(pub(crate) Timer, pub(crate) usize);

#[derive(Bundle, Clone, Default)]
pub struct AnimatedGifImageBundle {
    pub animated_gif: Handle<AnimatedGif>,
    pub timer: AnimatedGifTimer,
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

#[derive(Bundle, Clone, Default)]
pub struct AnimatedGifTextureAtlasBundle {
    pub animated_gif: Handle<AnimatedGif>,
    pub timer: AnimatedGifTimer,
    pub texture_atlas: TextureAtlas,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
}
