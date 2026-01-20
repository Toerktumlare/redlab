use bevy::prelude::*;

use crate::{
    BlockType, SpawnCtx, TextureAtlas,
    block_selection_plugin::{track_grid_cordinate, track_hovered_block, untrack_hovered_block},
    meshes::MeshId,
    render::{BlockEntities, DirtyRender},
};

fn is_redstone_block(block_type: &BlockType) -> bool {
    matches!(block_type, BlockType::RedStone(_))
}

pub fn redstone_block(
    ctx: SpawnCtx,
    dirty_render: ResMut<DirtyRender>,
    mut block_entities: ResMut<BlockEntities>,
) {
    let texture = ctx.atlas.handles.get(&TextureAtlas::Blocks).unwrap();
    let mut commands = ctx.commands;
    let grid = ctx.grid;
    let mut materials = ctx.materials;
    let mesh_registry = ctx.mesh_registry;

    for (position, block_type) in dirty_render.filter(&grid, is_redstone_block) {
        if block_entities.entities.get(&position).is_none() {
            match block_type {
                BlockType::RedStone(_) => {
                    let mesh = mesh_registry
                        .get(MeshId::RedStoneBlock)
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
            }
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
        Mesh3d(mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(texture),
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::from_translation(position.as_vec3()),
        Pickable {
            is_hoverable: true,
            ..default()
        },
    ))
}
