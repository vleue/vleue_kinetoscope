use std::marker::PhantomData;

use bevy::prelude::*;
// use bevy_app::{App, Plugin};
// use bevy_asset::{AddAsset, AssetEvent, Assets, Handle, HandleId};
// use bevy_ecs::event::EventReader;
// use bevy_ecs::schedule::{IntoSystemConfig, IntoSystemSetConfigs};
// use bevy_ecs::system::Resource;
// use bevy_ecs::system::{Commands, Local, Res, ResMut, StaticSystemParam};
// use bevy_render::render_asset::{PrepareAssetError, RenderAsset};
// use bevy_render::texture::{GpuImage, Image};
// use bevy_render::{
//     render_asset::{ExtractedAssets, PrepareAssetSet, PrepareNextFrameAssets, RenderAssets},
//     Extract, ExtractSchedule, RenderApp, RenderSet,
// };
// use bevy_time::{Time, Timer, TimerMode};
// use bevy_utils::{tracing::info, Duration, HashMap, HashSet};
// use gif::driver::magic_driver;
use gif::AnimatedGifTimer;
use gif::{
    driver::image_driver,
    // driver::{image_driver, texture_atlas_driver},
    loader::AnimatedGifLoader,
    AnimatedGif,
};

mod gif;
pub use gif::{AnimatedGifImageBundle, AnimatedGifTextureAtlasBundle};

pub struct AnimatedGifPlugin {
    // prepare_asset_set: PrepareAssetSet,
    phantom: PhantomData<fn() -> AnimatedGif>,
}

impl Default for AnimatedGifPlugin {
    fn default() -> Self {
        Self {
            // prepare_asset_set: Default::default(),
            phantom: PhantomData,
        }
    }
}

impl Plugin for AnimatedGifPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AnimatedGif>()
            .init_asset_loader::<AnimatedGifLoader>()
            .add_systems(Update, image_driver);
        // .add_systems(Update, (texture_atlas_driver, image_driver));
        // app.add_system(magic_driver);
        // if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
        //     render_app
        //         .configure_sets(
        //             (
        //                 PrepareAssetSet::PreAssetPrepare,
        //                 PrepareAssetSet::AssetPrepare,
        //                 PrepareAssetSet::PostAssetPrepare,
        //             )
        //                 .chain()
        //                 .in_set(RenderSet::Prepare),
        //         )
        //         .init_resource::<ExtractedAssets2>()
        //         .init_resource::<RenderAssets<AnimatedGif>>()
        //         .init_resource::<PrepareNextFrameAssets2>()
        //         .add_system_to_schedule(ExtractSchedule, extract_render_asset)
        //         .add_system(prepare_assets.in_set(self.prepare_asset_set.clone()))
        //         .add_system(queue.in_set(RenderSet::Queue));
        // }
    }
}

// fn extract_render_asset(
//     mut commands: Commands,
//     mut events: Extract<EventReader<AssetEvent<AnimatedGif>>>,
//     assets: Extract<Res<Assets<AnimatedGif>>>,
//     images: Extract<Res<Assets<Image>>>,
//     time: Extract<Res<Time>>,
//     mut timers: Local<HashMap<HandleId, AnimatedGifTimer>>,
// ) {
//     // let mut changed_assets = HashSet::default();
//     let mut removed = Vec::new();
//     for event in events.iter() {
//         match event {
//             AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
//                 info!("new!");
//                 timers.insert(
//                     handle.id(),
//                     AnimatedGifTimer(
//                         Timer::new(Duration::from_millis(10), bevy_time::TimerMode::Repeating),
//                         usize::MAX,
//                     ),
//                 );
//                 // changed_assets.insert(handle.clone_weak());
//             }
//             AssetEvent::Removed { handle } => {
//                 timers.remove(&handle.id());
//                 // changed_assets.remove(handle);
//                 removed.push(Handle::weak(handle.id()));
//             }
//         }
//     }

//     let mut extracted_assets = Vec::new();
//     // for handle in changed_assets.drain() {
//     for (handle, asset) in assets.iter() {
//         // if let Some(asset) = assets.get(&handle) {
//         let zut = timers.get_mut(&handle).unwrap();
//         if zut.1 == usize::MAX || zut.0.tick(time.delta()).just_finished() {
//             if zut.1 == usize::MAX {
//                 zut.1 = 0;
//             } else {
//                 zut.1 = (zut.1 + 1) % asset.frames.len();
//             }
//             zut.0 = Timer::new(
//                 Duration::from_millis(asset.frames[zut.1].delay.0 as u64),
//                 TimerMode::Repeating,
//             );
//             let image = asset.frames[zut.1].image.clone_weak();
//             info!("extracting image {} -> {:?}", zut.1, image);
//             let image = images.get(&image);
//             extracted_assets.push((Handle::weak(handle), image.unwrap().clone()));
//         }
//         // }
//     }

