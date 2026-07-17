// main.rs - 진입점: 대화형 메뉴 + 시뮬레이션 실행

mod device;
mod monitor;
mod physics;
mod render;
mod report;
mod sensor;
mod tunnel;

use device::{DeviceType, NuclearDevice};
use monitor::ObservationTower;
use physics::PhysicsEngine;
use render::TunnelRenderer;
use report::Report;
use sensor::SensorSystem;
use tunnel::Tunnel;

use std::io::{self, BufRead, Write};

fn main() {
    print_banner();

    let tunnel = Tunnel::default_layout();
    let device = interactive_device_select();

    println!();
    println!("  ▶ 시뮬레이션 시작...");
    println!();

    // 물리 시뮬레이션
    let states = PhysicsEngine::simulate(&tunnel, &device);

    // 계측
    let readings = SensorSystem::collect(&tunnel, &states);

    // 관측탑
    let obs = ObservationTower::measure(&states);

    // 렌더링
    TunnelRenderer::draw(&tunnel, &states);

    // 최종 보고서
    Report::print_full(&device, &tunnel, &states, &readings, &obs);
}

/// 핵장치 선택 인터랙션
fn interactive_device_select() -> NuclearDevice {
    println!();
    println!("  ┌─────────────────────────────────────────┐");
    println!("  │         핵폭발 장치 설정                    │");
    println!("  └─────────────────────────────────────────┘");
    println!("  [1] 내폭형 플루토늄");
    println!("  [2] 내폭형 고농축 우라늄 (HEU)");
    println!("  [3] 2단계 증폭형 수소탄");
    print!("  선택 (기본값 1): ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let choice = stdin.lock().lines().next()
        .and_then(|l| l.ok())
        .unwrap_or_default();

    let device_type = match choice.trim() {
        "2" => DeviceType::ImplosionHEU,
        "3" => DeviceType::ThermonuclearBoosted,
        _ => DeviceType::ImplosionPlutonium,
    };

    print!("  핵출력 (kt, 기본 10 kt): ");
    io::stdout().flush().unwrap();
    let yield_str = stdin.lock().lines().next()
        .and_then(|l| l.ok())
        .unwrap_or_default();
    let yield_kt: f64 = yield_str.trim().parse().unwrap_or(10.0);

    print!("  매설 깊이 (m, 기본 300 m): ");
    io::stdout().flush().unwrap();
    let depth_str = stdin.lock().lines().next()
        .and_then(|l| l.ok())
        .unwrap_or_default();
    let depth_m: f64 = depth_str.trim().parse().unwrap_or(300.0);

    println!();
    println!("  ✔ [{type}] {kt} kt, 깊이 {d} m 설정됨",
        type=device_type.name(), kt=yield_kt, d=depth_m);

    NuclearDevice::new(device_type, yield_kt, depth_m)
}

fn print_banner() {
    println!(r"
  ███╗   ██╗██╗   ██╗██╗  ██╗███████╗    ███████╗██╗███╗   ███╗██╗   ██╗██╗
  ████╗  ██║██║   ██║██║ ██╔╝██╔════╝    ██╔════╝██║████╗ ████║██║   ██║██║
  ██╔██╗ ██║██║   ██║█████╔╝ █████╗      ███████╗██║██╔████╔██║██║   ██║██║
  ██║╚██╗██║██║   ██║██╔═██╗ ██╔══╝      ╚════██║██║██║╚██╔╝██║██║   ██║██║
  ██║ ╚████║╚██████╔╝██║  ██╗███████╗    ███████║██║██║ ╚═╝ ██║╚██████╔╝███████╗
  ╚═╝  ╚═══╝ ╚═════╝ ╚═╝  ╚═╝╚══════╝    ╚══════╝╚═╝╚═╝     ╚═╝ ╚═════╝ ╚══════╝
    ");
    println!("  북한 핵실험장 수평갱도 물리 시뮬레이션  v0.1.0");
    println!("  ─────────────────────────────────────────────");
    println!("  ※ 본 프로그램은 교육·연구 목적의 시뮬레이션입니다.");
    println!();
}
