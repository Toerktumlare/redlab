use bevy::prelude::*;

use crate::{
    BlockType, SpawnCtx, TextureAtlas,
    block_selection_plugin::{track_grid_cordinate, track_hovered_block, untrack_hovered_block},
    meshes::MeshId,
    render::{BlockEntities, DirtyBlocks},
};

pub fn render_blocks(
    ctx: SpawnCtx,
    mut dirty_blocks: ResMut<DirtyBlocks>,
    mut block_entities: ResMut<BlockEntities>,
) {
    let texture = ctx.atlas.handles.get(&TextureAtlas::Blocks).unwrap();
    let mut commands = ctx.commands;
    let grid = ctx.grid;
    let mut materials = ctx.materials;
    let mesh_registry = ctx.mesh_registry;

    for position in dirty_blocks.drain() {
        let entity = block_entities.entities.get(&position);

        if let Some(block_data) = grid.get(position) {
            match entity {
                Some(entity) => match block_data.block_type {
                    BlockType::RedStone => {
                        let mesh = mesh_registry
                            .get(MeshId::RedStoneBlock)
                            .expect("Could not load Redstone Block Mesh from registry");
                        commands.entity(*entity).insert((
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
                        ));
                    }
                    BlockType::StandardGrass => {
                        let mesh = mesh_registry
                            .get(MeshId::StandardGrass)
                            .expect("Could not load Standard Grass Mesh from registry");
                        commands.entity(*entity).insert((
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
                        ));
                    }
                    BlockType::Dirt => {
                        let mesh = mesh_registry
                            .get(MeshId::StandardDirt)
                            .expect("Could not load Standard Dirt Mesh from registry");
                        commands.entity(*entity).insert((
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
                        ));
                    }
                    _ => {
                        warn!(
                            "could not render block: {block_data:?} are you sure this is a block?"
                        )
                    }
                },
                None => match block_data.block_type {
                    BlockType::StandardGrass => {
                        let mesh = mesh_registry
                            .get(MeshId::StandardGrass)
                            .expect("Could not load Standard Grass Mesh from registry");

                        info!("Positionl: {:?}", position.as_vec3());

                        let entity = commands
                            .spawn((
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
                            ))
                            .observe(track_hovered_block)
                            .observe(track_grid_cordinate)
                            .observe(untrack_hovered_block)
                            .id();
                        block_entities.entities.insert(position, entity);
                    }
                    BlockType::RedStone => {
                        let mesh = mesh_registry
                            .get(MeshId::RedStoneBlock)
                            .expect("Could not load Redstone Block Mesh from registry");

                        let entity = commands
                            .spawn((
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
                            ))
                            .observe(track_hovered_block)
                            .observe(track_grid_cordinate)
                            .observe(untrack_hovered_block)
                            .id();
                        block_entities.entities.insert(position, entity);
                    }
                    _ => {
                        warn!(
                            "could not render block: {block_data:?} are you sure this is a block?"
                        )
                    }
                },
            }
        } else if let Some(entity) = entity {
            info!("Despawning entity: {entity:?}");
            commands.entity(*entity).despawn();
            block_entities.entities.remove(&position);
        }
    }
}
