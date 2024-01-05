use std::env;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target_arch == "x86_64" && target_os == "android" {
        let android_home = env::var("ANDROID_HOME").expect("ANDROID_HOME not set");
        const ANDROID_NDK_VERSION: &str = "26.1.10909125";
        const LINUX_X86_64_LIB_DIR: &str =
            "toolchains/llvm/prebuilt/linux-x86_64/lib/clang/17/lib/linux/";
        println!("cargo:rustc-link-search={android_home}/ndk/{ANDROID_NDK_VERSION}/{LINUX_X86_64_LIB_DIR}");
        println!("cargo:rustc-link-lib=static=clang_rt.builtins-x86_64-android");
    }
}
