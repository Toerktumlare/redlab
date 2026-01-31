use bevy::prelude::*;

use crate::{RenderCtx, blocks::Renderable, render::DirtyRender};

pub fn renderer(mut ctx: RenderCtx, dirty_render: Res<DirtyRender>) {
    for position in &dirty_render.positions {
        let block_type = ctx.grid.get(*position).map(|b| b.block_type);

        let entity = ctx.block_entities.entities.get(position).copied();

        match (block_type, entity) {
            (Some(block_type), Some(entity)) => {
                block_type.update(&mut ctx, entity, *position);
            }

            (Some(block_type), None) => {
                block_type.spawn(&mut ctx, *position);
            }

            (None, Some(entity)) => {
                info!("deleted dust {}", entity);
                ctx.commands.entity(entity).despawn();
                ctx.block_entities.entities.remove(position);
            }

            (None, None) => {}
        }
    }
}
