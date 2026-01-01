use bevy::prelude::*;

use crate::block_selection_plugin::{
    track_grid_cordinate, track_hovered_block, untrack_hovered_block,
};

pub fn spawn_cross(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    texture: &Handle<Image>,
    position: IVec3,
) -> Entity {
    let vertical = get_mesh(JunctionUVs::Vertical);
    let horizontal = get_mesh(JunctionUVs::Horizontal);
    let dot = get_mesh(JunctionUVs::Dot);
    let mesh_array = [meshes.add(vertical), meshes.add(horizontal)];

    let dot_entity = spawn_redstone_mesh(dot, commands, materials, meshes, position, texture);

    for mesh in mesh_array {
        commands.entity(dot_entity).with_children(|parent| {
            parent.spawn((
                Name::new("Cross"),
                Mesh3d(mesh),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color_texture: Some(texture.clone()),
                    perceptual_roughness: 1.0,
                    emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                    base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    ..default()
                })),
            ));
        });
    }
    dot_entity
}

pub fn spawn_tcross_north(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    texture: &Handle<Image>,
    position: IVec3,
) -> Entity {
    let half_line = get_mesh(JunctionUVs::HLineN);
    let horizontal = get_mesh(JunctionUVs::Horizontal);
    let dot = get_mesh(JunctionUVs::Dot);
    let pos = position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01);

    commands
        .spawn((
            Name::new("Updated TCross"),
            Transform::from_translation(pos)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            GlobalTransform::default(),
            Pickable {
                is_hoverable: true,
                ..default()
            },
            Mesh3d(meshes.add(horizontal)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture.clone()),
                perceptual_roughness: 1.0,
                emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            })),
            Visibility::Visible,
            children![
                (
                    Mesh3d(meshes.add(half_line)),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(texture.clone()),
                        perceptual_roughness: 1.0,
                        emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                        base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    })),
                    Transform::from_translation(Vec3::Y * 0.25)
                ),
                (
                    Mesh3d(meshes.add(dot)),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(texture.clone()),
                        perceptual_roughness: 1.0,
                        emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                        base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    })),
                )
            ],
        ))
        .observe(track_hovered_block)
        .observe(track_grid_cordinate)
        .observe(untrack_hovered_block)
        .id()
}

pub fn spawn_tcross_south(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    texture: &Handle<Image>,
    position: IVec3,
) -> Entity {
    let half_line = get_mesh(JunctionUVs::HLineS);
    let horizontal = get_mesh(JunctionUVs::Horizontal);
    let dot = get_mesh(JunctionUVs::Dot);
    let pos = position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01);

    commands
        .spawn((
            Name::new("TCross South"),
            Transform::from_translation(pos)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            GlobalTransform::default(),
            Pickable {
                is_hoverable: true,
                ..default()
            },
            Mesh3d(meshes.add(horizontal)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture.clone()),
                perceptual_roughness: 1.0,
                emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            })),
            Visibility::Visible,
            children![
                (
                    Mesh3d(meshes.add(half_line)),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(texture.clone()),
                        perceptual_roughness: 1.0,
                        emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                        base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    })),
                    Transform::from_translation(Vec3::Y * -0.25)
                ),
                (
                    Mesh3d(meshes.add(dot)),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(texture.clone()),
                        perceptual_roughness: 1.0,
                        emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                        base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    })),
                )
            ],
        ))
        .observe(track_hovered_block)
        .observe(track_grid_cordinate)
        .observe(untrack_hovered_block)
        .id()
}

pub fn spawn_tcross_east(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    texture: &Handle<Image>,
    position: IVec3,
) -> Entity {
    let half_line = get_mesh(JunctionUVs::HLineE);
    let vertical = get_mesh(JunctionUVs::Vertical);
    let dot = get_mesh(JunctionUVs::Dot);
    let pos = position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01);

    commands
        .spawn((
            Name::new("TCross East"),
            Transform::from_translation(pos)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            GlobalTransform::default(),
            Pickable {
                is_hoverable: true,
                ..default()
            },
            Mesh3d(meshes.add(vertical)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture.clone()),
                perceptual_roughness: 1.0,
                emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            })),
            Visibility::Visible,
            children![
                (
                    Mesh3d(meshes.add(half_line)),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(texture.clone()),
                        perceptual_roughness: 1.0,
                        emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                        base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    })),
                    Transform::from_translation(Vec3::X * 0.25)
                ),
                (
                    Mesh3d(meshes.add(dot)),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(texture.clone()),
                        perceptual_roughness: 1.0,
                        emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                        base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    })),
                )
            ],
        ))
        .observe(track_hovered_block)
        .observe(track_grid_cordinate)
        .observe(untrack_hovered_block)
        .id()
}

