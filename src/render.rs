// render.rs - TUI ASCII 갱도 단면도 렌더러

use crate::physics::SectionState;
use crate::tunnel::Tunnel;

pub struct TunnelRenderer;

impl TunnelRenderer {
    /// 갱도 단면도를 ASCII 아트로 출력
    /// 구간 1(폭발측 우측) → 10(입구 좌측)
    pub fn draw(_tunnel: &Tunnel, states: &[SectionState]) {
        println!();
        println!("  [ 북한 핵실험장 수평갱도 시뮬레이션 단면도 ]");
        println!();
        println!("  입구                                            폭발장치");
        println!("  [10] ──┤ ─── [8] ──┤ ─── [6] ──┤ ─── [5] ─── ☢");
        println!("         차단벽3      차단벽2      차단벽1");
        println!();

        // 진행 바 (압력 시각화)
        println!("  과압 (kPa) 시각화:");
        for s in states.iter().rev() {
            let bar_max: usize = 50;
            // log 스케일 시각화
            let p = s.overpressure_kpa;
            let filled = if p <= 0.0 {
                0
            } else {
                let log_p = p.log10().max(0.0);
                let log_max = 6.0_f64; // 1_000_000 kPa 최대
                ((log_p / log_max) * bar_max as f64) as usize
            };
            let empty = bar_max.saturating_sub(filled);

            let bar_char = if p > 3000.0 { '█' } else if p > 500.0 { '▓' } else if p > 10.0 { '▒' } else { '░' };

            let bar: String = std::iter::repeat(bar_char).take(filled)
                .chain(std::iter::repeat(' ').take(empty))
                .collect();

            let tag = match &s.barrier_applied {
                Some(b) => format!(" ← {}", b),
                None => String::new(),
            };
            println!(
                "  구간{:>2} [{:<50}] {:>10.1} kPa{}",
                s.section, bar, p, tag
            );
        }
        println!();

        // 방사선 선량율
        println!("  방사선 선량율 (Gy/h, log 스케일):");
        for s in states.iter().rev() {
            let d = s.dose_rate_gy_per_h;
            let bar_max = 40usize;
            let filled = if d <= 0.0 {
                0
            } else {
                let log_d = d.log10().max(0.0);
                let log_max = 14.0_f64;
                ((log_d / log_max) * bar_max as f64) as usize
            };
            let empty = bar_max.saturating_sub(filled);
            let rc: String = std::iter::repeat('◆').take(filled)
                .chain(std::iter::repeat('·').take(empty))
                .collect();
            println!("  구간{:>2} [{:<40}] {:>10.3e} Gy/h", s.section, rc, d);
        }
        println!();
    }
}
