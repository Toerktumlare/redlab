use bevy::{
    ecs::error::info,
    platform::collections::{HashMap, HashSet},
    prelude::*,
};

use crate::{
    BlockType, TextureAtlas, Textures,
    block_selection_plugin::{track_grid_cordinate, track_hovered_block, untrack_hovered_block},
    cube::{Cube, CubeTextures, TileCoords},
    grid_plugin::Grid,
    redstone::{
        JunctionUVs, get_mesh, spawn_cross, spawn_redstone_mesh, spawn_tcross_east,
        spawn_tcross_north, spawn_tcross_south, spawn_tcross_west, update_redstone_mesh,
    },
    redstone_connection_plugin::JunctionType,
};

#[derive(Resource, Default)]
pub struct BlockEntities {
    entities: HashMap<IVec3, Entity>,
}

#[derive(Resource, Default)]
pub struct DirtyBlocks {
    pub positions: HashSet<IVec3>,
}

impl DirtyBlocks {
    pub fn mark(&mut self, position: IVec3) {
        self.positions.insert(position);
    }

    pub fn drain(&mut self) -> Vec<IVec3> {
        self.positions.drain().collect()
    }
}

pub struct BlockLifecyclePlugin;

impl Plugin for BlockLifecyclePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<DirtyBlocks>()
            .init_resource::<BlockEntities>();
    }
}

