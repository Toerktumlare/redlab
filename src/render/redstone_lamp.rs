use bevy::prelude::*;

use crate::{
    BlockType, SpawnCtx, TextureAtlas,
    block_selection_plugin::{track_grid_cordinate, track_hovered_block, untrack_hovered_block},
    blocks::Block,
    meshes::MeshId,
    render::{BlockEntities, DirtyRender, Position},
};

fn is_redstone_lamp(block_type: &BlockType) -> bool {
    matches!(block_type, BlockType::RedStoneLamp(_))
}

pub fn redstone_lamp(
    ctx: SpawnCtx,
    dirty_render: ResMut<DirtyRender>,
    mut block_entities: ResMut<BlockEntities>,
) {
    let texture = ctx.atlas.handles.get(&TextureAtlas::Blocks).unwrap();
    let mut commands = ctx.commands;
    let grid = ctx.grid;
    let mut materials = ctx.materials;
    let mesh_registry = ctx.mesh_registry;

    for (position, block_type) in dirty_render.filter(&grid, is_redstone_lamp) {
        match block_entities.entities.get(&position) {
            None => match block_type {
                BlockType::RedStoneLamp(_) => {
                    let mesh = mesh_registry
                        .get(MeshId::RedStoneLampOff)
                        .expect("Could not load Redstone Block Mesh from registry");

                    let entity = spawn(
                        &mut commands,
                        position,
                        &mut materials,
                        texture.clone(),
                        mesh.clone(),
                    )
                    .observe(track_hovered_block)
                    .observe(track_grid_cordinate)
                    .observe(untrack_hovered_block)
                    .id();

                    block_entities.entities.insert(position, entity);
                }
                _ => {
                    warn!("could not render block: {block_type:?} are you sure this is a block?")
                }
            },
            Some(entity) => match block_type {
                BlockType::RedStoneLamp(block) => {
                    let mesh = if block.is_powered() {
                        mesh_registry.get(MeshId::RedStoneLampOn)
                    } else {
                        mesh_registry.get(MeshId::RedStoneLampOff)
                    }
                    .expect("Could not load RedStoneLamp Mesh from registry");

                    commands.entity(*entity).insert(Mesh3d(mesh.clone()));
                }
                _ => {
                    warn!("could not render block: {block_type:?} are you sure this is a block?")
                }
            },
        }
    }
}

fn spawn<'a>(
    commands: &'a mut Commands,
    position: IVec3,
    materials: &'a mut Assets<StandardMaterial>,
    texture: Handle<Image>,
    mesh: Handle<Mesh>,
) -> EntityCommands<'a> {
    commands.spawn((
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
    ))
}
