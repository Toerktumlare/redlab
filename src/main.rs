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
    cube::{Cube, CubeTextures, TileCoords},
    main_camera::MainCameraPlugin,
};

mod cube;
mod main_camera;

#[derive(Resource, Default)]
struct Grid {
    map: HashMap<IVec3, Entity>,
}

#[derive(Component)]
struct BasicBlock;

#[derive(Component)]
struct HoverMarker;

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
        .add_plugins(MainCameraPlugin)
        .init_resource::<Grid>()
        .add_systems(Startup, startup)
        .add_systems(Update, draw_on_hover_arrow)
        .run();
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    mut grid: ResMut<Grid>,
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

    let grid_size = 5;
    let half_grid = grid_size / 2;

    let custom_texture_handle: Handle<Image> = asset_server.load("cube.png");

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
                        Some(TileCoords::new(0, 0)),
                        Some(TileCoords::new(1, 0)),
                        Some(TileCoords::new(2, 0)),
                        Some(TileCoords::new(3, 0)),
                        Some(TileCoords::new(4, 0)),
                        Some(TileCoords::new(5, 0)),
                    );
                    let mesh = Cube::new(textures);

                    info_once!("{mesh:#?}");
                    let entity = commands
                        .spawn((
                            Name::new("Basic"),
                            Mesh3d(meshes.add(mesh)),
                            MeshMaterial3d(materials.add(StandardMaterial {
                                base_color_texture: Some(custom_texture_handle.clone()),
                                ..default()
                            })),
                            Transform::from_xyz(pos_x as f32, y as f32, pos_z as f32),
                            BasicBlock,
                            Pickable {
                                is_hoverable: true,
                                ..default()
                            },
                        ))
                        .observe(
                            |event: On<Pointer<Over>>,
                             query: Query<&Transform, With<BasicBlock>>,
                             mut meshes: ResMut<Assets<Mesh>>,
                             mut materials: ResMut<Assets<StandardMaterial>>,
                             mut commands: Commands| {
                                info!("On event triggered!");
                                let mesh_transform = query.get(event.event_target()).unwrap();
                                let mesh_center = mesh_transform.translation;
                                let point = mesh_center + event.hit.normal.unwrap() * 0.501;
                                let rotation =
                                    Quat::from_rotation_arc(Vec3::Y, event.hit.normal.unwrap());
                                commands.spawn((
                                    Mesh3d(meshes.add(Cuboid::new(0.9, 0.01, 0.9))),
                                    MeshMaterial3d(materials.add(Color::srgb(0.7, 0.4, 0.8))),
                                    Transform::from_translation(point).with_rotation(rotation),
                                    HoverMarker,
                                ));
                            },
                        )
                        .observe(
                            |_event: On<Pointer<Out>>,
                             query: Query<Entity, With<HoverMarker>>,
                             mut commands: Commands| {
                                if let Some(entity) = query.iter().next() {
                                    commands.entity(entity).despawn();
                                }
                            },
                        )
                        .id();
                    grid.map.insert(IVec3::new(pos_x, y, pos_z), entity);
                } else {
                    let u = rng.next_u32();
                    let should_spawn = u < (u32::MAX / 10);
                    if should_spawn {
                        let entity = commands
                            .spawn((
                                Name::new("TopRow"),
                                Mesh3d(meshes.add(Cuboid::default())),
                                MeshMaterial3d(materials.add(Color::srgb(r, g, b))),
                                Transform::from_xyz(pos_x as f32, y as f32, pos_z as f32),
                                BasicBlock,
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

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(6.0, 4.0, -3.0),
    ));
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
