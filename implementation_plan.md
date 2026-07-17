# Bevy 3D 핵실험 시뮬레이션 렌더링 및 폰트 오류 해결 / 경고 제거 계획

## 문제 원인 분석 및 해결 방안
1. **마젠타 화면 출력**: `Cargo.toml`에서 `default-features = false` 설정 후 `tonemapping_luts` 기능이 누락되었습니다. `Cargo.toml`에 `tonemapping_luts` 기능을 추가하여 해결합니다.
2. **텍스트 두부문자(Font Tofu)**: `bevy_sprite` 기능 누락 및 예외 방지 폰트 처리를 위한 `default_font` 부재가 원인입니다. `Cargo.toml`에 `bevy_sprite`와 `default_font` 기능을 보강합니다.
3. **데드 코드 및 미사용 경고(Unused Warnings)**:
   - `blast.rs`, `hud.rs` 등에서 사용되지 않는 import 및 Query 변수를 제거합니다.
   - `tunnel.rs`, `main.rs`, `fireball.rs` 등에서 구조 정의에는 존재하지만 현재 참조되지 않아 빌드 경고를 유발하는 필드나 구조체에 `#[allow(dead_code)]`를 적용하여 경고를 깨끗하게 지우고 코드 안정성을 높입니다.

## Proposed Changes

### [Cargo.toml](file:///home/rheehose/nukesimul/Cargo.toml)
- [MODIFY] `bevy` 의존성 기능 목록에 `"bevy_sprite"`, `"tonemapping_luts"`, `"default_font"` 추가.

### [src/main.rs](file:///home/rheehose/nukesimul/src/main.rs)
- [MODIFY] `enum AppState`에 `#[allow(dead_code)]` 부여.

### [src/physics/blast.rs](file:///home/rheehose/nukesimul/src/physics/blast.rs)
- [MODIFY] 미사용 임포트 `TUNNEL_RADIUS` 제거.

### [src/ui/hud.rs](file:///home/rheehose/nukesimul/src/ui/hud.rs)
- [MODIFY] 미사용 임포트 `section_start` 및 `update_hud` 시스템의 미사용 Query `zones` 제거.

### [src/explosion/fireball.rs](file:///home/rheehose/nukesimul/src/explosion/fireball.rs)
- [DELETE] 미사용 컴포넌트 `MushroomStem` 제거.

### [src/explosion/shockwave.rs](file:///home/rheehose/nukesimul/src/explosion/shockwave.rs)
- [MODIFY] `pub struct ShockwaveRing`을 필드가 없는 unit struct 형태로 변경.

### [src/world/tunnel.rs](file:///home/rheehose/nukesimul/src/world/tunnel.rs)
- [MODIFY] `TunnelSection`, `BlastBarrier`, `BlastDoor`, `MeasurementEquipment` 구조체에 `#[allow(dead_code)]` 부여 및 미사용 상수 `WALL_THICKNESS` 제거.

---

## 검증 계획

### 1단계: 빌드 검증
- `cargo check --bin nukesimul` 명령어를 실행하여 에러와 경고 메시지가 모두 해소되었는지 검증합니다.

### 2단계: 실체 실행 검증 (사용자 수동 테스트 권장)
- `cargo run`을 실행하여 3D 터널 그래픽 및 HUD 텍스트가 정상 색상과 한글로 선명하게 렌더링되는지 확인합니다.
