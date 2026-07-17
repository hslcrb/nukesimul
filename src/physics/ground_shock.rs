// src/physics/ground_shock.rs
// 지진파 (지반 충격) 시스템: 갱도 흔들림 효과

use bevy::prelude::*;
use crate::physics::blast::BlastState;

/// 지진파 진동 컴포넌트
#[derive(Component)]
pub struct GroundShakable {
    pub base_pos: Vec3,
    pub phase: f32,
}

#[derive(Resource, Default)]
pub struct GroundShockState {
    pub active: bool,
    pub intensity: f32,    // 0~1
    pub elapsed: f32,
    pub duration: f32,     // 진동 지속 시간 (초)
}

pub struct GroundShockPlugin;

impl Plugin for GroundShockPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GroundShockState>()
            .add_systems(Update, (trigger_ground_shock, apply_ground_shake).chain());
    }
}

/// 폭발 발생 시 지진파 시작
fn trigger_ground_shock(
    blast: Res<BlastState>,
    mut shock: ResMut<GroundShockState>,
) {
    if blast.detonated && !shock.active && blast.elapsed < 0.1 {
        shock.active = true;
        shock.elapsed = 0.0;
        // kt에 비례한 진동 강도 및 지속시간
        shock.intensity = (blast.yield_kt / 10.0).sqrt().min(1.0);
        shock.duration = 3.0 + blast.yield_kt * 0.2;
    }
}

/// 갱도 물체 진동 적용
fn apply_ground_shake(
    mut shock: ResMut<GroundShockState>,
    mut query: Query<(&mut Transform, &GroundShakable)>,
    time: Res<Time>,
) {
    if !shock.active { return; }

    shock.elapsed += time.delta_seconds();
    if shock.elapsed > shock.duration {
        shock.active = false;
        // 원래 위치 복구
        for (mut t, shakable) in query.iter_mut() {
            t.translation = shakable.base_pos;
        }
        return;
    }

    let decay = 1.0 - (shock.elapsed / shock.duration);
    let amp = shock.intensity * decay * 0.08; // 최대 8cm 진동
    let freq = 8.0_f32; // Hz

    let t_sec = time.elapsed_seconds();

    for (mut transform, shakable) in query.iter_mut() {
        let shake_y = (t_sec * freq * std::f32::consts::TAU + shakable.phase).sin() * amp;
        let shake_z = (t_sec * freq * 1.3 * std::f32::consts::TAU + shakable.phase + 1.0).sin() * amp * 0.5;
        transform.translation = shakable.base_pos + Vec3::new(0.0, shake_y, shake_z);
    }
}
