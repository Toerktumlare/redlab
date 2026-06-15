use bevy::prelude::*;

use crate::{
    SpawnCtx, TextureAtlas,
    block_selection_plugin::{track_grid_cordinate, track_hovered_block, untrack_hovered_block},
    blocks::BlockType,
    meshes::MeshId,
    render::{BlockEntities, DirtyRedstone, Position, Pressed},
};

fn is_button_block(block_type: &BlockType) -> bool {
    matches!(block_type, BlockType::StoneButton { .. })
}

pub fn render_buttons(
    ctx: SpawnCtx,
    dirty_redstone: ResMut<DirtyRedstone>,
    mut block_entities: ResMut<BlockEntities>,
) {
    let texture = ctx.atlas.handles.get(&TextureAtlas::Blocks).unwrap();
    let mut commands = ctx.commands;
    let grid = ctx.grid;
    let mut materials = ctx.materials;
    let mesh_registry = ctx.mesh_registry;

    for (position, block_type) in dirty_redstone.filter(&grid, is_button_block) {
        info!("Placing a stone button");
        match block_type {
            BlockType::StoneButton {
                pressed,
                attached_face,
                ticks,
            } => {
                let entity = if let Some(entity) = block_entities.entities.get(&position) {
                    let transform = if *ticks > 13 {
                        Transform::from_translation(position.as_vec3() + Vec3::Y * -0.55)
                    } else {
                        Transform::from_translation(position.as_vec3() + Vec3::Y * -0.5)
                    };

                    commands
                        .entity(*entity)
                        .insert(transform)
                        .insert(Pressed(*pressed))
                        .id()
                } else {
                    let mesh = mesh_registry
                        .get(MeshId::StoneButton)
                        .expect("Could not load StoneButton Block Mesh from registry");

                    spawn(
                        &mut commands,
                        position,
                        &mut materials,
                        texture.clone(),
                        mesh.clone(),
                    )
                    .observe(track_hovered_block)
                    .observe(track_grid_cordinate)
                    .observe(untrack_hovered_block)
                    .id()
                };
                block_entities.entities.insert(position, entity);
            }
            _ => {
                warn!("could not render block: {block_type:?} are you sure this is a block?")
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
        Pressed(false),
    ))
}
