use bevy::prelude::*;

use crate::{
    SelectedBlock,
    blocks::{Block, BlockType, NeighbourUpdate, RecomputedResult},
    grid_plugin::Remove,
    grid_plugin::{BlockChange, Grid, Place},
    interactions::HoveredBlockInfo,
    render::DirtyRender,
};

#[derive(Event, Debug)]
pub struct ClickEvent(pub Action);

#[derive(Debug)]
pub enum Action {
    PlaceBlock(BlockType, IVec3, IVec3),
    Interact(IVec3),
}

pub(crate) fn request_place_selected_block(
    mut commands: Commands,
    selected_block: Res<SelectedBlock>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    hovered_block: Res<HoveredBlockInfo>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left)
        && let Some(position) = hovered_block.position
        && let Some(normal) = hovered_block.normal
    {
        if let Some(block_type) = selected_block.0 {
            info!("Triggered placement!");
            commands.trigger(ClickEvent(Action::PlaceBlock(block_type, position, normal)));
        } else {
            commands.trigger(ClickEvent(Action::Interact(position)));
        }
    }
}

pub(crate) fn try_place_in_world(event: On<ClickEvent>, mut commands: Commands, grid: Res<Grid>) {
    if let Action::PlaceBlock(block_type, position, normal) = event.0 {
        let position = position + normal;

        // TODO: resolve side placement from normal

        let result = block_type.on_placement(&grid, position, normal);

        let (block_type, visual_update, self_tick, neighbor_tick) = match result {
            RecomputedResult::Changed {
                new_block,
                visual_update,
                self_tick,
                neighbor_tick,
            } => (
                new_block.unwrap_or(block_type),
                visual_update,
                self_tick,
                neighbor_tick,
            ),
            RecomputedResult::Unchanged => (block_type, false, None, NeighbourUpdate::NONE),
        };

        if !block_type.try_place(&grid, position) {
            return;
        };

        commands.trigger(BlockChange::Place(Place::new(
            Some(block_type),
            position,
            visual_update,
            self_tick,
            neighbor_tick.to_vec(),
        )));
    }
}

pub(crate) fn request_delete_hovered_block(
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    hovered_block: Res<HoveredBlockInfo>,
    mut dirty_render: ResMut<DirtyRender>,
) {
    if mouse_buttons.just_pressed(MouseButton::Right)
        && let Some(position) = hovered_block.position
    {
        commands.trigger(BlockChange::Remove(Remove::new(
            position,
            true,
            None,
            NeighbourUpdate::EXTENDED.to_vec(),
        )));

        dirty_render.mark(position);
    }
}
