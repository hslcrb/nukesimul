# Bevy 3D 핵실험 시뮬레이션 렌더링 및 폰트 오류 해결 계획

## 문제 원인 분석
1. **마젠타 화면 출력**: `Cargo.toml`에서 `default-features = false` 설정 후 `tonemapping_luts` 기능이 누락되어 기본 톤매핑(TonyMcMapface)에 필요한 Lookup Table 텍스처를 로드하지 못해 모든 3D PBR Material이 마젠타색으로 렌더링되고 있습니다.
2. **텍스트 두부문자(Font Tofu)**: 2D/UI 및 텍스트 아틀라스 렌더링에 필요한 `bevy_sprite` 기능 누락 및 예외 방지 폰트 처리를 위한 `default_font` 부재로 인해 한글 폰트가 깨져 상자 모양(두부문자)으로 출력됩니다.

## proposed Changes

### [Cargo.toml](file:///home/rheehose/nukesimul/Cargo.toml)
- [MODIFY] `bevy` 의존성 기능 목록에 `"bevy_sprite"`, `"tonemapping_luts"`, `"default_font"` 추가.

```diff
-bevy = { version = "0.14", default-features = false, features = [
-    "bevy_asset",
-    "bevy_core_pipeline",
-    "bevy_pbr",
-    "bevy_render",
-    "bevy_scene",
-    "bevy_text",
-    "bevy_ui",
-    "bevy_winit",
-    "bevy_gizmos",
-    "bevy_state",
-    "multi_threaded",
-    "png",
-    "x11",
-    "wayland",
-    "dynamic_linking",
-] }
+bevy = { version = "0.14", default-features = false, features = [
+    "bevy_asset",
+    "bevy_core_pipeline",
+    "bevy_pbr",
+    "bevy_render",
+    "bevy_scene",
+    "bevy_text",
+    "bevy_ui",
+    "bevy_winit",
+    "bevy_gizmos",
+    "bevy_state",
+    "bevy_sprite",
+    "tonemapping_luts",
+    "default_font",
+    "multi_threaded",
+    "png",
+    "x11",
+    "wayland",
+    "dynamic_linking",
+] }
```

## 검증 계획

### 1단계: 빌드 검증
- `cargo check --bin nukesimul` 명령어를 실행하여 컴파일 및 의존성에 에러가 없는지 검증합니다.

### 2단계: 실체 실행 검증 (사용자 수동 테스트 권장)
- `cargo run`을 실행하여 3D 터널 모델(바닥, 벽 등)의 재질이 마젠타색 대신 회색/강철/암반색 등으로 정상 출력되는지 확인합니다.
- 좌측 상단 및 우측 하단 HUD 텍스트가 두부 문자로 깨지지 않고 정상 한글 및 한글 자모 자소 형태로 출력되는지 확인합니다.
