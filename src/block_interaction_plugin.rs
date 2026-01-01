use bevy::prelude::*;

use crate::{
    SelectedBlock,
    block_selection_plugin::HoveredBlockInfo,
    grid_plugin::{BlockChange, PlaceRequest, RemoveRequest},
};

pub struct BlockInteractionPlugin;

impl Plugin for BlockInteractionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            Update,
            (request_place_selected_block, request_delete_hovered_block),
        );
    }
}

pub fn request_place_selected_block(
    mut commands: Commands,
    selected_block: Res<SelectedBlock>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    hovered_block: Res<HoveredBlockInfo>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left)
        && let Some(block_type) = selected_block.0
        && let Some(position) = hovered_block.position
        && let Some(normal) = hovered_block.normal
    {
        info!(
            "position: {}, normal: {}, block: {:?}",
            position, normal, block_type,
        );

        commands.trigger(BlockChange::Place(PlaceRequest {
            position,
            normal,
            block_type,
        }));
    }
}

pub fn request_delete_hovered_block(
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    hovered_block: Res<HoveredBlockInfo>,
) {
    if mouse_buttons.just_pressed(MouseButton::Right)
        && let Some(position) = hovered_block.position
    {
        commands.trigger(BlockChange::Remove(RemoveRequest { position }));
    }
}
