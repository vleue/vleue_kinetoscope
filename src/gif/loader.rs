use std::io::BufReader;

use bevy::asset::io::Reader;
use bevy::render::texture::TextureFormatPixelInfo;
use bevy::utils::{BoxedFuture, HashMap};
use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
// use bevy_asset::{AssetLoader, BoxedFuture, Handle, LoadContext, LoadedAsset};
// use bevy_math::{Rect, Vec2};
// use bevy_render::{
//     render_resource::{Extent3d, TextureDimension, TextureFormat},
//     texture::{Image, TextureFormatPixelInfo},
// };
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
// use bevy_sprite::TextureAtlas;
// use bevy_utils::{
//     tracing::{debug, error},
//     HashMap,
// };
use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext, LoadedAsset};

use image::{codecs::gif::GifDecoder, AnimationDecoder, DynamicImage};
use rectangle_pack::{
    contains_smallest_box, pack_rects, volume_heuristic, GroupedRectsToPlace, PackedLocation,
    RectToInsert, TargetBin,
};

use super::{AnimatedGif, Frame};

#[derive(Default)]
pub struct AnimatedGifLoader;

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
            // let a = BufReader::new(bytes.as_slice());
            let decoder = GifDecoder::new(bytes.as_slice()).unwrap();
            let frames = decoder.into_frames();
            let frames_from_gif = frames.collect_frames().expect("error decoding gif");

            // let mut images = std::collections::HashMap::default();
            let mut i = 0;
            // let mut images = vec![];
            let mut frames = vec![];
            // let mut rects_to_place: GroupedRectsToPlace<Handle<Image>> = GroupedRectsToPlace::new();
            // let mut images_ordered = vec![];
            // let mut sub = load_context.begin_labeled_asset();
            for frame in frames_from_gif.iter() {
                let image = Image::from_dynamic(
                    DynamicImage::ImageRgba8(frame.buffer().clone()),
                    true,
                    RenderAssetUsages::all(),
                );
                let handle = load_context.add_labeled_asset(format!("frame{}", i), image);
                i += 1;
                // let handle = load_context.set_labeled_asset(
                //     &format!("frame{}", images.len()),
                //     LoadedAsset::new(image.clone()),
                // );
                // load_context.load(image);
                // load_context.add_labeled_asset(label, asset)

                // images.insert(handle.clone(), image.clone());
                frames.push(Frame {
                    delay: frame.delay().numer_denom_ms(),
                    image: handle.clone(),
                });
                // images_ordered.push(handle.clone());
                // rects_to_place.push_rect(
                //     handle,
                //     None,
                //     RectToInsert::new(
                //         image.texture_descriptor.size.width,
                //         image.texture_descriptor.size.height,
                //         1,
                //     ),
                // );
            }
            // load_context.
            // sub.finish(value, None)
            // let atlas = build_atlas(rects_to_place, &images, images_ordered, load_context);

            // let atlas_handle = load_context.set_labeled_asset("atlas", LoadedAsset::new(atlas));

            // load_context.set_default_asset(LoadedAsset::new(AnimatedGif {
            //     frames,
            //     atlas: atlas_handle,
            // }));

            // Ok(())
            Ok(AnimatedGif {
                frames,
                // atlas: atlas_handle,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["gif"]
    }
}

// fn build_atlas(
//     rects_to_place: GroupedRectsToPlace<Handle<Image>>,
//     images: &HashMap<Handle<Image>, Image>,
//     images_ordered: Vec<Handle<Image>>,
//     load_context: &mut LoadContext,
// ) -> TextureAtlas {
//     let mut atlas_texture = Image::default();
//     let mut current_width = atlas_texture.size().x as u32;
//     let mut current_height = atlas_texture.size().y as u32;
//     let mut rect_placements = None;

//     let format = TextureFormat::Rgba8UnormSrgb;