pub fn spawn_tcross_west(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    texture: &Handle<Image>,
    position: IVec3,
) -> Entity {
    let half_line = get_mesh(JunctionUVs::HLineW);
    let vertical = get_mesh(JunctionUVs::Vertical);
    let dot = get_mesh(JunctionUVs::Dot);
    let pos = position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01);

    commands
        .spawn((
            Name::new("TCross West"),
            Transform::from_translation(pos)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            GlobalTransform::default(),
            Pickable {
                is_hoverable: true,
                ..default()
            },
            Mesh3d(meshes.add(vertical)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture.clone()),
                perceptual_roughness: 1.0,
                emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            })),
            Visibility::Visible,
            children![
                (
                    Mesh3d(meshes.add(half_line)),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(texture.clone()),
                        perceptual_roughness: 1.0,
                        emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                        base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    })),
                    Transform::from_translation(Vec3::X * -0.25)
                ),
                (
                    Mesh3d(meshes.add(dot)),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(texture.clone()),
                        perceptual_roughness: 1.0,
                        emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                        base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    })),
                )
            ],
        ))
        .observe(track_hovered_block)
        .observe(track_grid_cordinate)
        .observe(untrack_hovered_block)
        .id()
}

pub fn spawn_corner_nw(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    texture: &Handle<Image>,
    position: IVec3,
) -> Entity {
    let half_line = get_mesh(JunctionUVs::HLineW);
    let half_line2 = get_mesh(JunctionUVs::HLineS);
    let dot = get_mesh(JunctionUVs::Dot);
    let pos = position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01);
    info!("Spawning, North West");

    commands
        .spawn((
            Name::new("NW"),
            Transform::from_translation(pos)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            GlobalTransform::default(),
            Pickable {
                is_hoverable: true,
                ..default()
            },
            Visibility::Visible,
            Mesh3d(meshes.add(dot)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture.clone()),
                perceptual_roughness: 1.0,
                emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            })),
            children![
                (
                    Mesh3d(meshes.add(half_line)),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(texture.clone()),
                        perceptual_roughness: 1.0,
                        emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                        base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    })),
                    Transform::from_translation(Vec3::X * -0.25),
                ),
                (
                    Mesh3d(meshes.add(half_line2)),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(texture.clone()),
                        perceptual_roughness: 1.0,
                        emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                        base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    })),
                    Transform::from_translation(Vec3::Y * -0.25),
                ),
            ],
        ))
        .observe(track_hovered_block)
        .observe(track_grid_cordinate)
        .observe(untrack_hovered_block)
        .id()
}

pub fn spawn_corner_ne(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    texture: &Handle<Image>,
    position: IVec3,
) -> Entity {
    let half_line = get_mesh(JunctionUVs::HLineE);
    let half_line2 = get_mesh(JunctionUVs::HLineS);
    let dot = get_mesh(JunctionUVs::Dot);
    let pos = position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01);
    info!("Spawning, North East");

    commands
        .spawn((
            Name::new("NE"),
            Transform::from_translation(pos)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            GlobalTransform::default(),
            Pickable {
                is_hoverable: true,
                ..default()
            },
            Visibility::Visible,
            Mesh3d(meshes.add(dot)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture.clone()),
                perceptual_roughness: 1.0,
                emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            })),
            children![
                (
                    Mesh3d(meshes.add(half_line)),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(texture.clone()),
                        perceptual_roughness: 1.0,
                        emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                        base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    })),
                    Transform::from_translation(Vec3::X * 0.25),
                ),
                (
                    Mesh3d(meshes.add(half_line2)),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(texture.clone()),
                        perceptual_roughness: 1.0,
                        emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                        base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    })),
                    Transform::from_translation(Vec3::Y * -0.25),
                ),
            ],
        ))
        .observe(track_hovered_block)
        .observe(track_grid_cordinate)
        .observe(untrack_hovered_block)
        .id()
}

pub fn spawn_corner_se(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    texture: &Handle<Image>,
    position: IVec3,
) -> Entity {
    let half_line = get_mesh(JunctionUVs::HLineE);
    let half_line2 = get_mesh(JunctionUVs::HLineN);
    let dot = get_mesh(JunctionUVs::Dot);
    let pos = position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01);
    info!("Spawning, South East");

    commands
        .spawn((
            Name::new("SE"),
            Transform::from_translation(pos)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            GlobalTransform::default(),
            Pickable {
                is_hoverable: true,
                ..default()
            },
            Visibility::Visible,
            Mesh3d(meshes.add(dot)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture.clone()),
                perceptual_roughness: 1.0,
                emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            })),
            children![
                (
                    Mesh3d(meshes.add(half_line)),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(texture.clone()),
                        perceptual_roughness: 1.0,
                        emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                        base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    })),
                    Transform::from_translation(Vec3::X * 0.25),
                ),
                (
                    Mesh3d(meshes.add(half_line2)),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(texture.clone()),
                        perceptual_roughness: 1.0,
                        emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                        base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    })),
                    Transform::from_translation(Vec3::Y * 0.25),
                ),
            ],
        ))
        .observe(track_hovered_block)
        .observe(track_grid_cordinate)
        .observe(untrack_hovered_block)
        .id()
}

