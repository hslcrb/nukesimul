// src/physics/blast.rs
// 폭압파 전파 물리 시스템 (ECS)

use bevy::prelude::*;
use crate::world::tunnel::{BlastBarrier, section_start, SECTION_LENGTHS};

// ── 리소스 ────────────────────────────────────────────────────
/// 시뮬레이션 전역 물리 상태
#[derive(Resource, Default)]
pub struct BlastState {
    /// 폭발 발생 여부
    pub detonated: bool,
    /// 폭발 후 경과 시간 (초)
    pub elapsed: f32,
    /// 핵출력 (kt)
    pub yield_kt: f32,
    /// 충격파 현재 반경 (m)
    pub shock_radius: f32,
    /// 구간별 최대 과압 기록 (kPa)
    pub section_peak_kpa: [f32; 10],
    /// 구간별 현재 방사선 선량율
    pub section_dose_rate: [f32; 10],
}

impl BlastState {
    pub fn new(yield_kt: f32) -> Self {
        BlastState {
            yield_kt,
            ..default()
        }
    }
}

// ── 폭압파 물리 ───────────────────────────────────────────────
/// 거리 r(m)에서 과압 P(kPa) 산출 – Sadovsky 근사 (터널 구속 보정)
/// P = P0 * (r0/r)^2.5  (터널 채널 효과로 지수 자유공간 3.0보다 낮음)
pub fn overpressure_kpa(yield_kt: f32, radius_m: f32) -> f32 {
    if radius_m < 0.5 {
        return 500_000.0;
    }
    // 스케일된 거리 Z = r / W^(1/3)   [W in kt]
    let w_cbrt = yield_kt.cbrt();
    let z = radius_m / w_cbrt;
    // Kingery-Bulmash 간이식 (kPa)
    let p = 980.0 / z.powf(2.5) + 120.0 / z.powf(1.5) + 14.0;
    p
}

/// 방사선 선량율 (Gy/h) – 초기 핵방사선 (1분 내)
pub fn prompt_dose_rate(yield_kt: f32, radius_m: f32) -> f32 {
    if radius_m < 1.0 {
        return yield_kt * 2.5e7;
    }
    // 역제곱 + 선형흡수 (암반 μ=0.3/m)
    let mu = 0.3_f32;
    let d0 = yield_kt * 2.5e7;
    d0 / (radius_m * radius_m) * (-mu * radius_m).exp()
}

/// 충격파 속도 (m/s) – 과압에서 Rankine-Hugoniot로 마하수 → 속도
pub fn shock_velocity(overpressure_kpa: f32) -> f32 {
    let p0 = 101.325_f32; // 표준대기압 kPa
    let ratio = (overpressure_kpa / p0 + 1.0).sqrt();
    340.0 * ratio.max(1.0) // 음속 340 m/s 기준
}

// ── ECS 시스템 ────────────────────────────────────────────────
pub struct BlastPlugin;

impl Plugin for BlastPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BlastState>()
            .add_systems(Update, (
                update_blast_wave,
                update_barrier_damage,
            ).chain());
    }
}

/// 매 프레임 충격파 반경 및 구간별 과압 갱신
fn update_blast_wave(
    mut state: ResMut<BlastState>,
    time: Res<Time>,
) {
    if !state.detonated { return; }

    state.elapsed += time.delta_seconds();

    // 충격파 현재 위치 (수치 적분 – 간단화)
    let dt = time.delta_seconds();
    let curr_op = overpressure_kpa(state.yield_kt, state.shock_radius.max(0.5));
    let vel = shock_velocity(curr_op);
    state.shock_radius += vel * dt;

    // 구간별 업데이트
    for sec in 1u8..=10 {
        let dist = (section_start(sec) + SECTION_LENGTHS[(sec - 1) as usize] * 0.5)
            .max(0.5);
        let op = overpressure_kpa(state.yield_kt, dist);
        let dr = prompt_dose_rate(state.yield_kt, dist);

        // 감쇠: 방사선은 elapsed에 따라 지수 감쇠 (T½≈1분인 초기방사선)
        let decay = (-0.693 / 60.0 * state.elapsed).exp();

        let idx = (sec - 1) as usize;
        state.section_peak_kpa[idx] = state.section_peak_kpa[idx].max(op);
        state.section_dose_rate[idx] = dr * decay;
    }
}

/// 차단벽 피해 업데이트 (과압 초과 시 시각적 손상)
fn update_barrier_damage(
    mut barriers: Query<(&mut BlastBarrier, &Transform)>,
    state: Res<BlastState>,
) {
    if !state.detonated { return; }

    for (mut barrier, transform) in barriers.iter_mut() {
        let dist = transform.translation.x.max(0.5);
        let op = overpressure_kpa(state.yield_kt, dist);
        let damage_ratio = (op / barrier.max_kpa as f32).min(1.0);
        barrier.damage = barrier.damage.max(damage_ratio);
    }
}
