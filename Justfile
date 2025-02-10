transportID := `adb devices -l | grep 192.168 | sed -n 's/.*transport_id:\([0-9]\+\).*/\1/p'`

build:
    pnpm tauri android build -t aarch64 --apk

install:
    adb -t {{transportID}} install -r src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk

dev:
    pnpm tauri android dev
