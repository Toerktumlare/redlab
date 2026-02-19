use bevy::prelude::*;

use crate::{
    SelectedBlock,
    blocks::{BlockType, Dust, RedStone, RedStoneLamp, RedStoneTorch, StandardGrass},
    redstone::GlobalTick,
};

pub(crate) fn select_block(
    key_input: Res<ButtonInput<KeyCode>>,
    mut selected_block: ResMut<SelectedBlock>,
    mut tick_counter: ResMut<GlobalTick>,
) {
    if key_input.just_pressed(KeyCode::Digit1) {
        if let Some(BlockType::StandardGrass(StandardGrass { .. })) = selected_block.0 {
            info!("Deselecting Grass");
            selected_block.0 = None;
        } else {
            info!("Selecting Grass");
            selected_block.0 = Some(BlockType::StandardGrass(StandardGrass::default()));
        }
    }

    if key_input.just_pressed(KeyCode::Digit2) {
        if let Some(BlockType::RedStone(RedStone { .. })) = selected_block.0 {
            info!("Deselecting RedStone");
            selected_block.0 = None;
        } else {
            info!("Selecting RedStone");
            selected_block.0 = Some(BlockType::RedStone(RedStone::default()));
        }
    }

    if key_input.just_pressed(KeyCode::Digit3) {
        if let Some(BlockType::RedStoneLamp { .. }) = selected_block.0 {
            info!("Deselecting RedStoneLamp");
            selected_block.0 = None;
        } else {
            info!("Selecting RedStoneLamp");
            selected_block.0 = Some(BlockType::RedStoneLamp(RedStoneLamp::default()));
        }
    }

    if key_input.just_pressed(KeyCode::Digit4) {
        if let Some(BlockType::Dust { .. }) = selected_block.0 {
            info!("Deselecting Dust");
            selected_block.0 = None;
        } else {
            info!("Selecting Dust");
            selected_block.0 = Some(BlockType::Dust(Dust::default()));
        }
    }

    if key_input.just_pressed(KeyCode::Digit5) {
        if let Some(BlockType::RedStoneTorch { .. }) = selected_block.0 {
            info!("Deselecting RedStone Torch");
            selected_block.0 = None;
        } else {
            info!("Selecting RedStoneTorch");
            selected_block.0 = Some(BlockType::RedStoneTorch(RedStoneTorch::default()));
        }
    }

    // if key_input.just_pressed(KeyCode::Digit6) {
    //     if let Some(BlockType::StoneButton { .. }) = selected_block.0 {
    //         info!("Deselecting Stone Button");
    //         selected_block.0 = None;
    //     } else {
    //         info!("Selecting Stone Button");
    //         selected_block.0 = Some(BlockType::StoneButton {
    //             ticks: 10,
    //             pressed: false,
    //             attached_face: IVec3::NEG_X,
    //         });
    //     }
    // }

    if key_input.just_pressed(KeyCode::Tab) {
        if tick_counter.is_running() {
            tick_counter.stop();
        } else {
            tick_counter.start();
        }
    }
}
