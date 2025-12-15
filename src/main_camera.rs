use bevy::{
    camera::{RenderTarget, ScalingMode},
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    window::WindowResized,
};

use bevy::dev_tools::picking_debug::{DebugPickingMode, DebugPickingPlugin};

const RES_WIDTH: u32 = 640;
const RES_HEIGHT: u32 = 360;

#[derive(Component)]
struct Canvas;

#[derive(Component)]
pub struct InnerCamera;

#[derive(Component)]
pub struct OuterCamera;

#[derive(Resource)]
struct CameraSettings {
    center: Vec3,
    distance: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            center: Vec3::ZERO,
            distance: 10.0,
        }
    }
}

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraSettings>()
            .insert_resource(MeshPickingSettings {
                require_markers: true,
                ..default()
            })
            .insert_resource(DebugPickingMode::Noisy)
            .add_plugins((MeshPickingPlugin, DebugPickingPlugin))
            .add_systems(Startup, startup)
            .add_systems(
                Update,
                (camera_movement, camera_reset, camera_zoom, fit_canvas),
            );
    }
}

fn startup(
    mut commands: Commands,
    camera_settings: Res<CameraSettings>,
    mut images: ResMut<Assets<Image>>,
) {
    let canvas_size = Extent3d {
        width: RES_WIDTH,
        height: RES_HEIGHT,
        ..default()
    };

    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    canvas.resize(canvas_size);

    let image_handle = images.add(canvas);

    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            target: RenderTarget::Image(image_handle.clone().into()),
            ..default()
        },
        Projection::from(OrthographicProjection {
            scale: 10.0,
            scaling_mode: ScalingMode::FixedHorizontal {
                viewport_width: 1.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Msaa::Off,
        Transform::from_xyz(10.0, 10.0, 10.0).looking_at(camera_settings.center, Vec3::Y),
        InnerCamera,
        MeshPickingCamera,
    ));

    commands.spawn((Sprite::from_image(image_handle), Canvas, Pickable::IGNORE));
    commands.spawn((Camera2d, Msaa::Off, OuterCamera, Pickable::IGNORE));
}

fn camera_zoom(
    mut projection: Single<&mut Projection, With<InnerCamera>>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
) {
    let zoom_speed = 0.1;
    if let Projection::Orthographic(ortho) = &mut **projection {
        let delta_zoom = -mouse_wheel_input.delta.y * zoom_speed;
        let multiplicative_zoom = 1.0 + delta_zoom;
        ortho.scale = (ortho.scale * multiplicative_zoom).clamp(5.0, 20.0);
    }
}

fn camera_movement(
    mut camera: Single<&mut Transform, With<InnerCamera>>,
    mut camera_settings: ResMut<CameraSettings>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    let delta = mouse_motion.delta;
    let sensitivity = 0.005;

    if mouse_buttons.pressed(MouseButton::Middle) {
        let pan_speed = 0.01;
        let right = camera.right();
        let up = camera.up();

        let pan = right * (-delta.x * pan_speed) + up * (delta.y * pan_speed);

        //move the camera by amount
        camera.translation += pan;

        //move center point by same amount
        camera_settings.center += pan;
    }
    if mouse_buttons.pressed(MouseButton::Right) {
        let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta.x * sensitivity;
        let pitch = pitch + delta.y * sensitivity;
        let pitch = pitch.clamp(-1.5, 1.5);
        camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        camera.translation = camera_settings.center - camera.forward() * camera_settings.distance;
    }
}

fn camera_reset(
    mut camera: Single<&mut Transform, With<InnerCamera>>,
    mut camera_settings: ResMut<CameraSettings>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    projection: Single<&mut Projection, With<InnerCamera>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let camera_origin = Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y);
        camera_settings.center = Vec3::ZERO;
        camera_settings.distance = 10.0;
        camera.translation = camera_origin.translation;
        camera.rotation = camera_origin.rotation;
        if let Projection::Orthographic(ortho) = &mut *projection.into_inner() {
            ortho.scale = 10.0;
        }
    }
}

fn fit_canvas(
    mut resize_message: MessageReader<WindowResized>,
    mut projection: Single<&mut Projection, With<OuterCamera>>,
) {
    let Projection::Orthographic(projection) = &mut **projection else {
        return;
    };

    for window_resized in resize_message.read() {
        let h_scale = window_resized.height / RES_HEIGHT as f32;
        let v_scale = window_resized.width / RES_WIDTH as f32;
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}