//     commands.insert_resource(ExtractedAssets2 {
//         extracted: extracted_assets,
//         removed,
//     });
// }

// pub fn prepare_assets(
//     mut extracted_assets: ResMut<ExtractedAssets2>,
//     mut render_assets: ResMut<RenderAssets<Image>>,
//     mut prepare_next_frame: ResMut<PrepareNextFrameAssets2>,
//     param: StaticSystemParam<<AnimatedGif as RenderAsset>::Param>,
// ) {
//     let mut param = param.into_inner();
//     let queued_assets = std::mem::take(&mut prepare_next_frame.assets);
//     for (handle, extracted_asset) in queued_assets {
//         match AnimatedGif::prepare_asset(extracted_asset, &mut param) {
//             Ok(prepared_asset) => {
//                 render_assets.insert(handle, prepared_asset);
//             }
//             Err(PrepareAssetError::RetryNextUpdate(extracted_asset)) => {
//                 prepare_next_frame.assets.push((handle, extracted_asset));
//             }
//         }
//     }

//     for removed in std::mem::take(&mut extracted_assets.removed) {
//         render_assets.remove(&removed);
//     }

//     for (handle, extracted_asset) in std::mem::take(&mut extracted_assets.extracted) {
//         match AnimatedGif::prepare_asset(extracted_asset, &mut param) {
//             Ok(prepared_asset) => {
//                 info!("prepared! {:?}", handle);
//                 render_assets.insert(handle, prepared_asset);
//             }
//             Err(PrepareAssetError::RetryNextUpdate(extracted_asset)) => {
//                 prepare_next_frame.assets.push((handle, extracted_asset));
//             }
//         }
//     }
// }

// #[derive(Resource)]
// pub struct PrepareNextFrameAssets2 {
//     assets: Vec<(Handle<Image>, Image)>,
// }

// impl Default for PrepareNextFrameAssets2 {
//     fn default() -> Self {
//         Self {
//             assets: Default::default(),
//         }
//     }
// }

// #[derive(Resource)]
// pub struct ExtractedAssets2 {
//     extracted: Vec<(Handle<Image>, Image)>,
//     removed: Vec<Handle<Image>>,
// }

// impl Default for ExtractedAssets2 {
//     fn default() -> Self {
//         Self {
//             extracted: Default::default(),
//             removed: Default::default(),
//         }
//     }
// }

// pub fn queue(
//     mut commands: Commands,
//     mut view_entities: Local<FixedBitSet>,
//     draw_functions: Res<DrawFunctions<Transparent2d>>,
//     render_device: Res<RenderDevice>,
//     render_queue: Res<RenderQueue>,
//     mut sprite_meta: ResMut<SpriteMeta>,
//     view_uniforms: Res<ViewUniforms>,
//     sprite_pipeline: Res<SpritePipeline>,
//     mut pipelines: ResMut<SpecializedRenderPipelines<SpritePipeline>>,
//     pipeline_cache: Res<PipelineCache>,
//     mut image_bind_groups: ResMut<ImageBindGroups>,
//     gpu_images: Res<RenderAssets<Image>>,
//     msaa: Res<Msaa>,
//     mut extracted_sprites: ResMut<ExtractedSprites>,
//     mut views: Query<(
//         &mut RenderPhase<Transparent2d>,
//         &VisibleEntities,
//         &ExtractedView,
//         Option<&Tonemapping>,
//         Option<&DebandDither>,
//     )>,
//     events: Res<SpriteAssetEvents>,
// ) {
//     // If an image has changed, the GpuImage has (probably) changed
//     for event in &events.images {
//         match event {
//             AssetEvent::Created { .. } => None,
//             AssetEvent::Modified { handle } | AssetEvent::Removed { handle } => {
//                 image_bind_groups.values.remove(handle)
//             }
//         };
//     }

//     let msaa_key = SpritePipelineKey::from_msaa_samples(msaa.samples());

//     if let Some(view_binding) = view_uniforms.uniforms.binding() {
//         let sprite_meta = &mut sprite_meta;

