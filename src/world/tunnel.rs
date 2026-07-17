// src/world/tunnel.rs
// 갱도 3D 지오메트리: 구간, 차단벽, 차단문, 계측장비 prop 생성

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// ── 컴포넌트 ─────────────────────────────────────────────────
/// 갱도 구간 번호 (1=폭발측 ~ 10=입구)
#[derive(Component, Debug, Clone, Copy)]
pub struct TunnelSection(pub u8);

/// 차단벽
#[derive(Component, Debug, Clone)]
pub struct BlastBarrier {
    pub order: u8,
    /// 최대 흡수 가능 과압 (kPa)
    pub max_kpa: f64,
    /// 방사선 차폐율 (0~1)
    pub shield: f64,
    /// 현재 구조적 피해 0~1
    pub damage: f32,
}

/// 차단문
#[derive(Component, Debug, Clone)]
pub struct BlastDoor {
    pub section: u8,
    pub open: bool,
}

/// 계측장비 마커
#[derive(Component)]
pub struct MeasurementEquipment {
    pub section: u8,
}

/// 핵폭발 장치 마커
#[derive(Component)]
pub struct NuclearDevice;

/// 무인관측탑 마커
#[derive(Component)]
pub struct ObservationTower;

// ── 갱도 설정 ─────────────────────────────────────────────────
pub const TUNNEL_RADIUS: f32 = 1.75;          // 내경 3.5m → 반지름
pub const SECTION_LENGTHS: [f32; 10] = [
    40.0, 50.0, 50.0, 50.0, 60.0,
    60.0, 60.0, 60.0, 60.0, 60.0,
];
pub const WALL_THICKNESS: f32 = 0.5;
pub const BARRIER_THICKNESS: f32 = 3.0;

/// 구간 1 시작(폭발측)부터 구간 n 시작까지 누적 거리
pub fn section_start(sec: u8) -> f32 {
    let idx = (sec as usize).saturating_sub(1);
    SECTION_LENGTHS[..idx].iter().sum()
}

// ── 월드 생성 플러그인 ───────────────────────────────────────
pub struct TunnelPlugin;

impl Plugin for TunnelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tunnel_world);
    }
}

