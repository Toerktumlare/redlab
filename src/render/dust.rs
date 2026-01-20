use bevy::prelude::*;

use crate::{
    BlockType, SpawnCtx,
    block_selection_plugin::{track_grid_cordinate, track_hovered_block, untrack_hovered_block},
    blocks::Block,
    materials::redstone::RedstoneMaterials,
    redstone::{
        JunctionUVs, get_mesh, junctions::JunctionType, spawn_corner_ne, spawn_corner_nw,
        spawn_corner_se, spawn_corner_sw, spawn_cross, spawn_tcross_east, spawn_tcross_north,
        spawn_tcross_south, spawn_tcross_west,
    },
    render::{BlockEntities, DirtyRender},
};

fn is_dust_block(block_type: &BlockType) -> bool {
    matches!(block_type, BlockType::Dust(_))
}

// MUST CHECK GRID! if not in grid spawn
pub fn dust(
    ctx: SpawnCtx,
    dirty_render: ResMut<DirtyRender>,
    mut block_entities: ResMut<BlockEntities>,
    redstone_materials: Res<RedstoneMaterials>,
) {
    let mut commands = ctx.commands;
    let grid = ctx.grid;
    let mut meshes = ctx.meshes;

    for (position, block_type) in dirty_render.filter(&grid, is_dust_block) {
        let BlockType::Dust(dust) = block_type else {
            continue;
        };

        let shape = dust.shape;
        let material = redstone_materials
            .get(shape.into(), dust.power().into())
            .unwrap();

        match block_entities.entities.get(&position) {
            Some(entity) => {
                commands.entity(*entity).despawn();
                block_entities.entities.remove(&position);

                let entity = match dust.shape {
                    JunctionType::Cross => {
                        spawn_cross(&mut commands, &mut meshes, position, material)
                    }
                    JunctionType::TNorth => {
                        spawn_tcross_north(&mut commands, &mut meshes, position, material)
                    }
                    JunctionType::TSouth => {
                        spawn_tcross_south(&mut commands, &mut meshes, position, material)
                    }
                    JunctionType::TEast => {
                        spawn_tcross_east(&mut commands, &mut meshes, position, material)
                    }
                    JunctionType::TWest => {
                        spawn_tcross_west(&mut commands, &mut meshes, position, material)
                    }
                    JunctionType::CornerNW => {
                        spawn_corner_nw(&mut commands, &mut meshes, position, material)
                    }
                    JunctionType::CornerNE => {
                        spawn_corner_ne(&mut commands, &mut meshes, position, material)
                    }
                    JunctionType::CornerSW => {
                        spawn_corner_sw(&mut commands, &mut meshes, position, material)
                    }
                    JunctionType::CornerSE => {
                        spawn_corner_se(&mut commands, &mut meshes, position, material)
                    }
                    _ => {
                        let mesh = match shape {
                            JunctionType::Horizontal => get_mesh(JunctionUVs::Horizontal),
                            JunctionType::Vertical => get_mesh(JunctionUVs::Vertical),
                            _ => get_mesh(JunctionUVs::Dot),
                        };

                        let transform = Transform::from_translation(
                            position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01),
                        )
                        .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));

                        let entity = spawn(&mut commands, material, meshes.add(mesh)).id();

                        commands.entity(entity).insert(transform);
                        entity
                    }
                };
                block_entities.entities.insert(position, entity);
            }
            None => {
                let entity = match dust.shape {
                    JunctionType::Cross => {
                        spawn_cross(&mut commands, &mut meshes, position, material)
                    }
                    JunctionType::TNorth => {
                        spawn_tcross_north(&mut commands, &mut meshes, position, material)
                    }
                    JunctionType::TSouth => {
                        spawn_tcross_south(&mut commands, &mut meshes, position, material)
                    }
                    JunctionType::TEast => {
                        spawn_tcross_east(&mut commands, &mut meshes, position, material)
                    }
                    JunctionType::TWest => {
                        spawn_tcross_west(&mut commands, &mut meshes, position, material)
                    }
                    JunctionType::CornerNW => {
                        spawn_corner_nw(&mut commands, &mut meshes, position, material)
                    }
                    JunctionType::CornerNE => {
                        spawn_corner_ne(&mut commands, &mut meshes, position, material)
                    }
                    JunctionType::CornerSW => {
                        spawn_corner_sw(&mut commands, &mut meshes, position, material)
                    }
                    JunctionType::CornerSE => {
                        spawn_corner_se(&mut commands, &mut meshes, position, material)
                    }
                    _ => {
                        let mesh = match shape {
                            JunctionType::Horizontal => get_mesh(JunctionUVs::Horizontal),
                            JunctionType::Vertical => get_mesh(JunctionUVs::Vertical),
                            _ => get_mesh(JunctionUVs::Dot),
                        };

                        let transform = Transform::from_translation(
                            position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01),
                        )
                        .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));

                        let entity = spawn(&mut commands, material, meshes.add(mesh)).id();

                        commands.entity(entity).insert(transform);
                        entity
                    }
                };

                commands
                    .entity(entity)
                    .observe(track_hovered_block)
                    .observe(track_grid_cordinate)
                    .observe(untrack_hovered_block);

                block_entities.entities.insert(position, entity);
            }
        }
    }
}

fn spawn<'a>(
    commands: &'a mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
) -> EntityCommands<'a> {
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Pickable {
            is_hoverable: true,
            ..default()
        },
    ))
}
