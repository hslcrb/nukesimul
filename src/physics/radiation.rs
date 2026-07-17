// src/physics/radiation.rs
// 방사선 전파 및 낙진 시스템

use bevy::prelude::*;
use crate::physics::blast::BlastState;

/// 방사선 존 컴포넌트 (구간에 부착)
#[derive(Component, Debug, Clone)]
pub struct RadiationZone {
    pub section: u8,
    /// 현재 선량율 (Gy/h)
    pub dose_rate: f32,
    /// 누적 선량 (Gy)
    pub cumulative_dose: f32,
}

/// 방사선 위험 등급
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RadHazard {
    Safe,        // < 0.1 Gy/h
    Elevated,    // 0.1 ~ 1.0
    Dangerous,   // 1.0 ~ 10.0
    Lethal,      // > 10.0
}

impl RadHazard {
    pub fn from_dose_rate(dr: f32) -> Self {
        if dr < 0.1 { Self::Safe }
        else if dr < 1.0 { Self::Elevated }
        else if dr < 10.0 { Self::Dangerous }
        else { Self::Lethal }
    }
    pub fn color(&self) -> Color {
        match self {
            Self::Safe      => Color::srgb(0.0, 1.0, 0.0),
            Self::Elevated  => Color::srgb(1.0, 1.0, 0.0),
            Self::Dangerous => Color::srgb(1.0, 0.27, 0.0),
            Self::Lethal    => Color::srgb(1.0, 0.0, 0.0),
        }
    }
    pub fn label(&self) -> &str {
        match self {
            Self::Safe      => "안전",
            Self::Elevated  => "주의",
            Self::Dangerous => "위험",
            Self::Lethal    => "치사",
        }
    }
}

pub struct RadiationPlugin;

impl Plugin for RadiationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_radiation_zones)
            .add_systems(Update, update_radiation_zones);
    }
}

fn spawn_radiation_zones(mut commands: Commands) {
    for sec in 1u8..=10 {
        commands.spawn(RadiationZone {
            section: sec,
            dose_rate: 0.0,
            cumulative_dose: 0.0,
        });
    }
}

fn update_radiation_zones(
    mut zones: Query<&mut RadiationZone>,
    state: Res<BlastState>,
    time: Res<Time>,
) {
    if !state.detonated { return; }
    let dt = time.delta_seconds();
    for mut zone in zones.iter_mut() {
        let dr = state.section_dose_rate[(zone.section - 1) as usize];
        zone.dose_rate = dr;
        zone.cumulative_dose += dr * dt / 3600.0; // Gy/h → Gy/s
    }
}
