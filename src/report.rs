// report.rs - 실험 결과 최종 리포트 출력

use crate::device::NuclearDevice;
use crate::monitor::ObservationTowerReading;
use crate::physics::SectionState;
use crate::sensor::SensorReading;
use crate::tunnel::Tunnel;

pub struct Report;

impl Report {
    pub fn print_full(
        device: &NuclearDevice,
        tunnel: &Tunnel,
        states: &[SectionState],
        readings: &[SensorReading],
        obs: &ObservationTowerReading,
    ) {
        println!();
        println!("╔══════════════════════════════════════════════════════════╗");
        println!("║         핵실험 시뮬레이션 결과 보고서                        ║");
        println!("╚══════════════════════════════════════════════════════════╝");
        println!();

        // 1. 장치 정보
        println!("▌ 핵폭발 장치");
        println!("  - 종류  : {}", device.device_type.name());
        println!("  - 출력  : {:.2} kt TNT 당량", device.yield_kt);
        println!("  - 매설깊이: {:.0} m", device.depth_m);
        let energy = device.total_energy_joules();
        println!("  - 총 에너지: {:.3e} J", energy);
        println!();

        // 2. 갱도 구조
        println!("▌ 갱도 구조");
        println!("  - 전체 길이: {:.0} m", tunnel.total_length_m);
        println!("  - 내경    : {:.1} m", tunnel.diameter_m);
        println!("  - 차단벽  : {}개", tunnel.barriers.len());
        println!("  - 차단문  : {}개", tunnel.doors.len());
        println!();

        // 3. 구간별 물리 상태
        println!("▌ 구간별 폭압·방사선 전파");
        println!(
            "  {:>4}  {:>8}  {:>16}  {:>18}  {}",
            "구간", "거리(m)", "과압(kPa)", "선량율(Gy/h)", "차단벽"
        );
        println!("  {}", "─".repeat(80));
        for s in states {
            let bar = s.barrier_applied.as_deref().unwrap_or("-");
            println!(
                "  {:>4}  {:>8.1}  {:>16.3}  {:>18.3e}  {}",
                s.section, s.distance_m, s.overpressure_kpa, s.dose_rate_gy_per_h, bar
            );
        }
        println!();

        // 4. 계측장비 측정값
        println!("▌ 계측장비(카메라) 측정값");
        for r in readings {
            if r.sensor_damaged {
                println!("  [{}] 구간 {} - 센서 손상 (과압 초과)", r.point_label, r.section);
            } else {
                println!(
                    "  [{}] 구간 {} | 과압={:.2} kPa | 선량율={:.3e} Gy/h",
                    r.point_label, r.section, r.overpressure_kpa, r.dose_rate_gy_per_h
                );
            }
        }
        println!();

        // 5. 누출 감지탑
        println!("▌ 무인관측탑 (누출방사능 탐지)");
        println!(
            "  - 외부 선량율 : {:.3e} Gy/h",
            obs.leaked_dose_rate_gy_per_h
        );
        println!("  - 경보 상태   : {}", obs.leakage_level.label());
        if obs.leakage_alert {
            println!("  ⚠ 경보 발령! 즉각 안전조치 필요.");
        } else {
            println!("  ✓ 정상 범위. 외부 누출 없음.");
        }
        println!();
        println!("══════════════════════════════════════════════════════════════");
    }
}