//         // Clear the vertex buffers
//         sprite_meta.vertices.clear();
//         sprite_meta.colored_vertices.clear();

//         sprite_meta.view_bind_group = Some(render_device.create_bind_group(&BindGroupDescriptor {
//             entries: &[BindGroupEntry {
//                 binding: 0,
//                 resource: view_binding,
//             }],
//             label: Some("sprite_view_bind_group"),
//             layout: &sprite_pipeline.view_layout,
//         }));

//         let draw_sprite_function = draw_functions.read().id::<DrawSprite>();

//         // Vertex buffer indices
//         let mut index = 0;
//         let mut colored_index = 0;

//         // FIXME: VisibleEntities is ignored

//         let extracted_sprites = &mut extracted_sprites.sprites;
//         // Sort sprites by z for correct transparency and then by handle to improve batching
//         // NOTE: This can be done independent of views by reasonably assuming that all 2D views look along the negative-z axis in world space
//         extracted_sprites.sort_unstable_by(|a, b| {
//             match a
//                 .transform
//                 .translation()
//                 .z
//                 .partial_cmp(&b.transform.translation().z)
//             {
//                 Some(Ordering::Equal) | None => a.image_handle_id.cmp(&b.image_handle_id),
//                 Some(other) => other,
//             }
//         });
//         let image_bind_groups = &mut *image_bind_groups;

//         for (mut transparent_phase, visible_entities, view, tonemapping, dither) in &mut views {
//             let mut view_key = SpritePipelineKey::from_hdr(view.hdr) | msaa_key;

//             if !view.hdr {
//                 if let Some(tonemapping) = tonemapping {
//                     view_key |= SpritePipelineKey::TONEMAP_IN_SHADER;
//                     view_key |= match tonemapping {
//                         Tonemapping::None => SpritePipelineKey::TONEMAP_METHOD_NONE,
//                         Tonemapping::Reinhard => SpritePipelineKey::TONEMAP_METHOD_REINHARD,
//                         Tonemapping::ReinhardLuminance => {
//                             SpritePipelineKey::TONEMAP_METHOD_REINHARD_LUMINANCE
//                         }
//                         Tonemapping::AcesFitted => SpritePipelineKey::TONEMAP_METHOD_ACES_FITTED,
//                         Tonemapping::AgX => SpritePipelineKey::TONEMAP_METHOD_AGX,
//                         Tonemapping::SomewhatBoringDisplayTransform => {
//                             SpritePipelineKey::TONEMAP_METHOD_SOMEWHAT_BORING_DISPLAY_TRANSFORM
//                         }
//                         Tonemapping::TonyMcMapface => {
//                             SpritePipelineKey::TONEMAP_METHOD_TONY_MC_MAPFACE
//                         }
//                         Tonemapping::BlenderFilmic => {
//                             SpritePipelineKey::TONEMAP_METHOD_BLENDER_FILMIC
//                         }
//                     };
//                 }
//                 if let Some(DebandDither::Enabled) = dither {
//                     view_key |= SpritePipelineKey::DEBAND_DITHER;
//                 }
//             }

//             let pipeline = pipelines.specialize(
//                 &pipeline_cache,
//                 &sprite_pipeline,
//                 view_key | SpritePipelineKey::from_colored(false),
//             );
//             let colored_pipeline = pipelines.specialize(
//                 &pipeline_cache,
//                 &sprite_pipeline,
//                 view_key | SpritePipelineKey::from_colored(true),
//             );

//             view_entities.clear();
//             view_entities.extend(visible_entities.entities.iter().map(|e| e.index() as usize));
//             transparent_phase.items.reserve(extracted_sprites.len());

//             // Impossible starting values that will be replaced on the first iteration
//             let mut current_batch = SpriteBatch {
//                 image_handle_id: HandleId::Id(Uuid::nil(), u64::MAX),
//                 colored: false,
//             };
//             let mut current_batch_entity = Entity::PLACEHOLDER;
//             let mut current_image_size = Vec2::ZERO;
//             // Add a phase item for each sprite, and detect when successive items can be batched.
//             // Spawn an entity with a `SpriteBatch` component for each possible batch.
//             // Compatible items share the same entity.
//             // Batches are merged later (in `batch_phase_system()`), so that they can be interrupted
//             // by any other phase item (and they can interrupt other items from batching).
//             for extracted_sprite in extracted_sprites.iter() {
//                 if !view_entities.contains(extracted_sprite.entity.index() as usize) {
//                     continue;
//                 }
//                 let new_batch = SpriteBatch {
//                     image_handle_id: extracted_sprite.image_handle_id,
//                     colored: extracted_sprite.color != Color::WHITE,
//                 };
//                 if new_batch != current_batch {
//                     // Set-up a new possible batch
//                     if let Some(gpu_image) =
//                         gpu_images.get(&Handle::weak(new_batch.image_handle_id))
//                     {
//                         current_batch = new_batch;
//                         current_image_size = Vec2::new(gpu_image.size.x, gpu_image.size.y);
//                         current_batch_entity = commands.spawn(current_batch).id();

