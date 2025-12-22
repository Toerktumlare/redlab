use bevy::{
    color::palettes::tailwind::RED_500, picking::pointer::PointerInteraction, prelude::*,
    window::WindowResolution,
};
use bevy_prng::WyRand;
use bevy_rand::{plugin::EntropyPlugin, prelude::GlobalRng};
use rand_core::RngCore;
use std::collections::HashMap;

use bevy::color::palettes::css::GHOST_WHITE;

use crate::{
    block_interaction_plugin::BlockInteractionPlugin,
    block_lifecycle_plugin::BlockLifecyclePlugin,
    block_selection_plugin::{
        BlockSelectionPlugin, track_grid_cordinate, track_hovered_block, untrack_hovered_block,
    },
    cube::{Cube, CubeTextures, TileCoords},
    main_camera::MainCameraPlugin,
};

mod block_interaction_plugin;
mod block_lifecycle_plugin;
mod block_selection_plugin;
mod cube;
mod main_camera;
mod pixel_picking_plugin;

#[derive(Resource, Default)]
struct Grid {
    map: HashMap<IVec3, Entity>,
}

#[derive(Component)]
struct PreviewBlock;

#[derive(Component)]
struct HoveredBlock;

#[derive(Resource, Default)]
struct HoveredBlockPosition {
    position: Option<IVec3>,
    normal: Option<Vec3>,
}

#[derive(Resource, Default)]
struct SelectedBlock(Option<Block>);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Block {
    StandardGrass,
    RedStone,
    Dust,
    RedStoneLamp,
}

#[derive(Resource, Default)]
struct Textures {
    handles: HashMap<Block, Handle<Image>>,
}

#[derive(Event)]
struct PlaceBlockRequestEvent;