pub fn spawn_corner_sw(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    texture: &Handle<Image>,
    position: IVec3,
) -> Entity {
    let half_line = get_mesh(JunctionUVs::HLineW);
    let half_line2 = get_mesh(JunctionUVs::HLineS);
    let dot = get_mesh(JunctionUVs::Dot);
    let pos = position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01);
    info!("Spawning, South West");

    commands
        .spawn((
            Name::new("SW"),
            Transform::from_translation(pos)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            GlobalTransform::default(),
            Pickable {
                is_hoverable: true,
                ..default()
            },
            Visibility::Visible,
            Mesh3d(meshes.add(dot)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture.clone()),
                perceptual_roughness: 1.0,
                emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            })),
            children![
                (
                    Mesh3d(meshes.add(half_line)),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(texture.clone()),
                        perceptual_roughness: 1.0,
                        emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                        base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    })),
                    Transform::from_translation(Vec3::X * -0.25),
                ),
                (
                    Mesh3d(meshes.add(half_line2)),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(texture.clone()),
                        perceptual_roughness: 1.0,
                        emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                        base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    })),
                    Transform::from_translation(Vec3::Y * 0.25),
                ),
            ],
        ))
        .observe(track_hovered_block)
        .observe(track_grid_cordinate)
        .observe(untrack_hovered_block)
        .id()
}
pub fn spawn_redstone_mesh(
    mesh: Mesh,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    position: IVec3,
    texture: &Handle<Image>,
) -> Entity {
    let entity = commands
        .spawn((
            Name::new("Redstone"),
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture.clone()),
                perceptual_roughness: 1.0,
                emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            })),
            Pickable {
                is_hoverable: true,
                ..default()
            },
            Transform::from_translation(position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01))
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ))
        .observe(track_hovered_block)
        .observe(track_grid_cordinate)
        .observe(untrack_hovered_block)
        .id();
    info!("Spawned: {}", entity);
    entity
}

pub fn update_redstone_mesh(
    entity: &Entity,
    mesh: Mesh,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    position: IVec3,
    texture: &Handle<Image>,
) -> Entity {
    commands
        .entity(*entity)
        .insert((
            Name::new("Updated Mesh"),
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture.clone()),
                perceptual_roughness: 1.0,
                emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                base_color: Color::linear_rgb(0.8, 0.0, 0.0),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            })),
            Pickable {
                is_hoverable: true,
                ..default()
            },
            Transform::from_translation(position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01))
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ))
        .observe(track_hovered_block)
        .observe(track_grid_cordinate)
        .observe(untrack_hovered_block)
        .id()
}
pub enum JunctionUVs {
    Vertical,
    Horizontal,
    HLineN,
    HLineS,
    HLineW,
    HLineE,
    Dot,
}

pub fn get_mesh(junction_uvs: JunctionUVs) -> Mesh {
    match junction_uvs {
        JunctionUVs::Vertical => {
            let mut mesh = Mesh::from(Rectangle::new(1., 1.));
            mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);
            let uvs = vec![
                [0.2, 0.3], // top-right
                [0.1, 0.3], // top-left
                [0.1, 0.4], // bottom-left
                [0.2, 0.4], // bottom-right
            ];
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            mesh
        }
        JunctionUVs::Horizontal => {
            let mut mesh = Mesh::from(Rectangle::new(1., 1.));
            mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);
            let uvs = vec![
                [0.2, 0.4], // top-right
                [0.2, 0.3], // top-left
                [0.1, 0.3], // bottom-left
                [0.1, 0.4], // bottom-right
            ];
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            mesh
        }
        JunctionUVs::HLineN => {
            let mut mesh = Mesh::from(Rectangle::new(1., 0.5));
            mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);
            let uvs = vec![
                [0.2, 0.3],  // top-right
                [0.1, 0.3],  // top-left
                [0.1, 0.35], // bottom-left
                [0.2, 0.35], // bottom-right
            ];
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            mesh
        }
        JunctionUVs::HLineS => {
            let mut mesh = Mesh::from(Rectangle::new(1., 0.5));
            mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);
            let uvs = vec![
                [0.2, 0.35], // top-right
                [0.1, 0.35], // top-left
                [0.1, 0.4],  // bottom-left
                [0.2, 0.4],  // bottom-right
            ];
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            mesh
        }
        JunctionUVs::HLineW => {
            let mut mesh = Mesh::from(Rectangle::new(0.5, 1.));
            mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);
            let uvs = vec![
                [0.2, 0.35], // top-right
                [0.2, 0.3],  // top-left
                [0.1, 0.3],  // bottom-left
                [0.1, 0.35], // bottom-right
            ];
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            mesh
        }
        JunctionUVs::HLineE => {
            let mut mesh = Mesh::from(Rectangle::new(0.5, 1.));
            mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);
            let uvs = vec![
                [0.2, 0.4],  // top-right
                [0.2, 0.35], // top-left
                [0.1, 0.35], // bottom-left
                [0.1, 0.4],  // bottom-right
            ];
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            mesh
        }
        JunctionUVs::Dot => {
            let mut mesh = Mesh::from(Rectangle::new(1., 1.));
            mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);
            let uvs = vec![
                [0.1, 0.3], // top-right
                [0.0, 0.3], // top-left
                [0.0, 0.4], // bottom-left
                [0.1, 0.4], // bottom-right
            ];
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            mesh
        }
    }
}
