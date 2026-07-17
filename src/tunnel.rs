// tunnel.rs - 갱도 구조체: 구간, 차단벽, 차단문 정의

use serde::{Deserialize, Serialize};

/// 갱도 구간 번호 (구조도 기준 1~10)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SectionId(pub u8);

/// 차단벽 종류
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BarrierKind {
    /// 핵폭풍·진해 차단벽 (콘크리트 + 모래 복합)
    BlastWave { order: u8 },
}

/// 차단문 (강철)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlastDoor {
    pub section: SectionId,
    pub label: String,
    pub open: bool,
}

impl BlastDoor {
    pub fn new(section: u8, label: &str) -> Self {
        BlastDoor {
            section: SectionId(section),
            label: label.to_string(),
            open: false,
        }
    }
}

/// 차단벽
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Barrier {
    pub section: SectionId,
    pub kind: BarrierKind,
    /// 최대 흡수 가능 과압 (kPa)
    pub max_overpressure_kpa: f64,
    /// 방사선 차폐율 (0.0~1.0)
    pub radiation_shield: f64,
}

impl Barrier {
    pub fn blast_wave(section: u8, order: u8, max_kpa: f64, shield: f64) -> Self {
        Barrier {
            section: SectionId(section),
            kind: BarrierKind::BlastWave { order },
            max_overpressure_kpa: max_kpa,
            radiation_shield: shield,
        }
    }
}

/// 계측 지점 (카메라/센서)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurePoint {
    pub section: SectionId,
    pub label: String,
}

/// 갱도 전체 구조
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tunnel {
    /// 폭발 장치 부터 입구까지 전체 길이 (m)
    pub total_length_m: f64,
    /// 갱도 내경 (m)
    pub diameter_m: f64,
    pub barriers: Vec<Barrier>,
    pub doors: Vec<BlastDoor>,
    pub measure_points: Vec<MeasurePoint>,
    /// 각 구간 길이 (m), index = 구간번호-1
    pub section_length: Vec<f64>,
}

impl Tunnel {
    /// 구조도에 근거한 북한 핵실험장 수평갱도 기본값
    pub fn default_layout() -> Self {
        // 구간 1(폭발부근)~10(입구) 간격 약 50m씩, 총 ~500m
        let section_length = vec![
            40.0, // 1: 폭발장치 ~ 1번 차단벽
            50.0, // 2
            50.0, // 3
            50.0, // 4
            60.0, // 5: 1번 차단문 부근
            60.0, // 6
            60.0, // 7
            60.0, // 8
            60.0, // 9
            60.0, // 10: 입구/10번 차단문
        ];

        let barriers = vec![
            // 1차 핵폭풍·진해 차단벽 (구간 5, 구조도 우측)
            Barrier::blast_wave(5, 1, 3_000.0, 0.70),
            // 2차 핵폭풍·진해 차단벽 (구간 6)
            Barrier::blast_wave(6, 2, 1_500.0, 0.80),
            // 3차 핵폭풍·진해 차단벽 (구간 8, 구조도 좌측)
            Barrier::blast_wave(8, 3, 800.0, 0.85),
        ];

        let doors = vec![
            BlastDoor::new(1, "1번 차단문"),
            BlastDoor::new(10, "10번 차단문"),
        ];

        let measure_points = vec![
            MeasurePoint {
                section: SectionId(4),
                label: "카메라(계측장비) #1".to_string(),
            },
            MeasurePoint {
                section: SectionId(7),
                label: "카메라(계측장비) #2".to_string(),
            },
        ];

        Tunnel {
            total_length_m: section_length.iter().sum(),
            diameter_m: 3.5,
            barriers,
            doors,
            measure_points,
            section_length,
        }
    }

    /// 입구로부터 구간 번호까지의 거리(m) - 폭발측에서 계산
    pub fn distance_from_blast(&self, section: u8) -> f64 {
        let idx = (section as usize).saturating_sub(1);
        self.section_length[..idx].iter().sum()
    }
}
