emulator:
   ANDROID_HOME="" ANDROID_SDK_ROOT="$HOME/Android/Sdk" ~/Android/Sdk/emulator/emulator @Pixel_7_API_34 \
       # -gpu guest \
       -no-snapshot \
       -wipe-data

install-apk:
    adb install -t android/app/build/intermediates/apk/debug/app-debug.apk
