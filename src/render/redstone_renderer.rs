use bevy::{prelude::*, render::render_resource::Face};

use crate::{
    BlockType, SpawnCtx, TextureAtlas,
    block_interaction_plugin::BlockFace,
    block_selection_plugin::{track_grid_cordinate, track_hovered_block, untrack_hovered_block},
    materials::redstone::RedstoneMaterials,
    meshes::MeshId,
    redstone::{
        JunctionUVs, get_mesh, spawn_corner_ne, spawn_corner_nw, spawn_corner_se, spawn_corner_sw,
        spawn_cross, spawn_redstone_mesh, spawn_tcross_east, spawn_tcross_north,
        spawn_tcross_south, spawn_tcross_west, update_redstone_mesh,
    },
    redstone_connection_plugin::JunctionType,
    render::{BlockEntities, DirtyRedstone},
};

#[derive(Component, Debug, Clone, Copy)]
pub struct Position(pub IVec3);

#[derive(Component, Debug, Clone, Copy)]
pub struct Pressed;

#[derive(Component, Debug, Clone, Copy)]
pub struct RedstoneLamp;

pub fn render_redstone(
    ctx: SpawnCtx,
    mut dirty_redstone: ResMut<DirtyRedstone>,
    mut block_entities: ResMut<BlockEntities>,
    redstone_materials: Res<RedstoneMaterials>,
) {
    let mut commands = ctx.commands;
    let mut materials = ctx.materials;
    let mut meshes = ctx.meshes;
    let texture = ctx.atlas.handles.get(&TextureAtlas::Blocks).unwrap();
    let grid = ctx.grid;
    let mesh_registry = ctx.mesh_registry;

    for position in dirty_redstone.drain() {
        let entity = block_entities.entities.get(&position);

        if let Some(block_data) = grid.get(position) {
            match entity {
                Some(entity) => match block_data.block_type {
                    BlockType::RedStoneLamp { powered } => {
                        let mesh = if powered {
                            mesh_registry.get(MeshId::RedStoneLampOn)
                        } else {
                            mesh_registry.get(MeshId::RedStoneLampOff)
                        }
                        .expect("Could not load RedStoneLamp Mesh from registry");

                        commands.entity(*entity).insert((
                            Name::new("RedstoneLamp"),
                            Mesh3d(mesh.clone()),
                            MeshMaterial3d(materials.add(StandardMaterial {
                                base_color_texture: Some(texture.clone()),
                                perceptual_roughness: 1.0,
                                ..default()
                            })),
                            Transform::from_translation(position.as_vec3()),
                            Pickable {
                                is_hoverable: true,
                                ..default()
                            },
                            Position(position),
                            RedstoneLamp,
                        ));
                    }
                    BlockType::Dust { shape, power } => {
                        let material = redstone_materials.get(shape.into(), power.into()).unwrap();
                        let entity = match shape {
                            JunctionType::Cross => {
                                commands.entity(*entity).despawn();
                                block_entities.entities.remove(&position);
                                spawn_cross(&mut commands, &mut meshes, position, material)
                            }
                            JunctionType::TNorth => {
                                commands.entity(*entity).despawn();
                                block_entities.entities.remove(&position);
                                spawn_tcross_north(&mut commands, &mut meshes, position, material)
                            }
                            JunctionType::TSouth => {
                                commands.entity(*entity).despawn();
                                block_entities.entities.remove(&position);
                                spawn_tcross_south(&mut commands, &mut meshes, position, material)
                            }
                            JunctionType::TEast => {
                                commands.entity(*entity).despawn();
                                block_entities.entities.remove(&position);
                                spawn_tcross_east(&mut commands, &mut meshes, position, material)
                            }
                            JunctionType::TWest => {
                                commands.entity(*entity).despawn();
                                block_entities.entities.remove(&position);
                                spawn_tcross_west(&mut commands, &mut meshes, position, material)
                            }
                            JunctionType::CornerNW => {
                                commands.entity(*entity).despawn();
                                block_entities.entities.remove(&position);
                                spawn_corner_nw(&mut commands, &mut meshes, position, material)
                            }
                            JunctionType::CornerNE => {
                                commands.entity(*entity).despawn();
                                block_entities.entities.remove(&position);
                                let material =
                                    redstone_materials.get(shape.into(), power.into()).unwrap();
                                spawn_corner_ne(&mut commands, &mut meshes, position, material)
                            }
                            JunctionType::CornerSW => {
                                commands.entity(*entity).despawn();
                                block_entities.entities.remove(&position);
                                let material =
                                    redstone_materials.get(shape.into(), power.into()).unwrap();
                                spawn_corner_sw(&mut commands, &mut meshes, position, material)
                            }
                            JunctionType::CornerSE => {
                                commands.entity(*entity).despawn();
                                block_entities.entities.remove(&position);
                                let material =
                                    redstone_materials.get(shape.into(), power.into()).unwrap();
                                spawn_corner_se(&mut commands, &mut meshes, position, material)
                            }
                            _ => {
                                let mesh = match shape {
                                    JunctionType::Horizontal => get_mesh(JunctionUVs::Horizontal),
                                    JunctionType::Vertical => get_mesh(JunctionUVs::Vertical),
                                    _ => get_mesh(JunctionUVs::Dot),
                                };
                                commands.entity(*entity).despawn_children();

                                let material =
                                    redstone_materials.get(shape.into(), power.into()).unwrap();

                                update_redstone_mesh(
                                    entity,
                                    mesh,
                                    &mut commands,
                                    &mut meshes,
                                    position,
                                    material,
                                )
                            }
                        };
                        block_entities.entities.insert(position, entity);
                    }
                    entity => {
                        warn!(
                            "entity: {:?} not covered by this renderer, is this a redstone classified block?",
                            entity
                        );
                    }
                },
                None => match block_data.block_type {
                    BlockType::RedStoneLamp { .. } => {
                        let mesh = mesh_registry
                            .get(MeshId::RedStoneLampOff)
                            .expect("Could not load RedstoneLampOff mesh from registry");

                        let entity = commands
                            .spawn((
                                Name::new("RedstoneLamp"),
                                Mesh3d(mesh.clone()),
                                MeshMaterial3d(materials.add(StandardMaterial {
                                    base_color_texture: Some(texture.clone()),
                                    perceptual_roughness: 1.0,
                                    ..default()
                                })),
                                Transform::from_translation(position.as_vec3()),
                                Pickable {
                                    is_hoverable: true,
                                    ..default()
                                },
                                Position(position),
                                RedstoneLamp,
                            ))
                            .observe(track_hovered_block)
                            .observe(track_grid_cordinate)
                            .observe(untrack_hovered_block)
                            .id();

                        block_entities.entities.insert(position, entity);
                    }
                    BlockType::RedStoneTorch { on_side, .. } => {
                        let stem_mesh = mesh_registry
                            .get(MeshId::RedstoneTorchStem)
                            .expect("Could not load redstone torch stem");

                        let glow_mesh = mesh_registry
                            .get(MeshId::RedstoneTorchGlow)
                            .expect("Could not load redstone torch glow");

                        let transform = {
                            let slant_pos = (25.0_f32).to_radians();
                            let slant_neg = (-25.0_f32).to_radians();
                            let distance = 0.37;
                            match on_side {
                                BlockFace::PosX => Transform::from_translation(
                                    position.as_vec3() - Vec3::X * distance,
                                )
                                .with_rotation(Quat::from_rotation_z(slant_neg)),
                                BlockFace::NegX => Transform::from_translation(
                                    position.as_vec3() - Vec3::NEG_X * distance,
                                )
                                .with_rotation(Quat::from_rotation_z(slant_pos)),
                                BlockFace::PosZ => Transform::from_translation(
                                    position.as_vec3() - Vec3::Z * distance,
                                )
                                .with_rotation(Quat::from_rotation_x(slant_pos)),
                                BlockFace::NegZ => Transform::from_translation(
                                    position.as_vec3() - Vec3::NEG_Z * distance,
                                )
                                .with_rotation(Quat::from_rotation_x(slant_neg)),
                                _ => {
                                    Transform::from_translation(position.as_vec3() - Vec3::Y * 0.25)
                                }
                            }
                        };

                        let entity = commands
                            .spawn((
                                Name::new("RedstoneTorch"),
                                Mesh3d(stem_mesh.clone()),
                                MeshMaterial3d(materials.add(StandardMaterial {
                                    base_color_texture: Some(texture.clone()),
                                    perceptual_roughness: 1.0,
                                    ..default()
                                })),
                                transform,
                                Pickable {
                                    is_hoverable: true,
                                    ..default()
                                },
                                Position(position),
                                children![(
                                    Name::new("RedstoneTorchGlow"),
                                    Mesh3d(glow_mesh.clone()),
                                    MeshMaterial3d(materials.add(StandardMaterial {
                                        emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                                        base_color: Color::linear_rgb(0.5, 0.0, 0.0),
                                        perceptual_roughness: 1.0,
                                        cull_mode: Some(Face::Front),
                                        unlit: true,
                                        ..default()
                                    })),
                                    Pickable {
                                        is_hoverable: true,
                                        ..default()
                                    },
                                    Transform::from_translation(Vec3::Y * 0.25),
                                )],
                            ))
                            .observe(track_hovered_block)
                            .observe(track_grid_cordinate)
                            .observe(untrack_hovered_block)
                            .id();

                        block_entities.entities.insert(position, entity);
                    }
                    BlockType::Dust { shape, power } => {
                        let material = redstone_materials.get(shape.into(), power.into()).unwrap();

                        let mesh = match shape {
                            JunctionType::Horizontal => get_mesh(JunctionUVs::Horizontal),
                            JunctionType::Vertical => get_mesh(JunctionUVs::Vertical),
                            _ => get_mesh(JunctionUVs::Dot),
                        };

                        let entity = spawn_redstone_mesh(
                            meshes.add(mesh),
                            &mut commands,
                            position,
                            material,
                        );

                        block_entities.entities.insert(position, entity);
                    }
                    BlockType::StoneButton { .. } => {
                        let mesh = mesh_registry
                            .get(MeshId::StoneButton)
                            .expect("Could not load StoneButton mesh from registry");

                        let entity = commands
                            .spawn((
                                Name::new("StoneButton"),
                                Mesh3d(mesh.clone()),
                                MeshMaterial3d(materials.add(StandardMaterial {
                                    base_color_texture: Some(texture.clone()),
                                    perceptual_roughness: 1.0,
                                    ..default()
                                })),
                                Transform::from_translation(position.as_vec3() + Vec3::Y * -0.5),
                                Pickable {
                                    is_hoverable: true,
                                    ..default()
                                },
                                Position(position),
                                Pressed,
                            ))
                            .observe(track_hovered_block)
                            .observe(track_grid_cordinate)
                            .observe(untrack_hovered_block)
                            .id();

                        block_entities.entities.insert(position, entity);
                    }
                    _ => {}
                },
            }
        } else if let Some(entity) = entity {
            commands.entity(*entity).despawn();
            block_entities.entities.remove(&position);
        }
    }
}
