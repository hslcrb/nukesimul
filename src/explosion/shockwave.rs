// src/explosion/shockwave.rs
// 충격파 링 메시 시각화

use bevy::prelude::*;
use crate::physics::blast::BlastState;

/// 충격파 링 컴포넌트
#[derive(Component)]
pub struct ShockwaveRing {
    pub spawned: bool,
}

pub struct ShockwavePlugin;

impl Plugin for ShockwavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_shockwave, update_shockwave).chain());
    }
}

fn spawn_shockwave(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    state: Res<BlastState>,
    existing: Query<(), With<ShockwaveRing>>,
) {
    if !state.detonated || !existing.is_empty() { return; }

    // 충격파 구체 (반투명 팽창)
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgba(0.9, 0.7, 0.3, 0.15),
                emissive: Color::srgb(1.5, 0.8, 0.1).into(),
                alpha_mode: AlphaMode::Blend,
                cull_mode: None,
                unlit: true,
                ..default()
            }),
            transform: Transform::from_xyz(3.0, 0.0, 0.0),
            ..default()
        },
        ShockwaveRing { spawned: true },
    ));

    // 2차 충격파 링 (얇은 디스크)
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Torus {
                minor_radius: 0.4,
                major_radius: 1.0,
            }),
            material: materials.add(StandardMaterial {
                base_color: Color::srgba(1.0, 0.9, 0.5, 0.25),
                emissive: Color::srgb(2.0, 1.0, 0.0).into(),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
            transform: Transform::from_xyz(3.0, 0.0, 0.0),
            ..default()
        },
        ShockwaveRing { spawned: true },
    ));
}

fn update_shockwave(
    state: Res<BlastState>,
    mut query: Query<&mut Transform, With<ShockwaveRing>>,
) {
    if !state.detonated { return; }
    let r = state.shock_radius;
    for mut tf in query.iter_mut() {
        tf.scale = Vec3::splat(r);
    }
}
