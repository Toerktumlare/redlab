use bevy::prelude::*;

use crate::{
    BlockType, SelectedBlock,
    block_selection_plugin::HoveredBlockInfo,
    grid_plugin::{BlockChange, Grid, PlaceRequest, RemoveRequest, UpdateRequest},
};

pub struct BlockInteractionPlugin;

impl Plugin for BlockInteractionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            Update,
            (request_place_selected_block, request_delete_hovered_block),
        )
        .add_observer(try_place_in_world)
        .add_observer(try_interact_with_world);
    }
}

#[derive(Event)]
pub struct ClickEvent(Action);

pub enum Action {
    PlaceBlock(BlockType, IVec3, IVec3),
    Interact(IVec3),
}

pub fn request_place_selected_block(
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
            commands.trigger(ClickEvent(Action::PlaceBlock(block_type, position, normal)));
        } else {
            commands.trigger(ClickEvent(Action::Interact(position)));
        }
    }
}

pub fn try_place_in_world(event: On<ClickEvent>, mut commands: Commands, grid: Res<Grid>) {
    if let Action::PlaceBlock(mut block_type, position, normal) = event.0 {
        if !block_type.can_be_placed(position, normal, &grid) {
            return;
        };

        if block_type.has_face() {
            block_type.set_attached_face(normal);
        }

        commands.trigger(BlockChange::Place(PlaceRequest {
            position,
            normal,
            block_type,
        }));
    }
}

pub fn try_interact_with_world(event: On<ClickEvent>, mut commands: Commands, grid: Res<Grid>) {
    if let Action::Interact(position) = event.0 {
        let Some(block_data) = grid.get(position) else {
            return;
        };

        if block_data.block_type.is_button()
            && !block_data.block_type.is_pressed()
            && let BlockType::StoneButton { attached_face, .. } = block_data.block_type
        {
            commands.trigger(BlockChange::Update(UpdateRequest {
                position,
                block_type: BlockType::StoneButton {
                    pressed: true,
                    attached_face,
                    ticks: 15,
                },
            }));
        }
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
