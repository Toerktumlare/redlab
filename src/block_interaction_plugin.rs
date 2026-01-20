use bevy::prelude::*;

use crate::{
    BlockType, SelectedBlock,
    block_selection_plugin::HoveredBlockInfo,
    blocks::{Block, RecomputedResult},
    grid_plugin::{BlockChange, Grid, Place, Remove},
    redstone::NotifyDelay,
    render::DirtyRender,
};

pub struct BlockInteractionPlugin;

impl Plugin for BlockInteractionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_observer(try_place_in_world);
    }
}

#[derive(Event, Debug)]
pub struct ClickEvent(Action);

#[derive(Debug)]
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
            info!("Triggered placement!");
            commands.trigger(ClickEvent(Action::PlaceBlock(block_type, position, normal)));
        } else {
            commands.trigger(ClickEvent(Action::Interact(position)));
        }
    }
}

pub fn try_place_in_world(event: On<ClickEvent>, mut commands: Commands, grid: Res<Grid>) {
    if let Action::PlaceBlock(block_type, position, normal) = event.0 {
        let position = position + normal;

        // TODO: resolve side placement from normal

        let result = block_type.neighbor_changed(&grid, position);

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
            RecomputedResult::Unchanged => (block_type, false, None, None),
        };

        if !block_type.try_place(&grid, position) {
            return;
        };

        commands.trigger(BlockChange::Place(Place::new(
            block_type,
            position,
            visual_update,
            self_tick,
            neighbor_tick,
        )));
    }
}

// pub fn try_interact_with_world(event: On<ClickEvent>, mut commands: Commands, grid: Res<Grid>) {
//     if let Action::Interact(position) = event.0 {
//         let Some(block_data) = grid.get(position) else {
//             return;
//         };

//         if block_data.block_type.is_button()
//             && !block_data.block_type.is_pressed()
//             && let BlockType::StoneButton { attached_face, .. } = block_data.block_type
//         {
//             commands.trigger(BlockChange::Update(UpdateRequest {
//                 position,
//                 block_type: BlockType::StoneButton {
//                     pressed: true,
//                     attached_face,
//                     ticks: 15,
//                 },
//             }));
//         }
//     }
// }

pub fn request_delete_hovered_block(
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
            Some(NotifyDelay::NextTick),
        )));

        dirty_render.mark(position);
    }
}
