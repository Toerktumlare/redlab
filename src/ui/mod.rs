use bevy::prelude::*;

mod debug_view;

pub use debug_view::debug_view_system;

#[derive(Component)]
pub struct TickText;

#[derive(Component)]
pub struct BlockPosInfo;

#[derive(Component)]
pub struct BlockPowerInfo;

#[derive(Component)]
pub struct Immediate;

#[derive(Component)]
pub struct Scheduled;
