// src/explosion/fireball.rs
// 핵폭발 화구: 파티클 + 구체 팽창 + 버섯구름 표현

use bevy::prelude::*;
use crate::physics::blast::BlastState;
use rand::Rng;

/// 화구 파티클 컴포넌트
#[derive(Component)]
pub struct FireballParticle {
    pub velocity: Vec3,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub base_scale: f32,
}

/// 중심 화구 메시
#[derive(Component)]
pub struct FireballCore;

/// 버섯운 stem
#[derive(Component)]
pub struct MushroomStem;

pub struct FireballPlugin;

impl Plugin for FireballPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                spawn_fireball_on_detonate,
                update_fireball_core,
                update_fireball_particles,
                cleanup_dead_particles,
            ));
    }
}

fn spawn_fireball_on_detonate(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    state: Res<BlastState>,
    existing: Query<(), With<FireballCore>>,
) {
    if !state.detonated || !existing.is_empty() { return; }

    let origin = Vec3::new(3.0, 0.0, 0.0);
    let mut rng = rand::thread_rng();

    // 중심 화구
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.5, 0.0),
                emissive: Color::rgb(8.0, 3.0, 0.0),
                unlit: true,
                ..default()
            }),
            transform: Transform::from_translation(origin),
            ..default()
        },
        FireballCore,
    ));

    // 폭발 点光源
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::rgb(1.0, 0.7, 0.2),
            intensity: 5_000_000.0,
            range: 200.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(origin),
        ..default()
    });

    // 파티클 스폰 (화구 파편 100개)
    for _ in 0..100 {
        let theta: f32 = rng.gen_range(0.0..std::f32::consts::TAU);
        let phi: f32 = rng.gen_range(0.0..std::f32::consts::PI);
        let speed: f32 = rng.gen_range(20.0..80.0);
        let vel = Vec3::new(
            speed * phi.sin() * theta.cos(),
            speed * phi.cos().abs() * 1.5, // 위쪽 편향 (버섯 효과)
            speed * phi.sin() * theta.sin(),
        );

        let scale = rng.gen_range(0.2_f32..0.8);
        let r: f32 = rng.gen_range(0.8..1.0);
        let g: f32 = rng.gen_range(0.2..0.6);
        let lifetime = rng.gen_range(1.5..4.0);

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(scale)),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(r, g, 0.0),
                    emissive: Color::rgb(r * 3.0, g, 0.0),
                    unlit: true,
                    ..default()
                }),
                transform: Transform::from_translation(origin),
                ..default()
            },
            FireballParticle {
                velocity: vel,
                lifetime,
                max_lifetime: lifetime,
                base_scale: scale,
            },
        ));
    }

    // 연기 파티클 (회색, 느리게 상승)
    for _ in 0..60 {
        let theta: f32 = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed: f32 = rng.gen_range(3.0..12.0);
        let vel = Vec3::new(
            speed * theta.cos() * 0.3,
            speed * rng.gen_range(1.0..2.5),
            speed * theta.sin() * 0.3,
        );
        let grey: f32 = rng.gen_range(0.3..0.7);
        let lifetime = rng.gen_range(5.0..10.0);
        let scale = rng.gen_range(1.0_f32..3.0);

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(scale)),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgba(grey, grey, grey, 0.6),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    ..default()
                }),
                transform: Transform::from_translation(origin),
                ..default()
            },
            FireballParticle {
                velocity: vel,
                lifetime,
                max_lifetime: lifetime,
                base_scale: scale,
            },
        ));
    }
}

fn update_fireball_core(
    state: Res<BlastState>,
    mut query: Query<&mut Transform, With<FireballCore>>,
) {
    if !state.detonated { return; }
    let t = state.elapsed;
    // 초기 급팽창 후 수렴
    let radius = if t < 0.5 {
        t * 60.0
    } else {
        30.0 * (-0.3 * (t - 0.5)).exp() + 5.0
    };
    for mut tf in query.iter_mut() {
        tf.scale = Vec3::splat(radius);
    }
}

fn update_fireball_particles(
    mut query: Query<(&mut Transform, &mut FireballParticle)>,
    time: Res<Time>,
    state: Res<BlastState>,
) {
    if !state.detonated { return; }
    let dt = time.delta_seconds();
    let gravity = Vec3::new(0.0, -4.0, 0.0); // 약한 중력 (고열 상승력으로 상쇄)

    for (mut tf, mut p) in query.iter_mut() {
        p.lifetime -= dt;
        if p.lifetime <= 0.0 { continue; }

        p.velocity += gravity * dt;
        tf.translation += p.velocity * dt;

        // 파티클 크기 감쇠
        let life_ratio = p.lifetime / p.max_lifetime;
        tf.scale = Vec3::splat(p.base_scale * life_ratio.sqrt());
    }
}

fn cleanup_dead_particles(
    mut commands: Commands,
    query: Query<(Entity, &FireballParticle)>,
) {
    for (entity, p) in query.iter() {
        if p.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
