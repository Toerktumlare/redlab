use bevy::prelude::*;

const EPS: f32 = 1e-4;

#[derive(Resource, Default, Debug)]
pub struct HoveredBlockInfo {
    pub position: Option<IVec3>,
    pub normal: Option<IVec3>,
    pub entity: Option<Entity>,
}

#[derive(Component)]
pub struct HoveredBlock;

pub struct BlockSelectionPlugin;

impl Plugin for BlockSelectionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<HoveredBlockInfo>();
    }
}

/// Track what block the mouse is hovering
pub fn track_hovered_block(event: On<Pointer<Over>>, mut commands: Commands) {
    let self_entity = event.event_target();
    commands.entity(self_entity).insert(HoveredBlock);
}

/// Untrack what block the mouse is hovering
pub fn untrack_hovered_block(
    event: On<Pointer<Out>>,
    mut commands: Commands,
    mut hovered: ResMut<HoveredBlockInfo>,
) {
    commands
        .entity(event.event_target())
        .remove::<HoveredBlock>();

    *hovered = HoveredBlockInfo::default();
}

/// Track what block we are hovering by grid cordinates
pub fn track_grid_cordinate(event: On<Pointer<Move>>, mut hovered: ResMut<HoveredBlockInfo>) {
    let target = event.event_target();
    let hit_data = &event.event.hit;

    if let Some(position) = hit_data.position
        && let Some(normal) = hit_data.normal
    {
        let inside_hit = position - (normal * EPS);

        let hovered_block = IVec3::new(
            inside_hit.x.round() as i32,
            inside_hit.y.round() as i32,
            inside_hit.z.round() as i32,
        );

        let face = face_from_normal(&normal);

        *hovered = HoveredBlockInfo {
            position: Some(hovered_block),
            normal: Some(face),
            entity: Some(target),
        };
    }
}

/// Clamps normal to a proper face since we might get floating point problems
/// For instance (0.99997,0,0) is fixed to (1,0,0)
fn face_from_normal(normal: &Vec3) -> IVec3 {
    let n = normal.normalize();
    if n.x.abs() > 0.9 {
        IVec3::new(n.x.signum() as i32, 0, 0)
    } else if n.y.abs() > 0.9 {
        IVec3::new(0, n.y.signum() as i32, 0)
    } else {
        IVec3::new(0, 0, n.z.signum() as i32)
    }
}
