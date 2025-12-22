use bevy::prelude::*;

use crate::{
    SelectedBlock,
    block_lifecycle_plugin::{PlaceBlockRequestEvent, RemoveBlockRequestEvent},
    block_selection_plugin::HoveredBlockInfo,
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
        && let Some(block) = selected_block.0
        && let Some(position) = hovered_block.position
        && let Some(normal) = hovered_block.normal
    {
        info!("Clicked mouse buttyn");
        let position = (position.as_vec3() + normal).round().as_ivec3();

        commands.trigger(PlaceBlockRequestEvent { position, block });
    }
}

pub fn request_delete_hovered_block(
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    hovered_block: Res<HoveredBlockInfo>,
) {
    if mouse_buttons.just_pressed(MouseButton::Right)
        && let Some(entity) = hovered_block.entity
    {
        commands.trigger(RemoveBlockRequestEvent { entity });
    }
}
