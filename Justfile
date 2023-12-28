emulator:
   ANDROID_HOME="" ANDROID_SDK_ROOT="$HOME/Android/Sdk" ~/Android/Sdk/emulator/emulator @Pixel_7_API_34 \
       # -gpu guest \
       -no-snapshot \
       -wipe-data

install-apk:
    adb install -t android/app/build/intermediates/apk/debug/app-debug.apk

generate-bindings:
    cargo build --release
    ./target/release/uniffi-bindgen generate --library --language kotlin -o out ./target/release/libcalc2.so
