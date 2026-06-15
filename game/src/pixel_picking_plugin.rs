use bevy::camera::visibility::RenderLayers;
use bevy::picking::PickingSystems;
use bevy::picking::backend::{HitData, PointerHits};
use bevy::picking::pointer::PointerId;
use bevy::prelude::*;

/// Put this on the camera that renders to a texture
#[derive(Component)]
pub struct PixelCamera;
/// Put this on the camera that looks at the world with the texture
#[derive(Component)]
pub struct OuterCamera;

pub struct PixelPickingPlugin;
impl Plugin for PixelPickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update_hits.in_set(PickingSystems::Backend));
    }
}

/// Modification of: https://github.com/bevyengine/bevy/blob/release-0.17.2/crates/bevy_picking/src/mesh_picking/mod.rs#L77
/// Casts rays into the scene using [`MeshPickingSettings`] and sends [`PointerHits`] events.
pub fn update_hits(
    backend_settings: Res<MeshPickingSettings>,
    pickables: Query<&Pickable>,
    layers: Query<&RenderLayers>,
    mut ray_cast: MeshRayCast,
    mut output: MessageWriter<PointerHits>,
    camera_query: Query<
        (Entity, &Camera, &GlobalTransform, Option<&RenderLayers>),
        With<PixelCamera>,
    >,
    window_query: Query<&Window>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };
    let Ok((camera_entity, camera, camera_transform, cam_layers)) = camera_query.single() else {
        return;
    };

    let filter = |entity| {
        mesh_cast_filter(
            entity,
            &backend_settings,
            &pickables,
            &layers,
            cam_layers.unwrap_or_default(),
        )
    };

    let early_exit_test = |entity_hit| {
        let Ok(pickable) = pickables.get(entity_hit) else {
            return false;
        };

        if *pickable == Pickable::IGNORE {
            return false;
        }

        pickable.should_block_lower
    };

    let settings = MeshRayCastSettings::default()
        .with_visibility(backend_settings.ray_cast_visibility)
        .with_filter(&filter)
        .with_early_exit_test(&early_exit_test);

    let normalized =
        window.cursor_position().unwrap_or_default() / Vec2::new(window.width(), window.height());

    let tex_size = Vec2::new(640.0, 360.0);
    let tex_pixel = normalized * tex_size;

    let ray = camera
        .viewport_to_world(camera_transform, tex_pixel)
        .unwrap(); // remove this unwrap, do it proper 
    let picks = ray_cast
        .cast_ray(ray, &settings)
        .iter()
        .map(|(entity, hit)| {
            let hit_data = HitData::new(
                camera_entity,
                hit.distance,
                Some(hit.point),
                Some(hit.normal),
            );
            (*entity, hit_data)
        })
        .collect::<Vec<_>>();
    let order = camera.order as f32;
    if !picks.is_empty() {
        output.write(PointerHits::new(PointerId::Mouse, picks, order));
    }
}

fn mesh_cast_filter(
    entity: Entity,
    settings: &Res<MeshPickingSettings>,
    pickables: &Query<&Pickable>,
    layers: &Query<&RenderLayers>,
    cam_layers: &RenderLayers,
) -> bool {
    if settings.require_markers {
        let is_pickable = pickables.get(entity).is_ok_and(|p| p.is_hoverable);
        if !is_pickable {
            return false;
        }
    }
    let entity_layers = layers.get(entity).unwrap_or_default();
    cam_layers.intersects(entity_layers)
}
