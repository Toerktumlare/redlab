use bevy::prelude::*;

use crate::{
    blocks::Tickable,
    grid_plugin::Grid,
    interactions::HoveredBlockInfo,
    redstone::{GlobalTick, Scheduler},
    ui::{BlockPosInfo, BlockPowerInfo, Immediate, TickText},
};

pub fn debug_info(tick_counter: Res<GlobalTick>, mut query: Query<&mut TextSpan, With<TickText>>) {
    for mut span in &mut query {
        **span = format!("{}", tick_counter.read());
    }
}

pub fn hovered_block(
    hovered_block_info: Res<HoveredBlockInfo>,
    grid: Res<Grid>,
    mut texts: ParamSet<(
        Query<&mut Text, With<BlockPosInfo>>,
        Query<&mut TextSpan, With<BlockPowerInfo>>,
    )>,
) {
    if let Some(position) = hovered_block_info.position {
        {
            let mut pos_query = texts.p0();
            let mut pos_span = pos_query.single_mut().unwrap();
            **pos_span = format!("(x: {}, y: {}, z: {})", position.x, position.y, position.z);
        }

        if let Some(block_type) = grid.get_blocktype(position) {
            {
                let mut power_query = texts.p1();
                let mut power_span = power_query.single_mut().unwrap();
                **power_span = format!("{}", block_type.power());
            }
        }
    }
}

pub fn scheduler_info(scheduler: Res<Scheduler>, mut query: Query<&mut Text, With<Immediate>>) {
    let mut s = String::new();
    for (tick, set) in scheduler.immediate_queue() {
        for p in set {
            s.push_str(&format!(
                "- {}: (x: {}, y: {}, z: {})\n",
                tick, p.x, p.y, p.z
            ));
        }
    }

    let mut text = query.single_mut().unwrap();
    **text = s;
}