//                         image_bind_groups
//                             .values
//                             .entry(Handle::weak(current_batch.image_handle_id))
//                             .or_insert_with(|| {
//                                 render_device.create_bind_group(&BindGroupDescriptor {
//                                     entries: &[
//                                         BindGroupEntry {
//                                             binding: 0,
//                                             resource: BindingResource::TextureView(
//                                                 &gpu_image.texture_view,
//                                             ),
//                                         },
//                                         BindGroupEntry {
//                                             binding: 1,
//                                             resource: BindingResource::Sampler(&gpu_image.sampler),
//                                         },
//                                     ],
//                                     label: Some("sprite_material_bind_group"),
//                                     layout: &sprite_pipeline.material_layout,
//                                 })
//                             });
//                     } else {
//                         // Skip this item if the texture is not ready
//                         continue;
//                     }
//                 }

//                 // Calculate vertex data for this item

//                 let mut uvs = QUAD_UVS;
//                 if extracted_sprite.flip_x {
//                     uvs = [uvs[1], uvs[0], uvs[3], uvs[2]];
//                 }
//                 if extracted_sprite.flip_y {
//                     uvs = [uvs[3], uvs[2], uvs[1], uvs[0]];
//                 }

//                 // By default, the size of the quad is the size of the texture
//                 let mut quad_size = current_image_size;

//                 // If a rect is specified, adjust UVs and the size of the quad
//                 if let Some(rect) = extracted_sprite.rect {
//                     let rect_size = rect.size();
//                     for uv in &mut uvs {
//                         *uv = (rect.min + *uv * rect_size) / current_image_size;
//                     }
//                     quad_size = rect_size;
//                 }

//                 // Override the size if a custom one is specified
//                 if let Some(custom_size) = extracted_sprite.custom_size {
//                     quad_size = custom_size;
//                 }

//                 // Apply size and global transform
//                 let positions = QUAD_VERTEX_POSITIONS.map(|quad_pos| {
//                     extracted_sprite
//                         .transform
//                         .transform_point(
//                             ((quad_pos - extracted_sprite.anchor) * quad_size).extend(0.),
//                         )
//                         .into()
//                 });

//                 // These items will be sorted by depth with other phase items
//                 let sort_key = FloatOrd(extracted_sprite.transform.translation().z);

//                 // Store the vertex data and add the item to the render phase
//                 if current_batch.colored {
//                     let vertex_color = extracted_sprite.color.as_linear_rgba_f32();
//                     for i in QUAD_INDICES {
//                         sprite_meta.colored_vertices.push(ColoredSpriteVertex {
//                             position: positions[i],
//                             uv: uvs[i].into(),
//                             color: vertex_color,
//                         });
//                     }
//                     let item_start = colored_index;
//                     colored_index += QUAD_INDICES.len() as u32;
//                     let item_end = colored_index;

//                     transparent_phase.add(Transparent2d {
//                         draw_function: draw_sprite_function,
//                         pipeline: colored_pipeline,
//                         entity: current_batch_entity,
//                         sort_key,
//                         batch_range: Some(item_start..item_end),
//                     });
//                 } else {
//                     for i in QUAD_INDICES {
//                         sprite_meta.vertices.push(SpriteVertex {
//                             position: positions[i],
//                             uv: uvs[i].into(),
//                         });
//                     }
//                     let item_start = index;
//                     index += QUAD_INDICES.len() as u32;
//                     let item_end = index;

//                     transparent_phase.add(Transparent2d {
//                         draw_function: draw_sprite_function,
//                         pipeline,
//                         entity: current_batch_entity,
//                         sort_key,
//                         batch_range: Some(item_start..item_end),
//                     });
//                 }
//             }
//         }
//         sprite_meta
//             .vertices
//             .write_buffer(&render_device, &render_queue);
//         sprite_meta
//             .colored_vertices
//             .write_buffer(&render_device, &render_queue);
//     }
// }
