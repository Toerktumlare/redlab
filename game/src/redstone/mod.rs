use bevy::prelude::*;

pub mod junctions;
pub mod ticks;

mod scheduler;
pub use scheduler::{NotifyDelay, Scheduler, Tick};

#[derive(Resource, Default, Debug)]
pub struct GlobalTick {
    counter: Tick,
    is_running: bool,
}

impl GlobalTick {
    pub fn tick(&mut self) {
        self.counter += 1;
    }

    pub fn read(&self) -> Tick {
        self.counter
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn start(&mut self) {
        self.is_running = true;
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }
}

pub fn spawn_cross(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    position: IVec3,
    material: Handle<StandardMaterial>,
) -> Entity {
    let vertical = get_mesh(JunctionUVs::Vertical);
    let horizontal = get_mesh(JunctionUVs::Horizontal);
    let dot = get_mesh(JunctionUVs::Dot);
    let mesh_array = [meshes.add(vertical), meshes.add(horizontal)];

    let dot_entity = spawn_redstone_mesh(meshes.add(dot), commands, position, material.clone());

    for mesh in mesh_array {
        commands.entity(dot_entity).with_children(|parent| {
            parent.spawn((
                Name::new("Cross"),
                Mesh3d(mesh),
                MeshMaterial3d(material.clone()),
            ));
        });
    }
    dot_entity
}

pub fn spawn_tcross_north(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    position: IVec3,
    material: Handle<StandardMaterial>,
) -> Entity {
    let half_line = get_mesh(JunctionUVs::HLineN);
    let horizontal = get_mesh(JunctionUVs::Horizontal);
    let dot = get_mesh(JunctionUVs::Dot);
    let pos = position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01);

    commands
        .spawn((
            Name::new("Updated TCross"),
            Transform::from_translation(pos)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            GlobalTransform::default(),
            Pickable {
                is_hoverable: true,
                ..default()
            },
            Mesh3d(meshes.add(horizontal)),
            MeshMaterial3d(material.clone()),
            Visibility::Visible,
            children![
                (
                    Mesh3d(meshes.add(half_line)),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(Vec3::Y * 0.25)
                ),
                (Mesh3d(meshes.add(dot)), MeshMaterial3d(material.clone()),)
            ],
        ))
        .id()
}

pub fn spawn_tcross_south(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    position: IVec3,
    material: Handle<StandardMaterial>,
) -> Entity {
    let half_line = get_mesh(JunctionUVs::HLineS);
    let horizontal = get_mesh(JunctionUVs::Horizontal);
    let dot = get_mesh(JunctionUVs::Dot);
    let pos = position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01);

    commands
        .spawn((
            Name::new("TCross South"),
            Transform::from_translation(pos)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            GlobalTransform::default(),
            Pickable {
                is_hoverable: true,
                ..default()
            },
            Mesh3d(meshes.add(horizontal)),
            MeshMaterial3d(material.clone()),
            Visibility::Visible,
            children![
                (
                    Mesh3d(meshes.add(half_line)),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(Vec3::Y * -0.25)
                ),
                (Mesh3d(meshes.add(dot)), MeshMaterial3d(material.clone()),)
            ],
        ))
        .id()
}

pub fn spawn_tcross_east(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    position: IVec3,
    material: Handle<StandardMaterial>,
) -> Entity {
    let half_line = get_mesh(JunctionUVs::HLineE);
    let vertical = get_mesh(JunctionUVs::Vertical);
    let dot = get_mesh(JunctionUVs::Dot);
    let pos = position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01);

    commands
        .spawn((
            Name::new("TCross East"),
            Transform::from_translation(pos)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            GlobalTransform::default(),
            Pickable {
                is_hoverable: true,
                ..default()
            },
            Mesh3d(meshes.add(vertical)),
            MeshMaterial3d(material.clone()),
            Visibility::Visible,
            children![
                (
                    Mesh3d(meshes.add(half_line)),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(Vec3::X * 0.25)
                ),
                (Mesh3d(meshes.add(dot)), MeshMaterial3d(material.clone()),)
            ],
        ))
        .id()
}

