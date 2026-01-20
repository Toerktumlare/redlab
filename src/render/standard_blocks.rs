use bevy::prelude::*;

use crate::{
    SpawnCtx, TextureAtlas,
    block_selection_plugin::{track_grid_cordinate, track_hovered_block, untrack_hovered_block},
    blocks::BlockType,
    meshes::MeshId,
    render::{BlockEntities, DirtyRender},
};

pub fn basic_blocks(
    ctx: SpawnCtx,
    dirty_render: ResMut<DirtyRender>,
    mut block_entities: ResMut<BlockEntities>,
) {
    let texture = ctx.atlas.handles.get(&TextureAtlas::Blocks).unwrap();
    let mut commands = ctx.commands;
    let grid = ctx.grid;
    let mut materials = ctx.materials;
    let mesh_registry = ctx.mesh_registry;

    for (position, block_type) in dirty_render.filter(&grid, is_basic_block) {
        info!(?position, ?block_type);
        let entity = block_entities.entities.get(&position);
        match entity {
            // TODO: Do we actually need to ever update here?
            Some(entity) => match block_type {
                BlockType::StandardGrass { .. } => {
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
                BlockType::Dirt { .. } => {
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
                        "could not update block: {block_type:?} are you sure this is a standard block?"
                    )
                }
            },
            None => match block_type {
                BlockType::StandardGrass { .. } => {
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
                _ => {
                    warn!(
                        "could not render block: {block_type:?} are you sure this is a standard block?"
                    )
                }
            },
        }
    }
}

fn is_basic_block(block_type: &BlockType) -> bool {
    matches!(
        block_type,
        BlockType::StandardGrass { .. } | BlockType::Dirt { .. }
    )
}
