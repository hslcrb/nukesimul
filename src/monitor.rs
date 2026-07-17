// monitor.rs - 무인관측탑: 갱도 외부 누출방사능 탐지

use crate::physics::SectionState;

/// 무인관측탑 측정 결과
#[derive(Debug, Clone)]
pub struct ObservationTowerReading {
    /// 갱도 밖으로 누출된 방사선량율 (Gy/h)
    pub leaked_dose_rate_gy_per_h: f64,
    /// 누출 판정 (IAEA 기준 0.1 mGy/h = 1e-4 Gy/h 초과 시 경보)
    pub leakage_alert: bool,
    /// 누출 등급
    pub leakage_level: LeakageLevel,
}

#[derive(Debug, Clone)]
pub enum LeakageLevel {
    /// 정상 - 누출 없음
    Normal,
    /// 주의 - 미량 누출
    Caution,
    /// 경보 - 위험 누출
    Alert,
    /// 위기 - 즉각 대피
    Critical,
}

impl LeakageLevel {
    pub fn label(&self) -> &str {
        match self {
            LeakageLevel::Normal   => "정상 (누출 없음)",
            LeakageLevel::Caution  => "주의 (미량 누출)",
            LeakageLevel::Alert    => "경보 (위험 누출)",
            LeakageLevel::Critical => "위기 (즉각 대피)",
        }
    }

    pub fn color_code(&self) -> &str {
        match self {
            LeakageLevel::Normal   => "GREEN",
            LeakageLevel::Caution  => "YELLOW",
            LeakageLevel::Alert    => "ORANGE",
            LeakageLevel::Critical => "RED",
        }
    }
}

pub struct ObservationTower;

impl ObservationTower {
    /// 갱도 입구(구간 10) 상태를 기반으로 외부 누출 측정
    pub fn measure(states: &[SectionState]) -> ObservationTowerReading {
        let entrance = states.last().expect("구간 데이터 없음");

        // 입구에서 탑까지 500m 거리, 개방공간 역제곱 가정
        let tower_dist = 500.0_f64;
        let leaked = entrance.dose_rate_gy_per_h / (tower_dist * tower_dist) * 1e4;

        let level = if leaked < 1e-4 {
            LeakageLevel::Normal
        } else if leaked < 1e-2 {
            LeakageLevel::Caution
        } else if leaked < 1.0 {
            LeakageLevel::Alert
        } else {
            LeakageLevel::Critical
        };

        let alert = matches!(level, LeakageLevel::Alert | LeakageLevel::Critical);

        ObservationTowerReading {
            leaked_dose_rate_gy_per_h: leaked,
            leakage_alert: alert,
            leakage_level: level,
        }
    }
}
