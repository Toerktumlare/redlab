use bevy::{ecs::system::SystemParam, prelude::*, window::WindowResolution};
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;
use std::{collections::HashMap, time::Duration};

use bevy::color::palettes::css::GHOST_WHITE;

use crate::{
    blocks::{BlockType, NeighbourUpdate, StandardGrass},
    grid_plugin::{BlockChange, Grid, GridPlugin, Place, grid_apply_changes, queue_block_change},
    interactions::BlockInteractionPlugin,
    main_camera::MainCameraPlugin,
    materials::redstone::{RedstoneColors, RedstoneMaterials, setup_redstone_materials},
    meshes::{MeshRegistry, setup_mesh_registry},
    redstone::{
        GlobalTick, Scheduler,
        ticks::{GlobalTickEvent, tick_the_counter},
    },
    render::{
        BlockEntities, DirtyRender, RenderPlugin, cleanup, debug_info, hovered_block, renderer,
        scheduler_info,
    },
    shaders::block::BlockMaterial,
    systems::recalculate_dirty_blocks,
    ui::debug_view_system,
};

// mod block_texture_updater;
mod block_position;
mod blocks;
mod grid_plugin;
mod interactions;
mod main_camera;
mod materials;
mod meshes;
mod pixel_picking_plugin;
mod redstone;
mod render;
mod shaders;
mod systems;
mod ui;

#[derive(Debug)]
pub struct BlockData {
    block_type: BlockType,
}

impl Default for BlockData {
    fn default() -> Self {
        Self {
            block_type: BlockType::Air,
        }
    }
}

#[derive(Resource, Default)]
struct SelectedBlock(Option<BlockType>);

#[derive(Resource, Default)]
pub struct Textures {
    handles: HashMap<TextureAtlas, Handle<Image>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TextureAtlas {
    Blocks,
}

#[derive(SystemParam)]
pub struct SpawnCtx<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
    pub atlas: Res<'w, Textures>,
    pub grid: Res<'w, Grid>,
    pub mesh_registry: Res<'w, MeshRegistry>,
}

#[derive(SystemParam)]
pub struct RenderCtx<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
    pub redstone_materials: ResMut<'w, RedstoneMaterials>,
    pub atlas: Res<'w, Textures>,
    pub grid: Res<'w, Grid>,
    pub mesh_registry: Res<'w, MeshRegistry>,
    pub block_entities: ResMut<'w, BlockEntities>,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameLoop {
    Input,
    Apply,
    React,
    Cleanup,
    Render,
}

fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(100)))
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "RedLab".to_string(),
                        resolution: WindowResolution::new(1280, 720)
                            .with_scale_factor_override(1.0),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(MaterialPlugin::<BlockMaterial>::default())
        .add_plugins((
            GridPlugin,
            MainCameraPlugin,
            BlockInteractionPlugin,
            RenderPlugin,
        ))
        .init_resource::<Textures>()
        .init_resource::<SelectedBlock>()
        .init_resource::<GlobalTick>()
        .init_resource::<DirtyRender>()
        .init_resource::<RedstoneColors>()
        .init_resource::<Scheduler>()
        .add_message::<GlobalTickEvent>()
        .add_systems(
            Startup,
            (
                startup,
                setup_mesh_registry,
                setup_redstone_materials,
                debug_view_system,
            )
                .chain(),
        )
        .add_systems(FixedUpdate, (tick_the_counter).chain())
        .add_systems(Update, grid_apply_changes.in_set(GameLoop::Apply))
        .add_systems(Update, (recalculate_dirty_blocks,).in_set(GameLoop::React))
        .add_systems(
            Update,
            (renderer, debug_info, hovered_block, scheduler_info, cleanup)
                .chain()
                .in_set(GameLoop::Render),
        )
        .configure_sets(
            Update,
            (
                GameLoop::Input,
                GameLoop::Apply,
                GameLoop::React,
                GameLoop::Cleanup,
                GameLoop::Render,
            )
                .chain(),
        )
        .add_observer(queue_block_change)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut textures: ResMut<Textures>) {
    let fonts = asset_server.load("fonts/retro_gaming.ttf");

    commands.spawn((
        Text::new("(1) Grass  (2) Redstone  (3) Lamp  (4) Dust  (5) Torch    (Space) Center Camera  (Tab) Run/Pause  (R) Reset"),
        TextFont {
            font: fonts.clone(),
            font_size: 17.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            bottom: px(5),
            left: px(15),
            ..default()
        },
        TextColor(GHOST_WHITE.into())
    ));

    let grid_size = 5;
    let half_grid = grid_size / 2;

    let blocks: Handle<Image> = asset_server.load("cube-sheet.png");
    textures.handles.insert(TextureAtlas::Blocks, blocks);

    for x in 0..grid_size {
        for z in 0..grid_size {
            let pos_x = x - half_grid;
            let pos_z = z - half_grid;
            let position = IVec3::new(pos_x, 0, pos_z);

            // TODO: recalculate block here before insertion
            commands.trigger(BlockChange::Place(Place::new(
                Some(BlockType::StandardGrass(StandardGrass::default())),
                position,
                true,
                None,
                NeighbourUpdate::NONE.to_vec(),
            )));
        }
    }

    commands.insert_resource(AmbientLight {
        brightness: 500.0,
        ..default()
    });
}

// fn random_f32(rng: &mut WyRand) -> f32 {
//     let v = rng.next_u32(); // 0 ..= u32::MAX
//     (v as f32) / (u32::MAX as f32) // 0.0 .. 1.0
// }
