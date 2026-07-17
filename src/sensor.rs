// sensor.rs - 계측장비/카메라: 구간별 측정값 수집

use crate::physics::SectionState;
use crate::tunnel::Tunnel;

/// 계측 장비가 수집한 데이터
#[derive(Debug, Clone)]
pub struct SensorReading {
    pub point_label: String,
    pub section: u8,
    pub distance_m: f64,
    pub overpressure_kpa: f64,
    pub dose_rate_gy_per_h: f64,
    /// 센서 손상 여부 (과압 > 500 kPa 이면 손상)
    pub sensor_damaged: bool,
}

pub struct SensorSystem;

impl SensorSystem {
    pub fn collect(
        tunnel: &Tunnel,
        states: &[SectionState],
    ) -> Vec<SensorReading> {
        tunnel
            .measure_points
            .iter()
            .map(|pt| {
                let sec = pt.section.0;
                let state = states
                    .iter()
                    .find(|s| s.section == sec)
                    .expect("계측 지점에 해당하는 구간 없음");

                let damaged = state.overpressure_kpa > 500.0;

                SensorReading {
                    point_label: pt.label.clone(),
                    section: sec,
                    distance_m: state.distance_m,
                    overpressure_kpa: if damaged { 0.0 } else { state.overpressure_kpa },
                    dose_rate_gy_per_h: if damaged { 0.0 } else { state.dose_rate_gy_per_h },
                    sensor_damaged: damaged,
                }
            })
            .collect()
    }
}
