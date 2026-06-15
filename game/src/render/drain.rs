use bevy::prelude::*;

use crate::{
    SpawnCtx,
    render::{BlockEntities, DirtyRender},
};

pub fn cleanup(
    ctx: SpawnCtx,
    mut dirty_render: ResMut<DirtyRender>,
    mut block_entities: ResMut<BlockEntities>,
) {
    let mut commands = ctx.commands;
    let grid = ctx.grid;

    for position in dirty_render.drain() {
        if let None = grid.get(position)
            && let Some(entity) = block_entities.entities.get(&position)
        {
            info!("Despawning entity: {entity:?}");
            commands.entity(*entity).despawn();
            block_entities.entities.remove(&position);
        }
    }
}
