use bevy::{prelude::*, render::render_resource::Face};

use crate::{
    TextureAtlas,
    blocks::{Block, BlockType, NeighbourUpdate, RecomputedResult, Renderable, Tickable},
    grid_plugin::Grid,
    interactions::{track_grid_cordinate, track_hovered_block, untrack_hovered_block},
    meshes::MeshId,
    redstone::NotifyDelay,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RedStoneTorch {
    pub lit: bool,
    pub attached_face: IVec3,
}

impl Block for RedStoneTorch {
    fn on_placement(&self, grid: &Grid, position: IVec3, normal: IVec3) -> RecomputedResult<'_> {
        let attached_block_has_power = grid.get_direct_signal(position - normal) > 0;
        let Some(_) = grid.get(position) else {
            return RecomputedResult::Changed {
                new_block: Some(BlockType::RedStoneTorch(RedStoneTorch {
                    lit: !attached_block_has_power,
                    attached_face: normal,
                })),
                visual_update: true,
                self_tick: None,
                neighbor_tick: NeighbourUpdate::DEFAULT,
            };
        };
        RecomputedResult::Unchanged
    }

    fn neighbor_changed(
        &self,
        grid: &crate::grid_plugin::Grid,
        position: IVec3,
    ) -> RecomputedResult<'_> {
        info!("Torch neighbour changed LETS RECOMPUTE THE TORCH!");
        let attached_block_has_power = grid.get_direct_signal(position - self.attached_face) > 0;

        if self.lit == attached_block_has_power {
            // If power has changed, schedule for next tick, but dont update the torch state yet
            return RecomputedResult::Changed {
                new_block: None,
                visual_update: false,
                self_tick: Some(NotifyDelay::NextTick),
                neighbor_tick: NeighbourUpdate::DEFAULT,
            };
        }

        RecomputedResult::Unchanged
    }

    fn try_place(&self, _grid: &crate::grid_plugin::Grid, _position: IVec3) -> bool {
        true
    }
}

impl Tickable for RedStoneTorch {
    fn on_tick(&self, grid: &Grid, position: IVec3) -> RecomputedResult<'_> {
        let attached_block_has_power = grid.get_direct_signal(position - self.attached_face) > 0;

        info!("TICK: {}", attached_block_has_power);
        if self.lit == attached_block_has_power {
            return RecomputedResult::Changed {
                new_block: Some(BlockType::RedStoneTorch(RedStoneTorch {
                    lit: !attached_block_has_power,
                    attached_face: self.attached_face,
                })),
                visual_update: true,
                self_tick: None,
                neighbor_tick: NeighbourUpdate::DEFAULT,
            };
        }

        RecomputedResult::Unchanged
    }

    fn power(&self) -> u8 {
        0
    }
}

impl Renderable for RedStoneTorch {
    fn spawn(&self, ctx: &mut crate::RenderCtx, position: IVec3) {
        let texture = ctx.atlas.handles.get(&TextureAtlas::Blocks);

        let Some(block_type) = ctx.grid.get_blocktype(position) else {
            return;
        };

        let BlockType::RedStoneTorch(block) = block_type else {
            return;
        };

        let stem_mesh = if block.lit {
            ctx.mesh_registry
                .get(MeshId::RedstoneTorchStemOn)
                .expect("Could not load redstone torch stem")
        } else {
            ctx.mesh_registry
                .get(MeshId::RedstoneTorchStemOff)
                .expect("Could not load redstone torch stem")
        };

        let transform = {
            let slant_pos = (25.0_f32).to_radians();
            let slant_neg = (-25.0_f32).to_radians();
            let distance = 0.37;
            match block.attached_face {
                IVec3::X => Transform::from_translation(position.as_vec3() - Vec3::X * distance)
                    .with_rotation(Quat::from_rotation_z(slant_neg)),
                IVec3::NEG_X => {
                    Transform::from_translation(position.as_vec3() - Vec3::NEG_X * distance)
                        .with_rotation(Quat::from_rotation_z(slant_pos))
                }
                IVec3::Z => Transform::from_translation(position.as_vec3() - Vec3::Z * distance)
                    .with_rotation(Quat::from_rotation_x(slant_pos)),
                IVec3::NEG_Z => {
                    Transform::from_translation(position.as_vec3() - Vec3::NEG_Z * distance)
                        .with_rotation(Quat::from_rotation_x(slant_neg))
                }
                _ => Transform::from_translation(position.as_vec3() - Vec3::Y * 0.25),
            }
        };

        let entity = ctx
            .commands
            .spawn((
                Name::new("RedstoneTorch"),
                Mesh3d(stem_mesh.clone()),
                MeshMaterial3d(ctx.materials.add(StandardMaterial {
                    base_color_texture: texture.cloned(),
                    perceptual_roughness: 1.0,
                    ..default()
                })),
                transform,
                Pickable {
                    is_hoverable: true,
                    ..default()
                },
            ))
            .observe(track_hovered_block)
            .observe(track_grid_cordinate)
            .observe(untrack_hovered_block)
            .id();

        if block.lit {
            let glow_mesh = ctx
                .mesh_registry
                .get(MeshId::RedstoneTorchGlow)
                .expect("Could not load redstone torch glow");

            ctx.commands.entity(entity).with_child((
                Name::new("RedstoneTorchGlow"),
                Mesh3d(glow_mesh.clone()),
                MeshMaterial3d(ctx.materials.add(StandardMaterial {
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
            ));
        }

        ctx.block_entities.entities.insert(position, entity);
    }

    fn update(&self, _ctx: &mut crate::RenderCtx, _entity: Entity, _position: IVec3) {
        todo!()
    }
}
