// physics.rs - 폭압파 전파 및 방사선 감쇠 물리 모델

use crate::tunnel::Tunnel;
use crate::device::NuclearDevice;

/// 구간별 물리 상태
#[derive(Debug, Clone)]
pub struct SectionState {
    pub section: u8,
    /// 폭발 장치로부터 거리 (m)
    pub distance_m: f64,
    /// 현재 과압 (kPa)
    pub overpressure_kpa: f64,
    /// 현재 방사선 선량율 (Gy/h)
    pub dose_rate_gy_per_h: f64,
    /// 이 구간에 차단벽이 있으면 적용 정보
    pub barrier_applied: Option<String>,
}

pub struct PhysicsEngine;

impl PhysicsEngine {
    /// 거리에 따른 과압 감쇠 (자유공간 경험식 - 터널 증폭 보정 포함)
    /// P(r) = P0 * (r0 / r)^2.5  (터널 구속효과 지수 완화)
    fn overpressure_at(p0: f64, r0: f64, r: f64) -> f64 {
        if r <= r0 {
            return p0;
        }
        p0 * (r0 / r).powf(2.5)
    }

    /// 거리에 따른 방사선 감쇠 (역제곱 + 선형흡수)
    fn dose_rate_at(d0: f64, r: f64, mu: f64) -> f64 {
        // μ: 암반 선흡수계수 (granite ~0.3 /m)
        if r <= 1.0 {
            return d0;
        }
        d0 / (r * r) * (-mu * r).exp()
    }

    /// 전체 갱도 구간별 시뮬레이션 실행
#[allow(unused_assignments)]
    pub fn simulate(tunnel: &Tunnel, device: &NuclearDevice) -> Vec<SectionState> {
        let a = std::f64::consts::PI * (tunnel.diameter_m / 2.0).powi(2);
        let p0 = device.initial_overpressure_kpa(a);
        let d0 = device.initial_dose_rate_gy_per_h();
        let mu = 0.3_f64; // granite

        let r0 = 1.0_f64; // 기준 거리 1m

        let mut states: Vec<SectionState> = Vec::new();
        let mut current_p: f64;
        let mut current_d: f64;

        for sec in 1u8..=10 {
            let dist = tunnel.distance_from_blast(sec).max(r0);

            // 거리에 따른 자연 감쇠
            let p_dist = Self::overpressure_at(p0, r0, dist);
            let d_dist = Self::dose_rate_at(d0, dist, mu);

            // 차단벽 적용 (해당 구간 차단벽이 있으면)
            let barrier_info = tunnel.barriers.iter().find(|b| b.section.0 == sec);
            let (p_after, d_after, barrier_label) = if let Some(bar) = barrier_info {
                let p_absorbed = (p_dist - bar.max_overpressure_kpa).max(0.0);
                let d_shielded = d_dist * (1.0 - bar.radiation_shield);
                let label = match &bar.kind {
                    crate::tunnel::BarrierKind::BlastWave { order } =>
                        format!("{}차 핵폭풍·진해 차단벽 적용", order),
                };
                (p_absorbed, d_shielded.min(d_dist), Some(label))
            } else {
                (p_dist, d_dist, None)
            };

            current_p = p_after;
            current_d = d_after;

            states.push(SectionState {
                section: sec,
                distance_m: dist,
                overpressure_kpa: current_p,
                dose_rate_gy_per_h: current_d,
                barrier_applied: barrier_label,
            });
        }
        states
    }
}