pub fn spawn_tcross_west(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    position: IVec3,
    material: Handle<StandardMaterial>,
) -> Entity {
    let half_line = get_mesh(JunctionUVs::HLineW);
    let vertical = get_mesh(JunctionUVs::Vertical);
    let dot = get_mesh(JunctionUVs::Dot);
    let pos = position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01);

    commands
        .spawn((
            Name::new("TCross West"),
            Transform::from_translation(pos)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            GlobalTransform::default(),
            Pickable {
                is_hoverable: true,
                ..default()
            },
            Mesh3d(meshes.add(vertical)),
            MeshMaterial3d(material.clone()),
            Visibility::Visible,
            children![
                (
                    Mesh3d(meshes.add(half_line)),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(Vec3::X * -0.25)
                ),
                (Mesh3d(meshes.add(dot)), MeshMaterial3d(material.clone()),)
            ],
        ))
        .id()
}

pub fn spawn_corner_nw(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    position: IVec3,
    material: Handle<StandardMaterial>,
) -> Entity {
    let half_line = get_mesh(JunctionUVs::HLineW);
    let half_line2 = get_mesh(JunctionUVs::HLineS);
    let dot = get_mesh(JunctionUVs::Dot);
    let pos = position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01);

    commands
        .spawn((
            Name::new("NW"),
            Transform::from_translation(pos)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            GlobalTransform::default(),
            Pickable {
                is_hoverable: true,
                ..default()
            },
            Visibility::Visible,
            Mesh3d(meshes.add(dot)),
            MeshMaterial3d(material.clone()),
            children![
                (
                    Mesh3d(meshes.add(half_line)),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(Vec3::X * -0.25),
                ),
                (
                    Mesh3d(meshes.add(half_line2)),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(Vec3::Y * -0.25),
                ),
            ],
        ))
        .id()
}

pub fn spawn_corner_ne(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    position: IVec3,
    material: Handle<StandardMaterial>,
) -> Entity {
    let half_line = get_mesh(JunctionUVs::HLineE);
    let half_line2 = get_mesh(JunctionUVs::HLineS);
    let dot = get_mesh(JunctionUVs::Dot);
    let pos = position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01);

    commands
        .spawn((
            Name::new("NE"),
            Transform::from_translation(pos)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            GlobalTransform::default(),
            Pickable {
                is_hoverable: true,
                ..default()
            },
            Visibility::Visible,
            Mesh3d(meshes.add(dot)),
            MeshMaterial3d(material.clone()),
            children![
                (
                    Mesh3d(meshes.add(half_line)),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(Vec3::X * 0.25),
                ),
                (
                    Mesh3d(meshes.add(half_line2)),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(Vec3::Y * -0.25),
                ),
            ],
        ))
        .id()
}

pub fn spawn_corner_se(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    position: IVec3,
    material: Handle<StandardMaterial>,
) -> Entity {
    let half_line = get_mesh(JunctionUVs::HLineE);
    let half_line2 = get_mesh(JunctionUVs::HLineN);
    let dot = get_mesh(JunctionUVs::Dot);
    let pos = position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01);

    commands
        .spawn((
            Name::new("SE"),
            Transform::from_translation(pos)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            GlobalTransform::default(),
            Pickable {
                is_hoverable: true,
                ..default()
            },
            Visibility::Visible,
            Mesh3d(meshes.add(dot)),
            MeshMaterial3d(material.clone()),
            children![
                (
                    Mesh3d(meshes.add(half_line)),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(Vec3::X * 0.25),
                ),
                (
                    Mesh3d(meshes.add(half_line2)),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(Vec3::Y * 0.25),
                ),
            ],
        ))
        .id()
}

