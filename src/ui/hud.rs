// src/ui/hud.rs
// 실시간 HUD: 과압·방사선 선량율·충격파 반경·경보 상태

use bevy::prelude::*;
use crate::physics::blast::BlastState;
use crate::physics::radiation::{RadiationZone, RadHazard};
use crate::world::tunnel::section_start;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_hud)
            .add_systems(Update, update_hud);
    }
}

// ── 마커 컴포넌트 ─────────────────────────────────────────────
#[derive(Component)] struct HudShockRadius;
#[derive(Component)] struct HudBlastTimer;
#[derive(Component)] struct HudSectionTable;
#[derive(Component)] struct HudAlertMessage;

fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/NanumGothic.ttf");

    // 배경 패널 (좌상단)
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Px(12.0),
            top: Val::Px(12.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        background_color: Color::srgba(0.0, 0.0, 0.0, 0.70).into(),
        border_radius: BorderRadius::all(Val::Px(6.0)),
        ..default()
    }).with_children(|parent| {
        // 제목
        parent.spawn(TextBundle::from_section(
            "● 핵실험 실시간 모니터",
            TextStyle {
                font: font.clone(),
                font_size: 16.0,
                color: Color::srgb(0.0, 1.0, 0.4),
            },
        ));

        // 충격파 반경
        parent.spawn((
            TextBundle::from_section(
                "충격파 반경: --- m",
                TextStyle { font: font.clone(), font_size: 13.0, color: Color::srgb(1.0, 0.5, 0.0) },
            ),
            HudShockRadius,
        ));

        // 타이머
        parent.spawn((
            TextBundle::from_section(
                "경과: 0.0 s",
                TextStyle { font: font.clone(), font_size: 13.0, color: Color::srgb(1.0, 1.0, 1.0) },
            ),
            HudBlastTimer,
        ));

        // 구간 테이블
        parent.spawn((
            TextBundle::from_section(
                "구간별 데이터 로딩 중...",
                TextStyle { font: font.clone(), font_size: 11.0, color: Color::srgb(0.5, 0.5, 0.5) },
            ),
            HudSectionTable,
        ));
    });

    // 경보 메시지 (화면 상단 중앙)
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Percent(30.0),
            top: Val::Px(16.0),
            width: Val::Percent(40.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    }).with_children(|p| {
        p.spawn((
            TextBundle::from_section(
                "",
                TextStyle { font: font.clone(), font_size: 22.0, color: Color::srgba(0.0, 0.0, 0.0, 0.0) },
            ),
            HudAlertMessage,
        ));
    });

    // 조작 가이드 (우하단)
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            right: Val::Px(12.0),
            bottom: Val::Px(12.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        background_color: Color::srgba(0.0, 0.0, 0.0, 0.55).into(),
        border_radius: BorderRadius::all(Val::Px(6.0)),
        ..default()
    }).with_children(|p| {
        for line in [
            "WASD – 이동",
            "우클릭 드래그 – 시점",
            "Q/E – 상하",
            "Space – 핵폭발 기폭",
            "R – 리셋",
        ] {
            p.spawn(TextBundle::from_section(
                line,
                TextStyle { font: font.clone(), font_size: 11.0, color: Color::srgb(0.75, 0.75, 0.75) },
            ));
        }
    });
}

fn update_hud(
    state: Res<BlastState>,
    zones: Query<&RadiationZone>,
    mut q_radius: Query<&mut Text, (With<HudShockRadius>, Without<HudBlastTimer>, Without<HudSectionTable>, Without<HudAlertMessage>)>,
    mut q_timer:  Query<&mut Text, (With<HudBlastTimer>,  Without<HudShockRadius>, Without<HudSectionTable>, Without<HudAlertMessage>)>,
    mut q_table:  Query<&mut Text, (With<HudSectionTable>, Without<HudShockRadius>, Without<HudBlastTimer>, Without<HudAlertMessage>)>,
    mut q_alert:  Query<&mut Text, (With<HudAlertMessage>, Without<HudShockRadius>, Without<HudBlastTimer>, Without<HudSectionTable>)>,
) {
    if let Ok(mut t) = q_radius.get_single_mut() {
        t.sections[0].value = format!("충격파 반경: {:.1} m", state.shock_radius);
    }
    if let Ok(mut t) = q_timer.get_single_mut() {
        t.sections[0].value = if state.detonated {
            format!("경과: {:.1} s", state.elapsed)
        } else {
            "경과: -- (기폭 전)".to_string()
        };
    }

    // 구간 테이블
    if let Ok(mut t) = q_table.get_single_mut() {
        let mut lines = vec!["구간  과압(kPa)   선량율(Gy/h)  상태".to_string()];
        for sec in 1u8..=10 {
            let idx = (sec - 1) as usize;
            let kpa = state.section_peak_kpa[idx];
            let dr  = state.section_dose_rate[idx];
            let hz  = RadHazard::from_dose_rate(dr);
            lines.push(format!(" {:>2}   {:>10.1}  {:>12.2e}  {}", sec, kpa, dr, hz.label()));
        }
        t.sections[0].value = lines.join("\n");
    }

    // 경보
    if let Ok(mut t) = q_alert.get_single_mut() {
        let max_dr = state.section_dose_rate.iter().cloned().fold(0.0_f32, f32::max);
        let hz = RadHazard::from_dose_rate(max_dr);
        match hz {
            RadHazard::Safe => {
                t.sections[0].value = "".to_string();
                t.sections[0].style.color = Color::srgba(0.0, 0.0, 0.0, 0.0);
            }
            other => {
                t.sections[0].value = format!("⚠ 방사선 경보: {}", other.label());
                t.sections[0].style.color = other.color();
            }
        }
    }
}
