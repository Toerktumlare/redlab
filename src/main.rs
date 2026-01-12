use bevy::{
    color::palettes::tailwind::RED_500, ecs::system::SystemParam,
    picking::pointer::PointerInteraction, prelude::*, window::WindowResolution,
};
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;
use std::{collections::HashMap, time::Duration};

use bevy::color::palettes::css::GHOST_WHITE;

use crate::{
    block_interaction_plugin::{
        BlockInteractionPlugin, request_delete_hovered_block, request_place_selected_block,
    },
    block_selection_plugin::BlockSelectionPlugin,
    block_texture_updater::grass_to_dirt_updater,
    blocks::{BlockType, Power},
    grid_plugin::{
        BlockChange, Grid, GridPlugin, PlaceRequest, grid_apply_changes, queue_block_change,
    },
    main_camera::MainCameraPlugin,
    materials::redstone::{RedstoneColors, setup_redstone_materials},
    meshes::{MeshRegistry, setup_mesh_registry},
    redstone::{
        GlobalTick,
        button_system::button_tick_system,
        ticks::{GlobalTickEvent, tick_the_counter},
    },
    redstone_connection_plugin::{JunctionType, update_redstone_system},
    render::{
        RenderPlugin, block_renderer::render_blocks, debug::render_debug_info,
        redstone_renderer::render_redstone,
    },
    shaders::block::BlockMaterial,
    systems::propagate_redstone::{
        propagate_block_power, propagate_dust_power, propagate_strong_power, propagate_torch_power,
        update_redstone_lamps,
    },
};

mod block_interaction_plugin;
mod block_selection_plugin;
mod block_texture_updater;
mod blocks;
mod grid_plugin;
mod main_camera;
mod materials;
mod meshes;
mod pixel_picking_plugin;
mod redstone;
mod redstone_connection_plugin;
mod render;
mod shaders;
mod systems;

#[derive(Component)]
pub struct TickText;

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
            BlockSelectionPlugin,
            BlockInteractionPlugin,
            RenderPlugin,
        ))
        .init_resource::<Textures>()
        .init_resource::<SelectedBlock>()
        .init_resource::<RedstoneColors>()
        .init_resource::<GlobalTick>()
        .add_message::<GlobalTickEvent>()
        .add_systems(
            Startup,
            (startup, setup_redstone_materials, setup_mesh_registry).chain(),
        )
        .add_systems(FixedUpdate, tick_the_counter)
        .add_systems(Update, (draw_on_hover_arrow, select_block))
        .add_systems(
            Update,
            (request_place_selected_block, request_delete_hovered_block).in_set(GameLoop::Input),
        )
        .add_systems(Update, grid_apply_changes.in_set(GameLoop::Apply))
        .add_systems(Update, grass_to_dirt_updater.in_set(GameLoop::React))
        .add_systems(
            Update,
            (
                update_redstone_system,
                propagate_strong_power,
                propagate_torch_power,
                propagate_block_power,
                propagate_dust_power,
                button_tick_system,
            )
                .in_set(GameLoop::React),
        )
        .add_systems(
            Update,
            (
                render_blocks,
                render_redstone,
                update_redstone_lamps,
                render_debug_info,
            )
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
    commands.spawn((
        Text::new("Ticks: "),
        TextFont {
            font: asset_server.load("fonts/retro_gaming.ttf"),
            font_size: 17.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: px(10),
            left: px(15),
            ..default()
        },
        TextColor(GHOST_WHITE.into()),
        children![(TextSpan::default(), TickText)],
    ));

    commands.spawn((
        Text::new("(1) Grass  (2) Redstone  (3) Lamp  (4) Dust  (5) Torch    (Space) Center Camera  (Tab) Run/Pause  (R) Reset"),
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

    let grid_size = 4;
    let half_grid = grid_size / 2;

    let blocks: Handle<Image> = asset_server.load("cube-sheet.png");
    textures.handles.insert(TextureAtlas::Blocks, blocks);

    for x in 0..grid_size {
        for z in 0..grid_size {
            let pos_x = x - half_grid;
            let pos_z = z - half_grid;
            let position = IVec3::new(pos_x, 0, pos_z);
            commands.trigger(BlockChange::Place(PlaceRequest {
                block_type: BlockType::StandardGrass {
                    power: Power::default(),
                },
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

// fn random_f32(rng: &mut WyRand) -> f32 {
//     let v = rng.next_u32(); // 0 ..= u32::MAX
//     (v as f32) / (u32::MAX as f32) // 0.0 .. 1.0
// }

fn draw_on_hover_arrow(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
    for (point, normal) in pointers
        .iter()
        .filter_map(|interactions| interactions.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
    {
        gizmos.arrow(point, point + normal.normalize() * 0.5, RED_500);
    }
}

fn select_block(
    key_input: Res<ButtonInput<KeyCode>>,
    mut selected_block: ResMut<SelectedBlock>,
    mut tick_counter: ResMut<GlobalTick>,
) {
    if key_input.just_pressed(KeyCode::Digit1) {
        if let Some(BlockType::StandardGrass { .. }) = selected_block.0 {
            info!("Deselecting Grass");
            selected_block.0 = None;
        } else {
            info!("Selecting Grass");
            selected_block.0 = Some(BlockType::StandardGrass {
                power: Power::default(),
            });
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
                power: Power::default(),
            });
        }
    }

    if key_input.just_pressed(KeyCode::Digit5) {
        if let Some(BlockType::RedStoneTorch { .. }) = selected_block.0 {
            info!("Deselecting RedStone Torch");
            selected_block.0 = None;
        } else {
            info!("Selecting RedStoneTorch");
            selected_block.0 = Some(BlockType::RedStoneTorch {
                on: true,
                attached_face: IVec3::NEG_X,
            });
        }
    }

    if key_input.just_pressed(KeyCode::Digit6) {
        if let Some(BlockType::StoneButton { .. }) = selected_block.0 {
            info!("Deselecting Stone Button");
            selected_block.0 = None;
        } else {
            info!("Selecting Stone Button");
            selected_block.0 = Some(BlockType::StoneButton {
                ticks: 10,
                pressed: false,
                attached_face: IVec3::NEG_X,
            });
        }
    }

    if key_input.just_pressed(KeyCode::Tab) {
        if tick_counter.is_running() {
            tick_counter.stop();
        } else {
            tick_counter.start();
        }
    }
}
