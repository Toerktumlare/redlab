use bevy::{
    color::palettes::tailwind::RED_500, ecs::system::SystemParam,
    picking::pointer::PointerInteraction, prelude::*, window::WindowResolution,
};
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;
use rand_core::RngCore;
use std::{collections::HashMap, time::Duration};

use bevy::color::palettes::css::GHOST_WHITE;

use crate::{
    block_interaction_plugin::{
        BlockInteractionPlugin, request_delete_hovered_block, request_place_selected_block,
    },
    block_lifecycle_plugin::{BlockLifecyclePlugin, draw},
    block_selection_plugin::BlockSelectionPlugin,
    block_texture_updater::grass_to_dirt_updater,
    grid_plugin::{BlockChange, GridPlugin, PlaceRequest, grid_apply_changes, queue_block_change},
    main_camera::MainCameraPlugin,
    redstone_connection_plugin::{JunctionType, update_redstone_system},
    shaders::block::BlockMaterial,
};

mod block_interaction_plugin;
mod block_lifecycle_plugin;
mod block_selection_plugin;
mod block_texture_updater;
mod cube;
mod grid_plugin;
mod main_camera;
mod pixel_picking_plugin;
mod redstone;
mod redstone_connection_plugin;
mod shaders;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum BlockType {
    Air,
    StandardGrass,
    Dirt,
    RedStone,
    RedStoneLamp { powered: bool },
    Dust { shape: JunctionType, power: u8 },
}

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
    pub command: Commands<'w, 's>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
    pub atlas: Res<'w, Textures>,
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
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(50)))
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
            BlockSelectionPlugin,
            BlockInteractionPlugin,
            BlockLifecyclePlugin,
        ))
        .init_resource::<Textures>()
        .init_resource::<SelectedBlock>()
        .add_systems(Startup, startup)
        .add_systems(Update, (draw_on_hover_arrow, select_block))
        .add_systems(
            Update,
            (request_place_selected_block, request_delete_hovered_block).in_set(GameLoop::Input),
        )
        .add_systems(Update, grid_apply_changes.in_set(GameLoop::Apply))
        .add_systems(Update, grass_to_dirt_updater.in_set(GameLoop::React))
        .add_systems(Update, update_redstone_system.in_set(GameLoop::React))
        .add_systems(Update, draw.in_set(GameLoop::Render))
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
    commands.spawn((
        Text::new("(1) Dust  (2) RedBlock  (3) Lamp  (4) Lever  (0) Erase   (Space) Play/Pause  (S) Step  (R) Reset  (C) Cycle camera"),
        TextFont {
            font: asset_server.load("fonts/retro_gaming.ttf"),
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
            commands.trigger(BlockChange::Place(PlaceRequest {
                block_type: BlockType::StandardGrass,
                normal: IVec3::ZERO,
                position,
            }));
        }
    }

    commands.insert_resource(AmbientLight {
        brightness: 500.0,
        ..default()
    });
}

fn random_f32(rng: &mut WyRand) -> f32 {
    let v = rng.next_u32(); // 0 ..= u32::MAX
    (v as f32) / (u32::MAX as f32) // 0.0 .. 1.0
}

fn draw_on_hover_arrow(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
    for (point, normal) in pointers
        .iter()
        .filter_map(|interactions| interactions.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
    {
        gizmos.arrow(point, point + normal.normalize() * 0.5, RED_500);
    }
}

fn select_block(key_input: Res<ButtonInput<KeyCode>>, mut selected_block: ResMut<SelectedBlock>) {
    if key_input.just_pressed(KeyCode::Digit1) {
        if let Some(BlockType::StandardGrass) = selected_block.0 {
            info!("Deselecting Grass");
            selected_block.0 = None;
        } else {
            info!("Selecting Grass");
            selected_block.0 = Some(BlockType::StandardGrass);
        }
    }
    if key_input.just_pressed(KeyCode::Digit2) {
        if let Some(BlockType::RedStone) = selected_block.0 {
            info!("Deselecting RedStone");
            selected_block.0 = None;
        } else {
            info!("Selecting RedStone");
            selected_block.0 = Some(BlockType::RedStone);
        }
    }
    if key_input.just_pressed(KeyCode::Digit3) {
        if let Some(BlockType::RedStoneLamp { .. }) = selected_block.0 {
            info!("Deselecting RedStoneLamp");
            selected_block.0 = None;
        } else {
            info!("Selecting RedStoneLamp");
            selected_block.0 = Some(BlockType::RedStoneLamp { powered: false });
        }
    }
    if key_input.just_pressed(KeyCode::Digit4) {
        if let Some(BlockType::Dust { .. }) = selected_block.0 {
            info!("Deselecting Dust");
            selected_block.0 = None;
        } else {
            info!("Selecting Dust");
            selected_block.0 = Some(BlockType::Dust {
                shape: JunctionType::Dot,
                power: 0,
            });
        }
    }
}
