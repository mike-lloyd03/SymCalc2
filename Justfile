transport_id := `adb devices -l | grep 192.168 | sed -n 's/.*transport_id:\([0-9]\+\).*/\1/p'`
avd_name := "Pixel_8_API_35"

build:
    pnpm tauri android build -t aarch64 --apk

install:
    adb -t {{transport_id}} install -r src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk

dev:
    pnpm tauri android dev

start-emulator:
    $ANDROID_HOME/emulator/emulator -avd {{avd_name}}
