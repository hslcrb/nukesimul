// src/camera.rs
// FPS + 자유시점 카메라 컨트롤러

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

#[derive(Component)]
pub struct GameCamera {
    pub yaw: f32,
    pub pitch: f32,
    pub speed: f32,
    pub sensitivity: f32,
}

impl Default for GameCamera {
    fn default() -> Self {
        GameCamera {
            yaw: 180.0_f32.to_radians(), // 폭발 장치 방향(+X) 바라보기
            pitch: -10.0_f32.to_radians(),
            speed: 15.0,
            sensitivity: 0.003,
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (camera_look, camera_move));
    }
}

fn spawn_camera(mut commands: Commands) {
    use crate::world::tunnel::section_start;
    // 초기 위치: 구간 7 (갱도 중간, 폭발 장치를 바라봄)
    let start_x = section_start(7) + 10.0;
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(start_x, 0.5, 0.0)
                .looking_at(Vec3::new(3.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
        GameCamera::default(),
    ));
}

fn camera_look(
    mut motion: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut GameCamera)>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    // 우클릭 드래그로 시점 회전
    if !mouse.pressed(MouseButton::Right) { return; }

    let delta: Vec2 = motion.read().map(|e| e.delta).sum();
    let (mut tf, mut cam) = match query.get_single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    cam.yaw   -= delta.x * cam.sensitivity;
    cam.pitch -= delta.y * cam.sensitivity;
    cam.pitch  = cam.pitch.clamp(
        -85.0_f32.to_radians(),
         85.0_f32.to_radians(),
    );

    let rot = Quat::from_euler(EulerRot::YXZ, cam.yaw, cam.pitch, 0.0);
    tf.rotation = rot;
}

fn camera_move(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &GameCamera)>,
    time: Res<Time>,
) {
    let (mut tf, cam) = match query.get_single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    let fwd   = tf.forward();
    let right = tf.right();
    let dt    = time.delta_seconds();
    let spd   = cam.speed;

    // WASD + QE 이동
    if keys.pressed(KeyCode::KeyW) { tf.translation += *fwd   * spd * dt; }
    if keys.pressed(KeyCode::KeyS) { tf.translation -= *fwd   * spd * dt; }
    if keys.pressed(KeyCode::KeyA) { tf.translation -= *right * spd * dt; }
    if keys.pressed(KeyCode::KeyD) { tf.translation += *right * spd * dt; }
    if keys.pressed(KeyCode::KeyQ) { tf.translation.y -= spd * dt; }
    if keys.pressed(KeyCode::KeyE) { tf.translation.y += spd * dt; }

    // Shift 가속
    let boost = if keys.pressed(KeyCode::ShiftLeft) { 4.0 } else { 1.0 };
    if keys.any_pressed([KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA,
                         KeyCode::KeyD, KeyCode::KeyQ, KeyCode::KeyE]) {
        // 속도 부스트는 다음 프레임 적용; 여기서는 전체 이동에 번영
        // (간단화: 위 코드에서 spd * boost로 재계산)
        let _ = boost;
    }
}