fn main() {
    App::new()
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
        .add_plugins((
            MainCameraPlugin,
            BlockSelectionPlugin,
            BlockInteractionPlugin,
            BlockLifecyclePlugin,
        ))
        .init_resource::<Grid>()
        .init_resource::<Textures>()
        .init_resource::<HoveredBlockPosition>()
        .init_resource::<SelectedBlock>()
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                select_block,
                request_place_selected_block,
                update_preview_block.run_if(
                    resource_changed::<SelectedBlock>.and(not(resource_added::<SelectedBlock>)),
                ),
                draw_on_hover_arrow,
                hover_block_visibility_system,
            ),
        )
        .run();
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    mut grid: ResMut<Grid>,
    mut textures: ResMut<Textures>,
) {
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

    let grid_size = 4;
    let half_grid = grid_size / 2;

    let grass_texture: Handle<Image> = asset_server.load("cube-sheet.png");
    textures.handles.insert(Block::StandardGrass, grass_texture);
    let grass_texture = textures.handles.get(&Block::StandardGrass);

    for x in 0..grid_size {
        for z in 0..grid_size {
            for y in 0..2 {
                let r = random_f32(&mut rng);
                let g = random_f32(&mut rng);
                let b = random_f32(&mut rng);

                let pos_x = x - half_grid;
                let pos_z = z - half_grid;
                if y == 0 {
                    let textures = CubeTextures::new(
                        Some(TileCoords::new(0, 1)),
                        Some(TileCoords::new(1, 1)),
                        Some(TileCoords::new(2, 1)),
                        Some(TileCoords::new(3, 1)),
                        Some(TileCoords::new(4, 1)),
                        Some(TileCoords::new(5, 1)),
                    );
                    let mesh = Cube::new(textures);

                    info_once!("{mesh:#?}");
                    let entity = commands
                        .spawn((
                            Mesh3d(meshes.add(mesh)),
                            MeshMaterial3d(materials.add(StandardMaterial {
                                base_color_texture: grass_texture.cloned(),
                                perceptual_roughness: 1.0,
                                ..default()
                            })),
                            Transform::from_xyz(pos_x as f32, y as f32, pos_z as f32),
                            Pickable {
                                is_hoverable: true,
                                ..default()
                            },
                        ))
                        .observe(track_hovered_block)
                        .observe(track_grid_cordinate)
                        .observe(untrack_hovered_block)
                        .id();
                    grid.map.insert(IVec3::new(pos_x, y, pos_z), entity);
                } else {
                    let u = rng.next_u32();
                    let should_spawn = u < (u32::MAX / 10);
                    if false {
                        let entity = commands
                            .spawn((
                                Name::new("TopRow"),
                                Mesh3d(meshes.add(Cuboid::default())),
                                MeshMaterial3d(materials.add(Color::srgb(r, g, b))),
                                Transform::from_xyz(pos_x as f32, y as f32, pos_z as f32),
                                Pickable {
                                    is_hoverable: true,
                                    ..default()
                                },
                            ))
                            .id();
                        grid.map.insert(IVec3::new(pos_x, y, pos_z), entity);
                    }
                }
            }
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

fn spawn_selection(
    event: On<Pointer<Over>>,
    mut commands: Commands,
    hover_entity: Query<Entity, With<PreviewBlock>>,
    mut selected_block: ResMut<SelectedBlock>,
) {
    if hover_entity.single().is_err() && selected_block.0.is_some() {
        selected_block.set_changed();
    }
    let self_entity = event.event_target();
    commands.entity(self_entity).insert(HoveredBlock);
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
        if selected_block.0 == Some(Block::StandardGrass) {
            info!("Deselecting Grass");
            selected_block.0 = None;
        } else {
            info!("Selecting Grass");
            selected_block.0 = Some(Block::StandardGrass);
        }
    }
    if key_input.just_pressed(KeyCode::Digit2) {
        if selected_block.0 == Some(Block::RedStone) {
            info!("Deselecting RedStone");
            selected_block.0 = None;
        } else {
            info!("Selecting RedStone");
            selected_block.0 = Some(Block::RedStone);
        }
    }
    if key_input.just_pressed(KeyCode::Digit3) {
        if selected_block.0 == Some(Block::RedStoneLamp) {
            info!("Deselecting RedStone");
            selected_block.0 = None;
        } else {
            info!("Selecting RedStone");
            selected_block.0 = Some(Block::RedStoneLamp);
        }
    }
}

fn request_place_selected_block(
    mut commands: Commands,
    selected_block: Res<SelectedBlock>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) && selected_block.0.is_some() {
        commands.trigger(PlaceBlockRequestEvent);
    }
}

fn update_preview_block(
    mut commands: Commands,
    selected_block: Res<SelectedBlock>,
    query: Query<&Transform>,
    preview_block: Query<Entity, With<PreviewBlock>>,
    textures: Res<Textures>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    pointers: Query<&PointerInteraction>,
) {
    let id = match selected_block.0 {
        Some(Block::StandardGrass) => {
            let mesh = Cube::get(Block::StandardGrass);
            let grass_texture = textures.handles.get(&Block::StandardGrass);
            Some(
                commands
                    .spawn((
                        Mesh3d(meshes.add(mesh)),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color_texture: grass_texture.cloned(),
                            perceptual_roughness: 1.0,
                            ..default()
                        })),
                        Visibility::Hidden,
                        PreviewBlock,
                    ))
                    .id(),
            )
        }
        None => {
            if let Ok(entity) = preview_block.single() {
                commands.entity(entity).despawn();
            }
            None
        }
        _ => None,
    };

    if let Some(id) = id {
        let point = pointers
            .iter()
            .filter_map(|interactions| interactions.get_nearest_hit())
            .find_map(|(entity, hit)| {
                let normal = hit.normal?;
                let transform = query.get(*entity).ok()?;
                Some(transform.translation + normal * 1.001)
            })
            .unwrap_or(Vec3::ZERO);

        commands
            .entity(id)
            .insert(Transform::from_translation(point));
    }
}

fn hover_block_visibility_system(
    mut query: Query<&mut Visibility, With<PreviewBlock>>,
    hovered_block: Query<&HoveredBlock>,
) {
    if let Ok(mut visibility) = query.single_mut() {
        if !hovered_block.is_empty() {
            *visibility = Visibility::Inherited;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
