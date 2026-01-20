use bevy::prelude::*;

use crate::{
    RenderCtx,
    block_selection_plugin::{track_grid_cordinate, track_hovered_block, untrack_hovered_block},
    blocks::{ALL_DIRS, Block, BlockType, DIRS, RecomputedResult, Renderable, Tickable},
    grid_plugin::Grid,
    redstone::{
        JunctionUVs, NotifyDelay, get_mesh,
        junctions::{JunctionType, resolve_junction},
        spawn_corner_ne, spawn_corner_nw, spawn_corner_se, spawn_corner_sw, spawn_cross,
        spawn_tcross_east, spawn_tcross_north, spawn_tcross_south, spawn_tcross_west,
    },
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Dust {
    pub shape: JunctionType,
    power: u8,
}

impl Block for Dust {
    fn neighbor_changed(&self, grid: &Grid, position: IVec3) -> RecomputedResult {
        let junction = resolve_junction(position, grid);
        if junction != self.shape {
            return RecomputedResult::Changed {
                new_block: Some(BlockType::Dust(Dust {
                    shape: junction,
                    power: self.power(),
                })),
                visual_update: true,
                self_tick: Some(NotifyDelay::Immediate),
                neighbor_tick: Some(NotifyDelay::NextTick),
            };
        }
        let None = grid.get(position) else {
            return RecomputedResult::Unchanged;
        };

        RecomputedResult::Changed {
            new_block: Some(BlockType::Dust(Dust {
                shape: junction,
                power: self.power(),
            })),
            visual_update: true,
            self_tick: None,
            neighbor_tick: None,
        }
    }

    fn try_place(&self, _grid: &Grid, _position: IVec3) -> bool {
        true
    }

    fn power(&self) -> u8 {
        self.power
    }
}

impl Tickable for Dust {
    fn on_tick(&self, grid: &Grid, position: IVec3) -> RecomputedResult {
        let mut new_power = 0;
        for dir in ALL_DIRS {
            let neighbour_pos = position + dir;
            let Some(neighbour_data) = grid.get(neighbour_pos) else {
                continue;
            };

            let neighbour_block = neighbour_data.block_type;

            new_power = new_power.max(neighbour_block.strong_power_emitted_to(
                position,
                neighbour_pos,
                &neighbour_block,
            ));
            info!("{new_power}");
            new_power = new_power.max(
                neighbour_block
                    .weak_power_emitted(position, neighbour_pos, &BlockType::Dust(*self))
                    .saturating_sub(1),
            );

            // vertical corners: above and below
            for &y_offset in &[IVec3::Y, IVec3::NEG_Y] {
                for dir in DIRS {
                    let neighbour_pos = position + dir + y_offset;
                    if let Some(neighbour_block) = grid.get_blocktype(neighbour_pos) {
                        new_power = new_power.max(
                            neighbour_block
                                .weak_power_emitted(
                                    position,
                                    neighbour_pos,
                                    &BlockType::Dust(*self),
                                )
                                .saturating_sub(1),
                        );
                    }
                }
            }
        }

        if self.power() != new_power {
            RecomputedResult::Changed {
                new_block: Some(BlockType::Dust(Dust {
                    shape: self.shape,
                    power: new_power,
                })),
                visual_update: true,
                self_tick: Some(NotifyDelay::Immediate),
                neighbor_tick: Some(NotifyDelay::NextTick),
            }
        } else {
            RecomputedResult::Unchanged
        }
    }
}

impl Renderable for Dust {
    fn spawn(&self, ctx: &mut RenderCtx, position: IVec3) {
        let Some(block_type) = ctx.grid.get_blocktype(position) else {
            return;
        };

        let BlockType::Dust(dust) = block_type else {
            return;
        };

        let material = ctx
            .redstone_materials
            .get(dust.shape.into(), dust.power().into())
            .unwrap();

        let entity = match dust.shape {
            JunctionType::Cross => {
                spawn_cross(&mut ctx.commands, &mut ctx.meshes, position, material)
            }
            JunctionType::TNorth => {
                spawn_tcross_north(&mut ctx.commands, &mut ctx.meshes, position, material)
            }
            JunctionType::TSouth => {
                spawn_tcross_south(&mut ctx.commands, &mut ctx.meshes, position, material)
            }
            JunctionType::TEast => {
                spawn_tcross_east(&mut ctx.commands, &mut ctx.meshes, position, material)
            }
            JunctionType::TWest => {
                spawn_tcross_west(&mut ctx.commands, &mut ctx.meshes, position, material)
            }
            JunctionType::CornerNW => {
                spawn_corner_nw(&mut ctx.commands, &mut ctx.meshes, position, material)
            }
            JunctionType::CornerNE => {
                spawn_corner_ne(&mut ctx.commands, &mut ctx.meshes, position, material)
            }
            JunctionType::CornerSW => {
                spawn_corner_sw(&mut ctx.commands, &mut ctx.meshes, position, material)
            }
            JunctionType::CornerSE => {
                spawn_corner_se(&mut ctx.commands, &mut ctx.meshes, position, material)
            }
            _ => {
                let mesh = match dust.shape {
                    JunctionType::Horizontal => get_mesh(JunctionUVs::Horizontal),
                    JunctionType::Vertical => get_mesh(JunctionUVs::Vertical),
                    _ => get_mesh(JunctionUVs::Dot),
                };

                let transform = Transform::from_translation(
                    position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01),
                )
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));

                let entity = spawn(&mut ctx.commands, material, ctx.meshes.add(mesh)).id();

                ctx.commands.entity(entity).insert(transform);
                entity
            }
        };

        ctx.commands
            .entity(entity)
            .observe(track_hovered_block)
            .observe(track_grid_cordinate)
            .observe(untrack_hovered_block);

        ctx.block_entities.entities.insert(position, entity);
    }

    fn update(&self, _ctx: &mut RenderCtx, _entity: Entity, _position: IVec3) {
        todo!()
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
