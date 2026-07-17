// device.rs - 핵폭발 장치 파라미터

use serde::{Deserialize, Serialize};

/// 핵장치 종류
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    /// 내폭형 플루토늄
    ImplosionPlutonium,
    /// 내폭형 고농축 우라늄
    ImplosionHEU,
    /// 2단계 수소탄
    ThermonuclearBoosted,
}

impl DeviceType {
    pub fn name(&self) -> &str {
        match self {
            DeviceType::ImplosionPlutonium => "내폭형 플루토늄",
            DeviceType::ImplosionHEU => "내폭형 고농축 우라늄(HEU)",
            DeviceType::ThermonuclearBoosted => "2단계 증폭형 수소탄",
        }
    }
}

/// 핵폭발 장치
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuclearDevice {
    pub device_type: DeviceType,
    /// 핵출력 (킬로톤, kt TNT 당량)
    pub yield_kt: f64,
    /// 폭발 깊이 (지표 아래 m)
    pub depth_m: f64,
}

impl NuclearDevice {
    pub fn new(device_type: DeviceType, yield_kt: f64, depth_m: f64) -> Self {
        NuclearDevice { device_type, yield_kt, depth_m }
    }

    /// 폭발 즉각 발생 에너지 (J)
    pub fn total_energy_joules(&self) -> f64 {
        // 1 kt TNT ≈ 4.184e12 J
        self.yield_kt * 4.184e12
    }

    /// 초기 충격파 최대 과압 (kPa) - 폭발 주변 갱도 진입부
    /// 갱도 단면적(m²)과 에너지로 근사
    pub fn initial_overpressure_kpa(&self, tunnel_area_m2: f64) -> f64 {
        // 매우 단순화: E / (A * L_ref) * 0.5 단위 계수
        // 실제 값은 ROCK SHOCK 모델 필요, 여기서는 교육용 스케일
        let energy = self.total_energy_joules();
        let l_ref = 1.0; // 1m 기준
        (energy / (tunnel_area_m2 * l_ref) * 1e-3).min(500_000.0)
    }

    /// 초기 방사선 선량율 (Gy/h, 폭발 직후 1m 기준)
    pub fn initial_dose_rate_gy_per_h(&self) -> f64 {
        // 간이 근사: yield_kt당 초기 선량율 스케일
        self.yield_kt * 2.5e6
    }
}