//     while rect_placements.is_none() {
//         let mut target_bins = std::collections::BTreeMap::new();
//         target_bins.insert(0, TargetBin::new(current_width, current_height, 1));
//         rect_placements = match pack_rects(
//             &rects_to_place,
//             &mut target_bins,
//             &volume_heuristic,
//             &contains_smallest_box,
//         ) {
//             Ok(rect_placements) => {
//                 atlas_texture = Image::new(
//                     Extent3d {
//                         width: current_width,
//                         height: current_height,
//                         depth_or_array_layers: 1,
//                     },
//                     TextureDimension::D2,
//                     vec![0; format.pixel_size() * (current_width * current_height) as usize],
//                     format,
//                     RenderAssetUsages::all(),
//                 );
//                 Some(rect_placements)
//             }
//             Err(rectangle_pack::RectanglePackError::NotEnoughBinSpace) => {
//                 current_height *= 2;
//                 current_width *= 2;
//                 None
//             }
//         };
//     }

//     let rect_placements = rect_placements.unwrap();

//     let mut texture_rects = vec![
//         Rect {
//             min: Vec2::ZERO,
//             max: Vec2::ZERO
//         };
//         rect_placements.packed_locations().len()
//     ];

//     let mut texture_handles = HashMap::default();
//     for (texture_handle, (_, packed_location)) in rect_placements.packed_locations().iter() {
//         let texture = &images.get(texture_handle).unwrap();
//         let min = Vec2::new(packed_location.x() as f32, packed_location.y() as f32);
//         let max = min
//             + Vec2::new(
//                 packed_location.width() as f32,
//                 packed_location.height() as f32,
//             );
//         texture_handles.insert(texture_handle.clone_weak(), texture_rects.len());
//         let index = images_ordered
//             .iter()
//             .enumerate()
//             .find(|(_, h)| h == &texture_handle)
//             .unwrap()
//             .0;
//         texture_rects[index] = Rect { min, max };
//         copy_converted_texture(&mut atlas_texture, format, texture, packed_location);
//     }

//     let atlas_texture_handle =
//         load_context.set_labeled_asset("atlas-texture", LoadedAsset::new(atlas_texture.clone()));

//     TextureAtlasLayout {
//         size: Vec2::new(
//             atlas_texture.texture_descriptor.size.width as f32,
//             atlas_texture.texture_descriptor.size.height as f32,
//         ),
//         // texture: atlas_texture_handle,
//         textures: texture_rects,
//         texture_handles: Some(texture_handles),
//     }
// }

// fn copy_texture_to_atlas(
//     atlas_texture: &mut Image,
//     texture: &Image,
//     packed_location: &PackedLocation,
// ) {
//     let rect_width = packed_location.width() as usize;
//     let rect_height = packed_location.height() as usize;
//     let rect_x = packed_location.x() as usize;
//     let rect_y = packed_location.y() as usize;
//     let atlas_width = atlas_texture.texture_descriptor.size.width as usize;
//     let format_size = atlas_texture.texture_descriptor.format.pixel_size();

//     for (texture_y, bound_y) in (rect_y..rect_y + rect_height).enumerate() {
//         let begin = (bound_y * atlas_width + rect_x) * format_size;
//         let end = begin + rect_width * format_size;
//         let texture_begin = texture_y * rect_width * format_size;
//         let texture_end = texture_begin + rect_width * format_size;
//         atlas_texture.data[begin..end].copy_from_slice(&texture.data[texture_begin..texture_end]);
//     }
// }

// fn copy_converted_texture(
//     atlas_texture: &mut Image,
//     format: TextureFormat,
//     texture: &Image,
//     packed_location: &PackedLocation,
// ) {
//     if format == texture.texture_descriptor.format {
//         copy_texture_to_atlas(atlas_texture, texture, packed_location);
//     } else if let Some(converted_texture) = texture.convert(format) {
//         debug!(
//             "Converting texture from '{:?}' to '{:?}'",
//             texture.texture_descriptor.format, format
//         );
//         copy_texture_to_atlas(atlas_texture, &converted_texture, packed_location);
//     } else {
//         error!(
//             "Error converting texture from '{:?}' to '{:?}', ignoring",
//             texture.texture_descriptor.format, format
//         );
//     }
// }