pub fn draw(
    mut commands: Commands,
    mut dirty_blocks: ResMut<DirtyBlocks>,
    grid: ResMut<Grid>,
    mut block_entities: ResMut<BlockEntities>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    texture_map: ResMut<Textures>,
) {
    let texture = texture_map.handles.get(&TextureAtlas::Blocks).unwrap();

    for position in dirty_blocks.drain() {
        let entity = block_entities.entities.get(&position);

        if let Some(block_data) = grid.get(position) {
            match entity {
                Some(entity) => match block_data.block_type {
                    BlockType::Air => todo!(),
                    BlockType::RedStone => {
                        let textures = CubeTextures::new(
                            Some(TileCoords::new(0, 2)),
                            Some(TileCoords::new(0, 2)),
                            Some(TileCoords::new(0, 2)),
                            Some(TileCoords::new(0, 2)),
                            Some(TileCoords::new(0, 2)),
                            Some(TileCoords::new(0, 2)),
                        );
                        let mesh = Cube::new(textures);
                        commands.entity(*entity).insert((
                            Mesh3d(meshes.add(mesh)),
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
                        ));
                    }
                    BlockType::StandardGrass => {
                        let textures = CubeTextures::new(
                            Some(TileCoords::new(0, 1)),
                            Some(TileCoords::new(1, 1)),
                            Some(TileCoords::new(2, 1)),
                            Some(TileCoords::new(3, 1)),
                            Some(TileCoords::new(4, 1)),
                            Some(TileCoords::new(5, 1)),
                        );
                        let mesh = Cube::new(textures);
                        commands.entity(*entity).insert((
                            Mesh3d(meshes.add(mesh)),
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
                        ));
                    }
                    BlockType::RedStoneLamp { .. } => {
                        let textures = CubeTextures::new(
                            Some(TileCoords::new(1, 2)),
                            Some(TileCoords::new(1, 2)),
                            Some(TileCoords::new(1, 2)),
                            Some(TileCoords::new(1, 2)),
                            Some(TileCoords::new(1, 2)),
                            Some(TileCoords::new(1, 2)),
                        );
                        let mesh = Cube::new(textures);
                        commands.entity(*entity).insert((
                            Mesh3d(meshes.add(mesh)),
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
                        ));
                    }
                    BlockType::Dirt => {
                        let textures = CubeTextures::new(
                            Some(TileCoords::new(1, 1)),
                            Some(TileCoords::new(1, 1)),
                            Some(TileCoords::new(1, 1)),
                            Some(TileCoords::new(1, 1)),
                            Some(TileCoords::new(1, 1)),
                            Some(TileCoords::new(1, 1)),
                        );
                        let mesh = Cube::new(textures);
                        commands.entity(*entity).insert((
                            Mesh3d(meshes.add(mesh)),
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
                        ));
                    }
                    BlockType::Dust { shape, .. } => {
                        let entity = match shape {
                            JunctionType::Cross => {
                                commands.entity(*entity).despawn();
                                block_entities.entities.remove(&position);
                                spawn_cross(
                                    &mut commands,
                                    &mut materials,
                                    &mut meshes,
                                    texture,
                                    position,
                                )
                            }
                            JunctionType::TNorth => {
                                commands.entity(*entity).despawn();
                                block_entities.entities.remove(&position);
                                spawn_tcross_north(
                                    &mut commands,
                                    &mut materials,
                                    &mut meshes,
                                    texture,
                                    position,
                                )
                            }
                            JunctionType::TSouth => {
                                commands.entity(*entity).despawn();
                                block_entities.entities.remove(&position);
                                spawn_tcross_south(
                                    &mut commands,
                                    &mut materials,
                                    &mut meshes,
                                    texture,
                                    position,
                                )
                            }
                            JunctionType::TEast => {
                                commands.entity(*entity).despawn();
                                block_entities.entities.remove(&position);
                                spawn_tcross_east(
                                    &mut commands,
                                    &mut materials,
                                    &mut meshes,
                                    texture,
                                    position,
                                )
                            }
                            JunctionType::TWest => {
                                commands.entity(*entity).despawn();
                                block_entities.entities.remove(&position);
                                spawn_tcross_west(
                                    &mut commands,
                                    &mut materials,
                                    &mut meshes,
                                    texture,
                                    position,
                                )
                            }
                            _ => {
                                let mesh = match shape {
                                    JunctionType::Horizontal => get_mesh(JunctionUVs::Horizontal),
                                    JunctionType::Vertical => get_mesh(JunctionUVs::Vertical),
                                    _ => get_mesh(JunctionUVs::Dot),
                                };
                                commands.entity(*entity).despawn_children();
                                update_redstone_mesh(
                                    entity,
                                    mesh,
                                    &mut commands,
                                    &mut materials,
                                    &mut meshes,
                                    position,
                                    texture,
                                )
                            }
                        };
                        info!("Inserting entity: {}, {}", entity, position);
                        block_entities.entities.insert(position, entity);
                    }
                },
                None => match block_data.block_type {
                    BlockType::Air => todo!(),
                    BlockType::StandardGrass => {
                        let textures = CubeTextures::new(
                            Some(TileCoords::new(0, 1)),
                            Some(TileCoords::new(1, 1)),
                            Some(TileCoords::new(2, 1)),
                            Some(TileCoords::new(3, 1)),
                            Some(TileCoords::new(4, 1)),
                            Some(TileCoords::new(5, 1)),
                        );
                        let mesh = Cube::new(textures);

                        info!("Positionl: {:?}", position.as_vec3());

                        let entity = commands
                            .spawn((
                                Mesh3d(meshes.add(mesh)),
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
                            ))
                            .observe(track_hovered_block)
                            .observe(track_grid_cordinate)
                            .observe(untrack_hovered_block)
                            .id();
                        block_entities.entities.insert(position, entity);
                    }
                    BlockType::RedStone => {
                        let textures = CubeTextures::new(
                            Some(TileCoords::new(0, 2)),
                            Some(TileCoords::new(0, 2)),
                            Some(TileCoords::new(0, 2)),
                            Some(TileCoords::new(0, 2)),
                            Some(TileCoords::new(0, 2)),
                            Some(TileCoords::new(0, 2)),
                        );
                        let mesh = Cube::new(textures);
                        let entity = commands
                            .spawn((
                                Mesh3d(meshes.add(mesh)),
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
                            ))
                            .observe(track_hovered_block)
                            .observe(track_grid_cordinate)
                            .observe(untrack_hovered_block)
                            .id();
                        block_entities.entities.insert(position, entity);
                    }
                    BlockType::RedStoneLamp { .. } => {
                        let textures = CubeTextures::new(
                            Some(TileCoords::new(1, 2)),
                            Some(TileCoords::new(1, 2)),
                            Some(TileCoords::new(1, 2)),
                            Some(TileCoords::new(1, 2)),
                            Some(TileCoords::new(1, 2)),
                            Some(TileCoords::new(1, 2)),
                        );
                        let mesh = Cube::new(textures);
                        let entity = commands
                            .spawn((
                                Mesh3d(meshes.add(mesh)),
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
                            ))
                            .observe(track_hovered_block)
                            .observe(track_grid_cordinate)
                            .observe(untrack_hovered_block)
                            .id();
                        block_entities.entities.insert(position, entity);
                    }
                    BlockType::Dust { shape, .. } => {
                        let mesh = match shape {
                            JunctionType::Horizontal => get_mesh(JunctionUVs::Horizontal),
                            JunctionType::Vertical => get_mesh(JunctionUVs::Vertical),
                            _ => get_mesh(JunctionUVs::Dot),
                        };
                        let entity = spawn_redstone_mesh(
                            mesh,
                            &mut commands,
                            &mut materials,
                            &mut meshes,
                            position,
                            texture,
                        );
                        block_entities.entities.insert(position, entity);
                    }
                    _ => {}
                },
            }
        } else if let Some(entity) = entity {
            info!("Despawning entity: {entity:?}");
            commands.entity(*entity).despawn();
            block_entities.entities.remove(&position);
        }
    }
}
