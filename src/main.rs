// src/main.rs - Bevy 3D 핵실험 갱도 시뮬레이션 게임 진입점

mod camera;
mod explosion;
mod physics;
mod ui;
mod world;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use camera::CameraPlugin;
use explosion::{fireball::FireballPlugin, shockwave::ShockwavePlugin};
use physics::{
    blast::{BlastPlugin, BlastState},
    ground_shock::GroundShockPlugin,
    radiation::RadiationPlugin,
};
use ui::hud::HudPlugin;
use world::tunnel::TunnelPlugin;

// ── 앱 상태 ───────────────────────────────────────────────────
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    #[default]
    Menu,
    Simulating,
}

fn main() {
    App::new()
        // ── 기본 Bevy 플러그인 ──
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "☢ 핵실험 갱도 시뮬레이션 v0.2.0".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        // ── Rapier 물리 ──
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // ── 게임 상태 ──
        .init_state::<AppState>()
        // ── 사용자 플러그인 ──
        .add_plugins((
            TunnelPlugin,
            CameraPlugin,
            BlastPlugin,
            RadiationPlugin,
            GroundShockPlugin,
            FireballPlugin,
            ShockwavePlugin,
            HudPlugin,
        ))
        // ── 씬 공통 설정 ──
        .insert_resource(AmbientLight {
            color: Color::srgb(0.15, 0.12, 0.1),
            brightness: 150.0,
        })
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.06)))
        // ── 인터랙션 시스템 ──
        .add_systems(Update, (handle_input, handle_reset))
        .run();
}

/// Space: 기폭  /  R: 리셋
fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<BlastState>,
) {
    if keys.just_pressed(KeyCode::Space) && !state.detonated {
        info!("☢ 핵폭발 기폭!");
        state.detonated = true;
    }
}

fn handle_reset(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut state: ResMut<BlastState>,
    fireball_entities: Query<Entity, With<explosion::fireball::FireballCore>>,
    particle_entities: Query<Entity, With<explosion::fireball::FireballParticle>>,
    shockwave_entities: Query<Entity, With<explosion::shockwave::ShockwaveRing>>,
) {
    if !keys.just_pressed(KeyCode::KeyR) { return; }

    // 상태 초기화
    *state = BlastState::new(state.yield_kt);

    // 폭발 엔티티 정리
    for e in fireball_entities.iter().chain(particle_entities.iter()).chain(shockwave_entities.iter()) {
        commands.entity(e).despawn();
    }
    info!("시뮬레이션 리셋 완료");
}
