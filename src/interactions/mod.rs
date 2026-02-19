use bevy::{color::palettes::tailwind::RED_500, picking::pointer::PointerInteraction, prelude::*};

mod hover;
mod keyboard;
mod mouse_click;

pub use hover::HoveredBlockInfo;
pub use hover::track_grid_cordinate;
pub use hover::track_hovered_block;
pub use hover::untrack_hovered_block;

use crate::GameLoop;
use crate::interactions::keyboard::select_block;
use crate::interactions::mouse_click::request_delete_hovered_block;
use crate::interactions::mouse_click::request_place_selected_block;
use crate::interactions::mouse_click::try_place_in_world;

pub struct BlockInteractionPlugin;

impl Plugin for BlockInteractionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<HoveredBlockInfo>()
            .add_systems(
                Update,
                (
                    draw_on_hover_arrow,
                    select_block,
                    request_place_selected_block,
                    request_delete_hovered_block,
                )
                    .in_set(GameLoop::Input),
            )
            .add_observer(try_place_in_world);
    }
}

fn draw_on_hover_arrow(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
    for (point, normal) in pointers
        .iter()
        .filter_map(|interactions| interactions.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
    {
        gizmos.arrow(point, point + normal.normalize() * 0.5, RED_500);
    }
}