pub fn spawn_corner_sw(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    position: IVec3,
    material: Handle<StandardMaterial>,
) -> Entity {
    let half_line = get_mesh(JunctionUVs::HLineW);
    let half_line2 = get_mesh(JunctionUVs::HLineS);
    let dot = get_mesh(JunctionUVs::Dot);
    let pos = position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01);

    commands
        .spawn((
            Name::new("SW"),
            Transform::from_translation(pos)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            GlobalTransform::default(),
            Pickable {
                is_hoverable: true,
                ..default()
            },
            Visibility::Visible,
            Mesh3d(meshes.add(dot)),
            MeshMaterial3d(material.clone()),
            children![
                (
                    Mesh3d(meshes.add(half_line)),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(Vec3::X * -0.25),
                ),
                (
                    Mesh3d(meshes.add(half_line2)),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(Vec3::Y * 0.25),
                ),
            ],
        ))
        .id()
}
pub fn spawn_redstone_mesh(
    mesh: Handle<Mesh>,
    commands: &mut Commands,
    position: IVec3,
    material: Handle<StandardMaterial>,
) -> Entity {
    commands
        .spawn((
            Name::new("Redstone"),
            Mesh3d(mesh),
            MeshMaterial3d(material.clone()),
            Pickable {
                is_hoverable: true,
                ..default()
            },
            Transform::from_translation(position.as_vec3() - (Vec3::Y * 0.5) + (Vec3::Y * 0.01))
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ))
        .id()
}

pub enum JunctionUVs {
    Vertical,
    Horizontal,
    HLineN,
    HLineS,
    HLineW,
    HLineE,
    Dot,
}

/// TODO: generate and place in precomputed meshes
pub fn get_mesh(junction_uvs: JunctionUVs) -> Mesh {
    match junction_uvs {
        JunctionUVs::Vertical => {
            let mut mesh = Mesh::from(Rectangle::new(1., 1.));
            mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);
            let uvs = vec![
                [0.2, 0.3], // top-right
                [0.1, 0.3], // top-left
                [0.1, 0.4], // bottom-left
                [0.2, 0.4], // bottom-right
            ];
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            mesh
        }
        JunctionUVs::Horizontal => {
            let mut mesh = Mesh::from(Rectangle::new(1., 1.));
            mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);
            let uvs = vec![
                [0.2, 0.4], // top-right
                [0.2, 0.3], // top-left
                [0.1, 0.3], // bottom-left
                [0.1, 0.4], // bottom-right
            ];
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            mesh
        }
        JunctionUVs::HLineN => {
            let mut mesh = Mesh::from(Rectangle::new(1., 0.5));
            mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);
            let uvs = vec![
                [0.2, 0.3],  // top-right
                [0.1, 0.3],  // top-left
                [0.1, 0.35], // bottom-left
                [0.2, 0.35], // bottom-right
            ];
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            mesh
        }
        JunctionUVs::HLineS => {
            let mut mesh = Mesh::from(Rectangle::new(1., 0.5));
            mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);
            let uvs = vec![
                [0.2, 0.35], // top-right
                [0.1, 0.35], // top-left
                [0.1, 0.4],  // bottom-left
                [0.2, 0.4],  // bottom-right
            ];
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            mesh
        }
        JunctionUVs::HLineW => {
            let mut mesh = Mesh::from(Rectangle::new(0.5, 1.));
            mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);
            let uvs = vec![
                [0.2, 0.35], // top-right
                [0.2, 0.3],  // top-left
                [0.1, 0.3],  // bottom-left
                [0.1, 0.35], // bottom-right
            ];
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            mesh
        }
        JunctionUVs::HLineE => {
            let mut mesh = Mesh::from(Rectangle::new(0.5, 1.));
            mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);
            let uvs = vec![
                [0.2, 0.4],  // top-right
                [0.2, 0.35], // top-left
                [0.1, 0.35], // bottom-left
                [0.1, 0.4],  // bottom-right
            ];
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            mesh
        }
        JunctionUVs::Dot => {
            let mut mesh = Mesh::from(Rectangle::new(1., 1.));
            mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);
            let uvs = vec![
                [0.1, 0.3], // top-right
                [0.0, 0.3], // top-left
                [0.0, 0.4], // bottom-left
                [0.1, 0.4], // bottom-right
            ];
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            mesh
        }
    }
}