fn spawn_tunnel_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ── 조명 ──
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::srgb(1.0, 0.95, 0.8),
            illuminance: 8000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ, -0.8, 0.4, 0.0,
        )),
        ..default()
    });

    // 내부 점광원 (갱도 분위기)
    for i in 0..5u8 {
        let x = section_start(i * 2 + 1) + 15.0;
        commands.spawn(PointLightBundle {
            point_light: PointLight {
                color: Color::srgb(0.8, 0.7, 0.5),
                intensity: 50_000.0,
                range: 40.0,
                ..default()
            },
            transform: Transform::from_xyz(x, TUNNEL_RADIUS - 0.3, 0.0),
            ..default()
        });
    }

    // ── 재질 ──
    let rock_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.45, 0.38, 0.30),
        perceptual_roughness: 0.9,
        metallic: 0.0,
        ..default()
    });
    let concrete_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.58, 0.55),
        perceptual_roughness: 0.85,
        ..default()
    });
    let steel_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.42, 0.45),
        perceptual_roughness: 0.3,
        metallic: 0.9,
        ..default()
    });
    let device_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.8, 0.3),
        emissive: Color::srgb(0.0, 2.0, 0.5).into(),
        perceptual_roughness: 0.2,
        metallic: 0.8,
        ..default()
    });
    let equipment_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.75, 0.1),
        perceptual_roughness: 0.4,
        metallic: 0.6,
        ..default()
    });
    let tower_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.9, 0.85),
        perceptual_roughness: 0.5,
        ..default()
    });

    let total_length: f32 = SECTION_LENGTHS.iter().sum();

    // ── 갱도 바닥 ──
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(total_length, 0.3, TUNNEL_RADIUS * 2.0)),
            material: rock_mat.clone(),
            transform: Transform::from_xyz(total_length / 2.0, -TUNNEL_RADIUS - 0.15, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(total_length / 2.0, 0.15, TUNNEL_RADIUS),
    ));

    // ── 갱도 천장 ──
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(total_length, 0.3, TUNNEL_RADIUS * 2.0)),
        material: rock_mat.clone(),
        transform: Transform::from_xyz(total_length / 2.0, TUNNEL_RADIUS + 0.15, 0.0),
        ..default()
    });

    // ── 갱도 좌우 벽 ──
    for side in [-1.0_f32, 1.0_f32] {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(total_length, TUNNEL_RADIUS * 2.0, 0.3)),
                material: rock_mat.clone(),
                transform: Transform::from_xyz(
                    total_length / 2.0,
                    0.0,
                    side * (TUNNEL_RADIUS + 0.15),
                ),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(total_length / 2.0, TUNNEL_RADIUS, 0.15),
        ));
    }

    // ── 차단벽 3개 (구간 5, 6, 8) ──
    let barriers_def: [(u8, u8, f64, f64); 3] = [
        (5, 1, 3_000.0, 0.70),
        (6, 2, 1_500.0, 0.80),
        (8, 3,   800.0, 0.85),
    ];
    for (sec, order, max_kpa, shield) in barriers_def {
        let x = section_start(sec) + SECTION_LENGTHS[(sec - 1) as usize] * 0.5;
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(
                    BARRIER_THICKNESS,
                    TUNNEL_RADIUS * 2.0,
                    TUNNEL_RADIUS * 2.0,
                )),
                material: concrete_mat.clone(),
                transform: Transform::from_xyz(x, 0.0, 0.0),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(BARRIER_THICKNESS / 2.0, TUNNEL_RADIUS, TUNNEL_RADIUS),
            TunnelSection(sec),
            BlastBarrier { order, max_kpa, shield, damage: 0.0 },
        ));

        // 라벨 (이미터 역할 – 실제 텍스트는 UI에서)
        // 통기 구멍 (차단벽 중앙 작은 구멍 표현 – 별도 컷아웃 생략, 게임플레이상 충돌체만 유지)
    }

    // ── 차단문 (구간 1, 10) ──
    for (sec, label) in [(1u8, "1번 차단문"), (10u8, "10번 차단문")] {
        let x = section_start(sec) + 2.0;
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(
                    0.4,
                    TUNNEL_RADIUS * 2.0,
                    TUNNEL_RADIUS * 2.0,
                )),
                material: steel_mat.clone(),
                transform: Transform::from_xyz(x, 0.0, 0.0),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(0.2, TUNNEL_RADIUS, TUNNEL_RADIUS),
            TunnelSection(sec),
            BlastDoor { section: sec, open: false },
        ));
        let _ = label; // 향후 UI 라벨에서 사용
    }

    // ── 핵폭발 장치 (구간 1, 폭발단 중앙) ──
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(1.2)),
            material: device_mat,
            transform: Transform::from_xyz(3.0, -TUNNEL_RADIUS + 1.5, 0.0),
            ..default()
        },
        NuclearDevice,
    ));

    // ── 계측장비 (구간 4, 7) ──
    for sec in [4u8, 7u8] {
        let x = section_start(sec) + 10.0;
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.4, 0.6, 0.4)),
                material: equipment_mat.clone(),
                transform: Transform::from_xyz(x, -TUNNEL_RADIUS + 0.5, TUNNEL_RADIUS * 0.6),
                ..default()
            },
            MeasurementEquipment { section: sec },
            TunnelSection(sec),
        ));
        // 카메라 렌즈 표현
        commands.spawn(PbrBundle {
            mesh: meshes.add(Sphere::new(0.12)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.05, 0.05, 0.15),
                emissive: Color::srgb(0.0, 0.0, 0.5).into(),
                ..default()
            }),
            transform: Transform::from_xyz(x, -TUNNEL_RADIUS + 0.7, TUNNEL_RADIUS * 0.6 + 0.21),
            ..default()
        });
    }

    // ── 무인관측탑 (갱도 외부, 입구 위) ──
    let entrance_x = total_length + 5.0;
    // 탑 몸체
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(3.0, 12.0, 3.0)),
            material: tower_mat.clone(),
            transform: Transform::from_xyz(entrance_x, 6.0, -8.0),
            ..default()
        },
        ObservationTower,
    ));
    // 탑 창문
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.5, 1.0, 0.1)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgba(0.4, 0.6, 0.9, 0.5),
            alpha_mode: AlphaMode::Blend,
            emissive: Color::srgb(0.0, 0.3, 0.6).into(),
            ..default()
        }),
        transform: Transform::from_xyz(entrance_x, 10.5, -6.55),
        ..default()
    });

    // ── 지형 (갱도 주변 암반) ──
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(total_length + 30.0, 30.0, 40.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.35, 0.28, 0.22),
                perceptual_roughness: 1.0,
                ..default()
            }),
            transform: Transform::from_xyz(
                total_length / 2.0,
                TUNNEL_RADIUS + 15.0,
                0.0,
            ),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(
            (total_length + 30.0) / 2.0,
            15.0,
            20.0,
        ),
    ));
}
